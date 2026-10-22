#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-only
# Fetch/build Mesa's open-source A6xx QRISC + IR3 tools on Ubuntu 24.04.
# Outputs an archive with tools, sources, dependencies and diagnostic logs.
# No firmware blobs or proprietary instruction data are downloaded.
set -Eeuo pipefail
umask 022
if [[ $(uname -m) != x86_64 ]]; then echo 'Required: x86_64' >&2; exit 2; fi
. /etc/os-release
if [[ "$ID" != ubuntu || "$VERSION_ID" != 24.04 ]]; then
  echo "Required: Ubuntu 24.04; detected ${PRETTY_NAME:-unknown}" >&2
  exit 2
fi

MESA_VERSION=26.2.4
MESA_SHA256=bce5f7fbebb934373b86c999a064d52fb5065878dc57f287f95346648ec832e9
BASE="${NABU_FREEDRENO_OUT:-$PWD/nabu-freedreno-codex-output}"
STAGE="$BASE/nabu-freedreno-ubuntu24.04"
BUILD="$BASE/build"
PARTDIR="$BASE/parts"
mkdir -p "$STAGE"/{archives,bin,logs,runtime,python-wheelhouse} "$BUILD" "$PARTDIR"
LOG="$STAGE/logs/progress.log"
exec > >(tee -a "$LOG") 2>&1
printf 'Ubuntu: %s; timestamp: %s\n' "$PRETTY_NAME" "$(date -u +%FT%TZ)"
printf 'Working directory: %s\n' "$BASE"

finish() {
  code=$?
  trap - EXIT
  echo "Final shell status before packaging: $code"
  printf 'exit_code=%s\n' "$code" > "$STAGE/logs/final-status.txt"
  printf 'Mesa version: %s\nsource SHA256: %s\n' "$MESA_VERSION" "$MESA_SHA256" > "$STAGE/README.txt"
  cat >> "$STAGE/README.txt" <<'EOF'
Use bin/qrisc-disasm to decode *QRISC/SQE* instructions, bin/ir3-disasm for A6xx GPU shaders.
These are distinct from Hexagon instructions. Neither by itself identifies
which portions of a signed .mbn image are QRISC, shader code, or metadata.
Some versions of qrisc-disasm expect a raw .fw format rather than ELF/MBN.
See logs for build output and the tested command-line options.
Set LD_LIBRARY_PATH to STAGE/runtime on Debian if required.
This archive contains only Mesa upstream sources/build tools and logs,
NOT vendor firmware or rebuilt images.
EOF
  # progress.log is streamed by tee and may change during packing. Never
  # record a checksum for a log that is still being written.
  (cd "$STAGE" && find . -type f ! -name 'MANIFEST.sha256' ! -path './logs/progress.log' -print0 | sort -z | xargs -0 -r sha256sum) > "$STAGE/MANIFEST.sha256"
  echo 'Packing output to <= 190 MiB parts...'
  tar -C "$BASE" -cf - "$(basename "$STAGE")" | zstd -T0 -19 -c | split -b 190m -d -a 3 - "$PARTDIR/nabu-freedreno-ubuntu24.04.tar.zst.part"
  sha256sum "$PARTDIR"/*.part* > "$PARTDIR/PARTS.sha256"
  ls -lh "$PARTDIR"/*.part*
  echo "Generated parts in $PARTDIR ; exit status: $code"
  exit "$code"
}
trap finish EXIT

get() {
  local dest="$1" url="$2"
  if [[ -s "$dest" ]]; then return; fi
  curl --fail --location --retry 5 --retry-delay 3 --connect-timeout 30 -o "$dest.partial" "$url"
  mv "$dest.partial" "$dest"
}

# Fetch exact release with independently published SHA256 checksum.
MESA_TARBALL="$STAGE/archives/mesa-${MESA_VERSION}.tar.xz"
get "$MESA_TARBALL" "https://archive.mesa3d.org/mesa-${MESA_VERSION}.tar.xz"
printf '%s  %s\n' "$MESA_SHA256" "$MESA_TARBALL" | sha256sum -c -
mkdir -p "$BUILD/source"
tar -xJf "$MESA_TARBALL" -C "$BUILD/source"
MESASRC="$BUILD/source/mesa-${MESA_VERSION}"

# Try installation of build dependencies, but retain evidence if unprivileged.
if command -v apt-get >/dev/null 2>&1; then
  if [[ $(id -u) == 0 ]]; then APT=(apt-get)
  elif command -v sudo >/dev/null 2>&1 && sudo -n true 2>/dev/null; then APT=(sudo -n apt-get)
  else APT=(); fi
  if (( ${#APT[@]} )); then
    "${APT[@]}" update > "$STAGE/logs/apt-update.log" 2>&1 || echo 'apt update unavailable (see log)'
    "${APT[@]}" install --no-install-recommends -y \
      build-essential python3 python3-pip python3-venv \
      pkg-config flex bison git libexpat1-dev libdrm-dev libzstd-dev \
      libelf-dev libudev-dev libwayland-dev libx11-dev libxext-dev \
      libxfixes-dev libxdamage-dev libffi-dev zlib1g-dev \
      > "$STAGE/logs/apt-install.log" 2>&1 || echo 'APT dependencies incomplete; continuing'
  else
    echo 'No privileged apt. Continuing with available workspace build dependencies.'
  fi
fi

# Local Python dependencies without modifying system packages.
PYSITE="$BUILD/pysite"
mkdir -p "$PYSITE"
if python3 -m pip --version >/dev/null 2>&1; then
  python3 -m pip download -d "$STAGE/python-wheelhouse" \
     'meson==1.9.1' 'ninja==1.13.0' 'Mako==1.3.10' 'PyYAML==6.0.3' \
     'packaging>=24.0' > "$STAGE/logs/pip-download.log" 2>&1 || true
  python3 -m pip install --no-warn-script-location --target "$PYSITE" \
     'meson==1.9.1' 'ninja==1.13.0' 'Mako==1.3.10' 'PyYAML==6.0.3' \
     'packaging>=24.0' > "$STAGE/logs/pip-install.log" 2>&1 || true
else
  echo 'Python pip unavailable; attempting system meson/ninja.'
fi
export PYTHONPATH="$PYSITE${PYTHONPATH:+:$PYTHONPATH}"
if python3 -m mesonbuild.mesonmain --version >/dev/null 2>&1; then
  MESON=(python3 -m mesonbuild.mesonmain)
elif command -v meson >/dev/null 2>&1; then
  MESON=(meson)
else
  echo 'Meson missing. Source and logs will still be packaged.' >&2; exit 3
fi
if [[ -x "$PYSITE/bin/ninja" ]]; then export PATH="$PYSITE/bin:$PATH"; fi
command -v ninja || { echo 'Missing ninja'; exit 3; }
"${MESON[@]}" --version | tee "$STAGE/logs/meson-version.txt"
ninja --version | tee "$STAGE/logs/ninja-version.txt"

# Build only free-software GPU analysis tools; no OpenGL/Vulkan drivers.
BUILDROOT="$BUILD/mesa-build"
if ! "${MESON[@]}" setup "$BUILDROOT" "$MESASRC" \
    -Dbuild-tests=false -Dtools=freedreno -Dgallium-drivers=[] -Dvulkan-drivers=[] \
    -Dplatforms=[] -Dglx=disabled -Degl=disabled -Dgles1=disabled -Dgles2=disabled \
    -Dgbm=disabled -Dopengl=false -Dllvm=disabled -Dvideo-codecs=[] \
    > "$STAGE/logs/meson-setup.log" 2>&1; then
  echo 'Mesa configuration failed, keeping full log and source release.'
  exit 4
fi
ninja -C "$BUILDROOT" -t targets all | grep -E '(qrisc-(asm|disasm)|ir3-disasm)' \
  > "$STAGE/logs/mesa-targets.txt" || true

for target in \
  src/freedreno/qrisc/qrisc-disasm \
  src/freedreno/qrisc/qrisc-asm \
  src/freedreno/isa/ir3-disasm; do
  name="${target##*/}"
  if ninja -C "$BUILDROOT" "$target" > "$STAGE/logs/build-${name}.log" 2>&1; then
    cp "$BUILDROOT/$target" "$STAGE/bin/$name"
    chmod 755 "$STAGE/bin/$name"
    ("$STAGE/bin/$name" -h || :) > "$STAGE/logs/help-${name}.txt" 2>&1
    (ldd "$STAGE/bin/$name" || :) > "$STAGE/logs/ldd-${name}.txt" 2>&1
    echo "Built $name"
  else
    echo "Build FAILED for $name; log retained."
  fi
