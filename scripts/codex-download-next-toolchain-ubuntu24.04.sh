#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-only
# Run on an INTERNET-CONNECTED UBUNTU 24.04 x86_64 Codex environment.
# Constructs independently licensed analysis tooling. Never downloads firmware.
# Produces <=190 MiB numbered parts to work around small file attachment limits.
set -Eeuo pipefail
IFS=$'\n\t'

if [[ "$(. /etc/os-release; echo "$ID:$VERSION_ID")" != 'ubuntu:24.04' ]] || [[ "$(uname -m)" != x86_64 ]]; then
    echo 'ERROR: run on Ubuntu 24.04 x86_64, NOT the Debian analysis host.' >&2
    exit 2
fi

ROOT="${OUTPUT_ROOT:-$PWD/nabu-next-toolchain-codex-ubuntu24.04}"
ROOT="$(mkdir -p "$ROOT" && cd "$ROOT" && pwd)"
STAGE="$ROOT/stage/nabu-next-toolchain-ubuntu24.04"
SRCS="$ROOT/sources"
BUILDS="$ROOT/build"
LOGS="$STAGE/logs"
mkdir -p "$STAGE" "$SRCS" "$BUILDS" "$LOGS" "$STAGE/ubuntu-runtime/debs"
: > "$LOGS/progress.log"
mark() { printf '%s | %s\n' "$(date -u +%FT%TZ)" "$*" | tee -a "$LOGS/progress.log"; }
JOBS="${JOBS:-2}"
SU=()
if [[ "$(id -u)" -ne 0 ]]; then SU=(sudo); fi
export DEBIAN_FRONTEND=noninteractive

mark 'Installing build prerequisites on Ubuntu; these are NOT Debian packages.'
"${SU[@]}" apt-get update 2>&1 | tee "$LOGS/apt-update.log"
"${SU[@]}" apt-get install -y --no-install-recommends \
    ca-certificates curl git gpg zstd xz-utils file binutils \
    build-essential ninja-build cmake meson pkg-config \
    python3 python3-pip python3-venv \
    libpcre2-dev libssl-dev liblzma-dev zlib1g-dev \
    libc++1-18 libc++abi1-18 libunwind-18 \
    2>&1 | tee "$LOGS/apt-install.log"
# Rizin can use vendored fallbacks for these optional libraries.
for p in python3-yaml libmagic-dev liblz4-dev libzstd-dev libxxhash-dev libmspack-dev libzip-dev libzydis-dev libcapstone-dev; do
    "${SU[@]}" apt-get install -y --no-install-recommends "$p" >> "$LOGS/apt-optional.log" 2>&1 || \
        mark "Optional Ubuntu package unavailable: $p"
done

