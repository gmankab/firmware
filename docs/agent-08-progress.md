# Agent 08 — Cirrus CS35L41 WMFW reconstruction

**Branch:** `agent/08-cirrus-wmfw`. The user's prescribed starting point is checkpoint 50 (`b7ed4b5`), but that bundle was **not present in the mounted files** for this agent session. The supplied `recompile-firmware.bundle` instead ends at `69ebf0d1c50859189a9196a21155c388ac914d7b`; this independent WMFW branch starts there. The branch must be cherry-picked onto checkpoint 50 after the checkpoint-50 bundle is provided. No Venus source files are changed.

## Reference

- `cs35l41-dsp1-spk-prot.wmfw` is 34,236 bytes, SHA-256 `d1bea27e41001024eb3878276292b5bcc0cda4484ffced0880e12c03ec9bac2c` (matches `reference/xiaomi-nabu/firmware.sha256`).
- Header `0x0000–0x0028`: WMFW v3, HALO core 4, source version `0x0300`; timestamp, four DSP memory-size words and footer are modeled independently.
- Eleven WMFW sections have 8-byte descriptors; packed memory kinds are `0x10` PM, `0x11` XM, `0x12` YM; `0xf2` is algorithm metadata and `0xff` is informational text.
- HALO PM payload `0x278c–0x76c4` is 20,280 bytes (4,056 packed 5-byte program words); *this is not evidence of an instruction encoding*, and no HALO program has been authored yet.
- Public format reference: Linux `include/linux/firmware/cirrus/wmfw.h` and `drivers/firmware/cirrus/cs_dsp.c`.

## Checkpoint 08-01 — checked layout

New `wmfw_plan` builds the exact 40-byte header and eleven 8-byte region descriptors with a strict `ImagePlan`, rejects overlaps/truncation and checks structural region boundaries and packed word counts. **128 independent bytes of 34,236**; the eleven payloads remain explicitly unimplemented. It cannot finalize a WMFW firmware using implicit zero-fill or reference payload copying. Command-line coverage and strict metadata checking are included.

### Verification

```sh
cargo test --workspace --offline --locked
cargo build --workspace --release --offline --locked
python3 -m unittest discover -s tests -p 'test_*.py'
cargo run --offline --locked -p firmware-tools -- wmfw-coverage
cargo run --offline --locked -p firmware-tools -- verify-wmfw-layout /private/cirrus/cs35l41-dsp1-spk-prot.wmfw
```

**Not flashable / not HALO program reconstruction.** The packed DSP program and proprietary algorithm-control values remain unimplemented. A byte comparison only certifies the independently generated fields, not their runtime semantics or rights to redistribute original code.

## Checkpoint 08-02 — text region

The 48-byte `INFO_TEXT` payload at `0x30–0x60` is now produced from an
independently authored human-readable build timestamp. Cumulative independent
coverage is **176 / 34,236 bytes**, and the ten remaining opaque DSP/algorithm
payloads are deliberately absent. All reference fields compare exactly,
and the partial builder still refuses to finalize a full WMFW.

## Checkpoint 08-03 — HALO framework algorithm controls

Reconstructed the **948-byte** first `ALGORITHM_DATA` payload (`0x76cc–0x7a80`)
from high-level register offsets and twelve named firmware controls (HALO
state/heartbeat, audio block size, firmware build metadata, multirate fields,
LRCLK delay and bypass flags). The WMFW v3 encoder uses checked counted-string
fields with 4-byte padding, typed memory kinds, flags and register lengths;
it never imports the reference bytes to build output. The bounded parser
accepts the generated algorithm, rejects truncation and nonzero alignment
padding. Independent coverage is **1,124 / 34,236 bytes**, with nine region
payloads still missing.

## Checkpoint 08-04 — CSPL and event logger controls

Added 33 CSPL and 12 event-logger controls as independently authored names,
register offsets, packed-memory types, access flags and value lengths. The
WMFW v3 length-prefixed serializer produces exactly the 2,100-byte CSPL
(`0x7a88–0x82bc`) and 760-byte event metadata (`0x82c4–0x85bc`) regions.
The host-diagnostic parser validates the records and rejects malformed data.
**Cumulative independently generated metadata: 3,984 / 34,236 bytes.** Seven
DSP memory payloads are still missing, including all 20,280 program bytes.
