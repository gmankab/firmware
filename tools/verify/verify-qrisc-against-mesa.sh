#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-only
# Cross-compiler check for a PUBLIC-ISA SYNTHETIC demonstration, not OEM firmware.
set -euo pipefail
if (( $# != 2 )); then
  echo "Usage: $0 /path/to/qrisc-asm /path/to/mesa/share/freedreno/registers" >&2
  exit 2
fi
ASM=$(realpath "$1")
RNN_PATH=$(realpath "$2")
export RNN_PATH
ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
D=$(mktemp -d); trap 'rm -rf "$D"' EXIT
cat > "$D/public.asm" <<'ASM_SOURCE'
[016ee001]
loop:
jump #end
mov $02, 0x2a
end:
brne $02, 0x0, #loop
ret
ASM_SOURCE
"$ASM" "$D/public.asm" "$D/mesa.fw" >/dev/null
# Mesa writes a leading extra null word. Exclude it from instruction comparisons.
tail -c +5 "$D/mesa.fw" > "$D/mesa-trimmed.bin"
(
  cd "$ROOT"
  cargo run --locked --offline --quiet -p firmware-tools --bin qrisc-public-demo -- "$D/rust.bin"
)
cmp "$D/mesa-trimmed.bin" "$D/rust.bin"
echo "PASS: Mesa and typed Rust QRISC assembler emitted the same 20 bytes for public synthetic program"
