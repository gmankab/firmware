#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-only
# Rebuild only the INDEPENDENT metadata, then explicitly provide opaque HALO
# DSP program/data as private external source inputs. Never commit firmware.
set -euo pipefail
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 /private/cirrus/cs35l41-dsp1-spk-prot.wmfw" >&2
    exit 2
fi
ref="$1"
repo="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo"
if [ ! -f "$ref" ]; then
    echo "WMFW reference missing: $ref" >&2
    exit 1
fi
ref="$(realpath "$ref")"
work="$(mktemp -d "${TMPDIR:-/tmp}/agent08-wmfw-audit.XXXXXXXX")"
cleanup() { rm -rf -- "$work"; }
trap cleanup EXIT

cargo test --workspace --offline --locked
python3 -m unittest discover -s tests -p 'test_*.py'
cargo build --workspace --release --offline --locked
cargo run --offline --locked -q -p firmware-tools -- verify-wmfw-layout "$ref"
cargo run --offline --locked -q -p firmware-tools -- wmfw-coverage
cargo run --offline --locked -q -p firmware-tools -- wmfw-memory-map
cargo run --offline --locked -q -p firmware-tools -- wmfw-cs35l41-bus-map
cargo run --offline --locked -q -p firmware-tools -- simulate-halo-wmfw-load "$ref"
cargo run --offline --locked -q -p firmware-tools -- wmfw-byte-provenance > "$work/wmfw-provenance.json"
python3 - "$work/wmfw-provenance.json" <<'PYPROVENANCE'
import json,sys
r=json.load(open(sys.argv[1],encoding='utf-8'))
assert r['total_bytes']==34236
assert r['source_authored_metadata_bytes']==3984
assert r['reference_observed_zero_bytes']==5748
assert r['external_opaque_dsp_bytes']==24504
assert r['independently_decompiled_halo_instructions']==0
spans=r['spans']
assert spans[0]['start']==0 and spans[-1]['end']==34236
assert all(a['end']==b['start'] for a,b in zip(spans,spans[1:]))
assert sum(x['end']-x['start'] for x in spans)==34236
print('WMFW provenance ledger verified: 34236 categorized bytes, 0 HALO instructions decompiled')
PYPROVENANCE
cargo run --offline --locked -q -p firmware-tools -- wmfw-control-ranges
cargo run --offline --locked -q -p firmware-tools -- inspect-wmfw-program-patterns "$ref"

# Synthetic *runtime register* fixture verifies HALO v3 algorithm-directory
# parsing without guessing a live device's algorithm bases from packed WMFW.
python3 - "$work/synthetic-halo-unpacked-registry.bin" <<'PYHALO'
import struct, sys
# The firmware's own region is in the header, not a duplicate table entry.
# These disjoint addresses are synthetic and are not measurements of Nabu.
head=[4,1,1,0x400a4,0x010203,0x20,0x20,0,0,2]
algs=[(0xcd,0x010203,0x100,0x100,0x100,0x100),
      (0xf20b,0x010203,0x300,0x100,0x300,0)]
with open(sys.argv[1],'wb') as f:
    f.write(struct.pack('!10I', *head))
    for alg in algs: f.write(struct.pack('!6I', *alg))
    f.write(struct.pack('!I', 0x00bedead))
PYHALO
cargo run --offline --locked -q -p firmware-tools -- \
  inspect-halo-runtime-registry "$work/synthetic-halo-unpacked-registry.bin" \
  > "$work/synthetic-halo-resolved-controls.txt"
grep -q 'Resolved 57 named HALO controls' "$work/synthetic-halo-resolved-controls.txt"
grep -q 'AHB 0x03400400..0x03400590' "$work/synthetic-halo-resolved-controls.txt"

# Explicit original DSP payloads are temporarily isolated outside Git.
cargo run --offline --locked -q -p firmware-tools -- extract-wmfw-external-payloads "$ref" "$work/original-dsp-payloads"
cargo run --offline --locked -q -p firmware-tools -- assemble-wmfw-with-external-payloads "$work/original-dsp-payloads" "$work/repacked.wmfw"
cmp "$ref" "$work/repacked.wmfw"
sha256sum "$work/repacked.wmfw"

# Alternative source footprint: packed Y-memory bytes 669..6114 contain a
# reference-confirmed 5445-byte zero range. No zero bytes are extracted, and the
# remaining 24807 proprietary bytes are still hash-validated external inputs.
cargo run --offline --locked -q -p firmware-tools -- extract-wmfw-sparse-y-inputs "$ref" "$work/sparse-y-external"
cargo run --offline --locked -q -p firmware-tools -- assemble-wmfw-with-sparse-y "$work/sparse-y-external" "$work/repacked-sparse.wmfw"
cmp "$ref" "$work/repacked-sparse.wmfw"
sha256sum "$work/repacked-sparse.wmfw"
[ "$(find "$work/sparse-y-external" -type f | wc -l)" -eq 8 ]
[ "$(wc -c < "$work/repacked-sparse.wmfw")" -eq 34236 ]

# More compact Y representation, with two reference-confirmed byte intervals:
# [45,348) and [669,6114) relative to the packed Y payload, not DSP word indices.
cargo run --offline --locked -q -p firmware-tools -- extract-wmfw-compact-y-inputs "$ref" "$work/compact-y-external"
cargo run --offline --locked -q -p firmware-tools -- assemble-wmfw-with-compact-y "$work/compact-y-external" "$work/repacked-compact-y.wmfw"
cmp "$ref" "$work/repacked-compact-y.wmfw"
sha256sum "$work/repacked-compact-y.wmfw"
[ "$(find "$work/compact-y-external" -type f | wc -l)" -eq 9 ]
[ "$(find "$work/compact-y-external" -type f -printf '%s\n' | awk '{s+=$1} END {print s}')" -eq 24504 ]

# Reject a compact Y-memory input corrupted after extraction.
printf '\001' | dd of="$work/compact-y-external/region-0f90-middle.bin" bs=1 seek=0 conv=notrunc status=none
if cargo run --offline --locked -q -p firmware-tools -- assemble-wmfw-with-compact-y "$work/compact-y-external" "$work/should-fail-compact.wmfw" >/dev/null 2>&1; then
    echo 'ERROR: altered compact-Y external payload was accepted' >&2
    exit 1
fi
test ! -e "$work/should-fail-compact.wmfw"

# Reject a payload corrupted after extraction, without overwriting reference.
printf '\001' | dd of="$work/sparse-y-external/region-0f90-prefix.bin" bs=1 seek=0 conv=notrunc status=none
if cargo run --offline --locked -q -p firmware-tools -- assemble-wmfw-with-sparse-y "$work/sparse-y-external" "$work/should-fail.wmfw" >/dev/null 2>&1; then
    echo 'ERROR: altered sparse external payload was accepted' >&2
    exit 1
fi
test ! -e "$work/should-fail.wmfw"
git diff --check
printf '\nAgent08 exact reference REPACK verified. Independent 3984 metadata bytes; 30252 externally supplied proprietary DSP bytes.\n'
printf 'Sparse repack: 5445 reference-verified zero bytes emitted, 24807 proprietary external bytes required.\n'
printf 'Compact repack: 5748 reference-verified zero bytes emitted, 24504 proprietary external bytes required.\n'
printf 'HALO source reconstruction is not complete, regardless of byte-identical repack.\n'