done

# Provide useful default non-distribution runtime libs for Debian portability.
# Avoid copying Ubuntu's glibc/loader over Debian's system glibc.
for tool in "$STAGE/bin"/*; do
  [[ -f "$tool" ]] || continue
  while IFS= read -r dep; do
    case "$(basename "$dep")" in
      libc.so.*|libm.so.*|libdl.so.*|libpthread.so.*|librt.so.*|ld-linux*|libresolv.so.*|libnss_*) continue;;
    esac
    [[ -f "$dep" ]] && cp -L -n "$dep" "$STAGE/runtime/" || true
  done < <(ldd "$tool" | awk '/=> \/[[:graph:]]+/ {print $3}')
done

if [[ -f "$STAGE/bin/qrisc-disasm" ]]; then
  sample="$MESASRC/src/freedreno/tests/reference/qrisc_test.fw"
  if [[ -f "$sample" ]]; then
    if "$STAGE/bin/qrisc-disasm" -u "$sample" > "$STAGE/logs/qrisc-public-smoke.asm" 2> "$STAGE/logs/qrisc-public-smoke.err"; then
      if [[ -x "$STAGE/bin/qrisc-asm" ]] &&
         "$STAGE/bin/qrisc-asm" "$STAGE/logs/qrisc-public-smoke.asm" "$STAGE/logs/qrisc-public-reassembled.fw" > "$STAGE/logs/qrisc-public-asm.log" 2>&1 &&
         cmp -s "$sample" "$STAGE/logs/qrisc-public-reassembled.fw"; then
        echo 'PASS: Mesa public QRISC sample: disassemble, reassemble, byte-exact cmp' | tee "$STAGE/logs/qrisc-public-roundtrip.txt"
      else
        echo 'FAIL: Mesa public QRISC sample roundtrip; see logs' | tee "$STAGE/logs/qrisc-public-roundtrip.txt"
      fi
    fi
  fi
fi
if [[ -f "$STAGE/bin/ir3-disasm" ]]; then
  "$STAGE/bin/ir3-disasm" -g 630 -x 42b400f820010004 \
    > "$STAGE/logs/ir3-public-smoke.txt" 2>&1 || true
fi

if [[ ! -x "$STAGE/bin/qrisc-disasm" || ! -x "$STAGE/bin/ir3-disasm" ]]; then
  echo 'Decoder build incomplete: check logs.' >&2
  exit 5
fi

echo 'All requested Mesa decoder executables built.'
