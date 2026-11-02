# Agent 20-19 — Uncredited indexed-descriptor geometry

The pinned postmarketOS Novatek DLM contains two groups of 140-byte
(`0x8c`) descriptor records: 17 records at `0x22a64` and 4 at `0x25230`.
Each group has a companion bank at file offset `+0x4000`. The first
*little-endian halfword* at relative byte `+0x48` in each record includes a
15-bit word index and an unproven high-bit selector flag. The index points
at the **end of that record array** when interpreted relative to file
`0x22000` and scaled by four:

`index = (first_record_file + count * 0x8c + bank * 0x4000 - 0x22000) / 4`.

| Record group (lower bank) | Record count | Lower 15-bit index | Upper 15-bit index | Addresses checked |
| --- | ---: | ---: | ---: | ---: |
| `0x22a64` | 17 | `0x4ec` | `0x14ec` | 34 |
| `0x25230` | 4 | `0xd18` | `0x1d18` | 8 |
| **Total** | **21** | | | **42** |

All 42 pointers satisfy the equation in the pinned image. The NDS32
consumer at upper-ILM file `0x10242–0x10252` reads the `+0x48` directory
word, masks `0x7fff`, and uses a shift of two. The relationship is a
**verified geometry observation**, not proof that the original firmware
linker used a generated symbolic pointer or that the 0x8000 selector
flag is correctly understood. Both bytes of each halfword therefore remain
**`unknown_dlm`** in the ledger. This also avoids adding overlapping
coverage to Agent 19's earlier 2,940-byte reference-assisted pointer
relocation transform.

A second check establishes that the twelve reference-observed bytes at
`[0x22710,0x2271c)` equal the twelve bytes at `[0x2df8c,0x2df98)`.
They resemble a tiny configuration signature with four nonzero 16-bit
fields and two zero words; neither its payload values nor protocol meaning
has been established from original source, so both spans remain
**external/reference-dependent**. This copy does not overlap the previously
verified 428-byte calibration alias or seven 3,136-byte bank-copy aliases.

The dedicated Rust and independently coded Python modules validate the
indexed formula, reject changed lower-15-bit pointers while ignoring the
unproved selector flag, reject truncated reference spans, and reject ledger
changes that promote either side to authored source.

```sh
# From the repository root, with the pinned image outside Git:
export NOVATEK_REFERENCE=/private/novatek_nt36523_fw.bin
cargo run --offline --locked -p firmware-tools --bin novatek-dlm -- ledger > /tmp/agent19-dlm-ledger.json
cargo run --offline --locked -p firmware-tools --bin novatek-dlm -- verify-descriptor-geometry "$NOVATEK_REFERENCE"
python3 tools/verify-novatek-dlm-descriptor-geometry.py "$NOVATEK_REFERENCE" --ledger /tmp/agent19-dlm-ledger.json
```

**Provenance:** zero independent source bytes added. Source-generated
DLM data remains 9,353 bytes; 40,199 DLM bytes are unresolved. These
checks only document the behavior of externally provided DLM bytes.
