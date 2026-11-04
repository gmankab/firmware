#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-only
# Verify independently authored Venus boot+bridge bytes, then cross-check
# the complete gap via a separate LLVM/Clang A32 assembly round-trip.
# Optional second argument is a separately emitted service-region artifact.
set -euo pipefail
if (( $# < 1 || $# > 2 )); then
  echo 'usage: venus-bridge-verify-full-gap.sh /private/venus.mbn [/tmp/emitted-service-region.bin]' >&2
  exit 2
fi
REFERENCE=$1
SERVICE=${2:-}
ROOT=$(cd "$(dirname "$0")/../.." && pwd)
cd "$ROOT"
WORK=$(mktemp -d)
trap 'rm -rf "$WORK"' EXIT
cargo run --offline --locked -q -p firmware-tools --bin venus_bridge -- verify "$REFERENCE"
cargo run --offline --locked -q -p firmware-tools --bin venus_bridge -- verify-joined "$REFERENCE"
cargo run --offline --locked -q -p firmware-tools --bin venus_bridge -- emit "$WORK/bridge.bin"
cargo run --offline --locked -q -p firmware-tools --bin venus_bridge -- emit-joined "$WORK/bootbridge.bin"

# All literal pools are explicit. Intervals are runtime addresses in ascending
# order; they are NOT A32 instructions. Never emit unknown bytes as .word.
ranges=(
  0x8614:0x8618 0x8684:0x8688 0x8834:0x883c
  0x899c:0x89bc 0x8b94:0x8bac 0x8bd8:0x8bdc
  0x8c1c:0x8c24 0x8c48:0x8c50 0x8c58:0x8c5c
  0x8c80:0x8c88 0x8d44:0x8d50 0x8e7c:0x8e88 0x8e94:0x8e98
  0x8ec0:0x8ec4 0x9110:0x9124 0x913c:0x9140
  0x9258:0x9264 0x92c8:0x92d8 0x934c:0x9358
  0x94e8:0x94f4 0x9534:0x9538 0x95e0:0x95e8
  0x96bc:0x96d0 0x9704:0x970c
  0x9734:0x9738 0x98a0:0x98bc 0x9968:0x996c
  0x9a18:0x9a1c 0x9ac8:0x9acc 0x9b44:0x9b50
  0x9bc8:0x9bd0 0x9d9c:0x9db4 0x9e20:0x9e2c
  0x9ea8:0x9eb4 0x9ec4:0x9ec8 0x9ed8:0x9edc
  0x9f9c:0x9fc8
  0x9fd8:0x9fdc 0xa01c:0xa024 0xa034:0xa038
  0xa040:0xa044 0xa098:0xa0a0 0xa0e0:0xa0e8
  0xa0f4:0xa0f8 0xa1ac:0xa1b4 0xa1c0:0xa1c4
  0xa228:0xa230 0xa27c:0xa288 0xa32c:0xa334 0xa398:0xa3a0
  0xa44c:0xa454 0xa520:0xa524 0xa5b4:0xa5bc
  0xa638:0xa640 0xa69c:0xa6a4 0xa7e8:0xa800
  0xa85c:0xa864 0xa870:0xa874 0xa888:0xa88c
  0xa89c:0xa8a0 0xa8ac:0xa8b0 0xa904:0xa908
  0xa914:0xa918 0xa990:0xa99c 0xa9d0:0xa9d4 0xaa08:0xaa0c
)
args=()
for interval in "${ranges[@]}"; do args+=(--data-range "$interval"); done
python3 tools/venus/venus-bridge-a32-roundtrip.py "$REFERENCE" \
  --file-offset 0xc490 --vma 0x8490 --length 0x257c \
  "${args[@]}" --binary-out "$WORK/llvm-rebuilt-gap.bin"
cmp "$WORK/bridge.bin" "$WORK/llvm-rebuilt-gap.bin"

python3 - "$REFERENCE" "$WORK/bootbridge.bin" "$WORK/bridge.bin" "$SERVICE" <<'PY'
from pathlib import Path
import hashlib, sys
reference, bootbridge, bridge = (Path(p).read_bytes() for p in sys.argv[1:4])
assert len(bridge)==9596 and len(bootbridge)==43532
assert bridge==bootbridge[33936:]
assert bootbridge==reference[0x4000:0x4000+43532]
print('Agent 02 verified 9,596 gap bytes; 43,532 combined boot+bridge bytes.')
print('boot+bridge SHA-256:',hashlib.sha256(bootbridge).hexdigest())
if sys.argv[4]:
    service = Path(sys.argv[4]).read_bytes()
    assert service and len(service)%4==0
    combined=bootbridge+service
    assert combined==reference[0x4000:0x4000+len(combined)]
    print('Separately supplied service fragment is adjacent and byte-identical:',
          len(service),'service bytes;',len(combined),'combined bytes.')
    print('Service source provenance and OEM authentication remain separate checks.')
PY