mark 'Capturing the three x86-64 shared-library runtimes missing on Debian.'
(
    cd "$STAGE/ubuntu-runtime/debs"
    apt-get download 'libc++1-18' 'libc++abi1-18' 'libunwind-18' 2>&1
) | tee "$LOGS/apt-download-runtimes.log"
mkdir -p "$STAGE/ubuntu-runtime/root"
for deb in "$STAGE"/ubuntu-runtime/debs/*.deb; do
    dpkg-deb -x "$deb" "$STAGE/ubuntu-runtime/root"
done
find "$STAGE/ubuntu-runtime/root" \( -name 'libc++.so*' -o -name 'libc++abi.so*' -o -name 'libunwind.so*' \) \
    -print > "$LOGS/runtime-files.txt"
# Flatten multiarch/LLVM paths and dereference package-local symlink chains.
mkdir -p "$STAGE/ubuntu-runtime/flat"
for soname in 'libc++.so.1' 'libc++abi.so.1' 'libunwind.so.1'; do
    found=''
    while IFS= read -r candidate; do
        if [[ -f "$candidate" && ! -L "$candidate" ]]; then
            found="$candidate"; break
        fi
    done < <(find "$STAGE/ubuntu-runtime/root" -type f -name "$soname*" | sort)
    if [[ -n "$found" ]]; then
        cp "$found" "$STAGE/ubuntu-runtime/flat/$soname"
    else
        mark "WARN: no regular package payload for $soname; verify Debian runtime manually."
    fi
done

mark 'Downloading standalone Hexagon V5 disassembler and V4-capable IDA processor source.'
# Important: the IDA processor requires a separately licensed IDA installation.
# The standalone disassembler is primarily V5, so V4 validation is required.
if git clone --depth=1 https://github.com/programa-stic/hexag00n.git "$SRCS/hexag00n" \
        > "$LOGS/hexag00n-git.log" 2>&1; then
    git -C "$SRCS/hexag00n" rev-parse HEAD > "$LOGS/hexag00n-commit.txt"
else
    mark 'WARN: hexag00n clone failed; see log.'
fi
if git clone --depth=1 https://github.com/n-o-o-n/idp_hexagon.git "$SRCS/idp_hexagon" \
        > "$LOGS/idp-hexagon-git.log" 2>&1; then
    git -C "$SRCS/idp_hexagon" rev-parse HEAD > "$LOGS/idp-hexagon-commit.txt"
else
    mark 'WARN: idp_hexagon clone failed; see log.'
fi

mark 'Downloading LLVM 6.0.1 historical Hexagon backend source (pre-V4 deprecation).'
LEGACY_URL='https://releases.llvm.org/6.0.1/llvm-6.0.1.src.tar.xz'
if curl -fL --retry 4 --retry-delay 3 "$LEGACY_URL" \
        -o "$SRCS/llvm-6.0.1.src.tar.xz" > "$LOGS/llvm6-download.log" 2>&1; then
    (cd "$SRCS" && sha256sum llvm-6.0.1.src.tar.xz) > "$LOGS/llvm6-archive-sha256.txt"
    mkdir -p "$SRCS/llvm6"
    tar -xJf "$SRCS/llvm-6.0.1.src.tar.xz" -C "$SRCS/llvm6" --strip-components=1
else
    mark 'WARN: LLVM6 source download failed; see log.'
fi

# Building old LLVM on Ubuntu 24.04 may need legacy compatibility changes.
# Always ship the original source and complete logs if the build fails.
if [[ "${BUILD_LLVM6:-1}" = 1 && -f "$SRCS/llvm6/CMakeLists.txt" ]]; then
    mark 'Attempting LLVM6 Hexagon-only llvm-objdump, llvm-mc, llvm-readobj, llvm-tblgen build.'
    if cmake -S "$SRCS/llvm6" -B "$BUILDS/llvm6" -G Ninja \
            -DCMAKE_BUILD_TYPE=Release \
            -DCMAKE_CXX_STANDARD=11 -DCMAKE_CXX_STANDARD_REQUIRED=ON \
            -DCMAKE_CXX_FLAGS='-Wno-error' \
            -DLLVM_TARGETS_TO_BUILD=Hexagon \
            -DLLVM_INCLUDE_TESTS=OFF -DLLVM_INCLUDE_EXAMPLES=OFF \
            -DLLVM_ENABLE_ZLIB=OFF -DLLVM_ENABLE_TERMINFO=OFF \
            -DLLVM_BUILD_TOOLS=ON > "$LOGS/llvm6-cmake.log" 2>&1 \
       && cmake --build "$BUILDS/llvm6" --parallel "$JOBS" \
            --target llvm-objdump llvm-mc llvm-readobj llvm-tblgen \
            > "$LOGS/llvm6-build.log" 2>&1; then
        mkdir -p "$STAGE/llvm6/bin"
        for x in llvm-objdump llvm-mc llvm-readobj llvm-tblgen; do
            cp -L "$BUILDS/llvm6/bin/$x" "$STAGE/llvm6/bin/$x"
        done
        "$STAGE/llvm6/bin/llvm-objdump" --version \
            > "$LOGS/llvm6-objdump-version.txt" 2>&1 || true
        mark 'LLVM6 tools built; check Hexagon V4 decoding on Debian.'
    else
        mark 'WARN: LLVM6 build failed; source and build logs retained.'
    fi
fi

mark 'Downloading Rizin v0.9.1 with integrated Hexagon decoder.'
if git clone --depth=1 --recurse-submodules -b v0.9.1 \
        https://github.com/rizinorg/rizin.git "$SRCS/rizin" \
        > "$LOGS/rizin-git.log" 2>&1; then
    git -C "$SRCS/rizin" rev-parse HEAD > "$LOGS/rizin-commit.txt"
else
    mark 'WARN: Rizin clone failed; see log.'
fi
if [[ "${BUILD_RIZIN:-1}" = 1 && -f "$SRCS/rizin/meson.build" ]]; then
    mark 'Building Rizin with system PCRE2 and source-provided Capstone fallback.'
    RIZIN_OPTS=(
        --prefix=/opt/nabu-rizin --libdir=lib
        -Dbuildtype=release -Dlocal=enabled -Dportable=true
        -Dinstall_sigdb=false -Denable_tests=false -Denable_rz_test=false
        -Dsubprojects_check=false
        -Duse_sys_pcre2=enabled
        -Duse_sys_capstone=disabled
        -Duse_sys_magic=auto
        -Duse_sys_zlib=auto -Duse_sys_lz4=auto -Duse_sys_libzstd=auto
        -Duse_sys_lzma=auto -Duse_sys_openssl=auto
        -Duse_sys_xxhash=auto -Duse_sys_libmspack=auto -Duse_sys_libzip=auto
    )
    if meson setup "$BUILDS/rizin" "$SRCS/rizin" "${RIZIN_OPTS[@]}" \
            --wrap-mode=default > "$LOGS/rizin-meson.log" 2>&1 \
       && meson compile -C "$BUILDS/rizin" -j "$JOBS" \
            > "$LOGS/rizin-build.log" 2>&1 \
       && DESTDIR="$STAGE/rizin-staged" meson install -C "$BUILDS/rizin" \
            > "$LOGS/rizin-install.log" 2>&1; then
        mark 'Rizin built and staged (runtime library compatibility must be checked on Debian).'
    else
        mark 'WARN: Rizin build failed; sources and logs retained.'
    fi
fi

mark 'Creating reproducible manifest and downloader instructions.'
mkdir -p "$STAGE/sources"
for repo in hexag00n idp_hexagon rizin; do
    if [[ -d "$SRCS/$repo" ]]; then
        # Preserve complete source including submodules without Git pack history.
        tar -C "$SRCS" --exclude='.git' -cf - "$repo" | tar -C "$STAGE/sources" -xf -
    fi
done
if [[ -f "$SRCS/llvm-6.0.1.src.tar.xz" ]]; then
    cp "$SRCS/llvm-6.0.1.src.tar.xz" "$STAGE/sources/"
fi
cat > "$STAGE/README.txt" <<'EOF'
Xiaomi Nabu additional toolchain - RESEARCH ONLY
Built on Ubuntu 24.04 x86_64 for consumption on Debian 13.

Folders:
  llvm6/bin       Historical LLVM6 Hexagon disassembler/assembler CLI (IF build succeeded).
  rizin-staged    Rizin binaries and libraries (IF build succeeded).
  ubuntu-runtime  Ubuntu .debs and extracted x86_64 libc++/ABI/unwind libraries.
  sources         Upstream-source snapshots and LLVM6 source archive; licenses apply individually.
  logs            Every attempted build/download plus version/commit information.

Example Debian setup for supplied LLVM22 binaries (from PREVIOUS upload):
  export LD_LIBRARY_PATH="$PWD/ubuntu-runtime/flat:${LD_LIBRARY_PATH:-}"
  /path/to/llvm22/bin/llvm-objdump --version

WARNING: LLVM6, llvm22 and Rizin may still fail due to ABI or ISA mismatches.
WARNING: IDA plugin needs a separate, legally acquired IDA installation.
WARNING: legacy LLVM6 is a V4 candidate; do not claim decoding until tested.
WARNING: none of these builds, nor recovered C-like pseudocode, is Nabu firmware.
WARNING: do not redistribute original Qualcomm, Cirrus, or Novatek binaries as GPLv3.
EOF
(
    cd "$STAGE"
    find . -type f ! -name MANIFEST.sha256 -print0 | sort -z | xargs -0 -r sha256sum > MANIFEST.sha256
)
(
    cd "$ROOT/stage"
    ARCHIVE="$ROOT/nabu-next-toolchain-ubuntu24.04.tar.zst"
    # zstd -19 may be CPU-intensive; override ZSTD_LEVEL if necessary.
    tar -cf - nabu-next-toolchain-ubuntu24.04 | zstd -q -T"$JOBS" -"${ZSTD_LEVEL:-12}" -o "$ARCHIVE"
    sha256sum "$ARCHIVE" > "$ARCHIVE.sha256"
    split -b 190M -d -a 3 "$ARCHIVE" "$ARCHIVE.part"
    # Renumber into part001, part002, ...
    i=1
    for part in "$ARCHIVE".part[0-9][0-9][0-9]; do
        printf -v number '%03d' "$i"
        mv "$part" "$ARCHIVE.part$number.new"
        i=$((i + 1))
    done
    for part in "$ARCHIVE".part[0-9][0-9][0-9].new; do mv "$part" "${part%.new}"; done
    sha256sum "$ARCHIVE".part[0-9][0-9][0-9] > "$ROOT/PARTS.sha256"
)
mark 'DONE: upload numbered *.part001, *.part002 ... IN ORDER, one per message.'
printf '\nResult paths:\n'
ls -lh "$ROOT"/nabu-next-toolchain-ubuntu24.04.tar.zst.part[0-9][0-9][0-9]
printf 'Checksums: %s\n' "$ROOT/PARTS.sha256"
