#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-only
# Agent 15: independent transport and provenance checks, NOT HALO ISA decode.
set -euo pipefail
if [ "$#" -ne 1 ]; then
  echo "usage: $0 /private/cs35l41-dsp1-spk-prot.wmfw" >&2
  exit 2
fi
ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
REF=$1
if [ ! -f "$REF" ]; then echo "missing WMFW input: $REF" >&2; exit 2; fi
TMP=$(mktemp -d)
trap 'rm -rf -- "$TMP"' EXIT
cd "$ROOT"
# Each agent must use its own target directory; callers may override.
: "${CARGO_TARGET_DIR:=$ROOT/target-agent15}"
export CARGO_TARGET_DIR
cargo test --workspace --offline --locked -q
python3 -m unittest discover -s tests -p 'test_*.py'
cargo build --workspace --release --offline --locked -q
cargo run --offline --locked -q --example agent15_pm_audit -- "$REF" --json-out "$TMP/rust-report.json" >/dev/null
python3 tools/agent15_pm_python.py "$REF" --json-out "$TMP/python-report.json" >/dev/null
cmp "$TMP/rust-report.json" "$TMP/python-report.json"
python3 - "$TMP/rust-report.json" <<'PY'
import json,sys
r=json.load(open(sys.argv[1]))
assert r['pm_payload_bytes']==20280
assert r['40bit_storage_words']==4056
assert r['20byte_transport_blocks']==1014
assert r['32bit_host_writes']==5070
assert r['transport_byte_mismatches']==0
assert r['independently_rebuilt_executable_bytes']==0
assert r['source_verified_20bit_half_slots']==0
assert r['first_uncovered_pm_word']==0
print('Agent 15 HALO PM audit: 20280 reference bytes, 4056 storage words, 5070 host writes; mismatch=0; independent HALO executable=0')
PY
git diff --check
