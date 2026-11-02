# Current firmware reconstruction status

Generated from `reference/xiaomi-nabu/coverage.toml` by `python3 tools/firmware.py status`.

**Caution:** counts include reported source encoding, metadata, literal tables and some modeled layout bytes. They are *not* proof of high-level source recovery, device behavior or newly valid OEM signatures.

| Firmware | Original bytes | Reported generated/modelled | Not credited |
| --- | ---: | ---: | ---: |
| `modemuw.jsn` | 713 | 713 | 0 |
| `a640_zap.mbn` | 14,256 | 7,856 | 6,400 |
| `venus.mbn` | 1,159,200 | 529,056 | 630,144 |
| `adsp.mbn` | 14,430,684 | 31,608 | 14,399,076 |
| `cdsp.mbn` | 9,455,252 | 11,964 | 9,443,288 |
| `modem.mbn` | 5,101,612 | 19,528 | 5,082,084 |
| `wlanmdsp.mbn` | 4,331,136 | 52,836 | 4,278,300 |
| `cs35l41-dsp1-spk-prot.wmfw` | 34,236 | 10,488 | 23,748 |
| `novatek_nt36523_fw.bin` | 196,608 | 149,097 | 47,511 |
| **Total** | **34,723,697** | **813,146** | **33,910,551** |

## Venus first executable segment

Reported source/layout: **529,056 / 1,059,316 bytes**; uncovered: **530,260 bytes**.

| Uncovered ARM runtime addresses | Bytes |
| --- | ---: |
| `0xaa0c–0x12e5c` | 33,872 |
| `0x68000–0x69b80` | 7,040 |
| `0x69e44–0x6b860` | 6,684 |
| `0x6b990–0x710c0` | 22,320 |
| `0x73e10–0xa8000` | 213,488 |
| `0xc65ac–0x1029f4` | 246,856 |

## Important limits

- `modemuw.jsn` is the only independently complete original file in this ledger.
- A640 exact whole-file reproduction requires externally supplied original OEM authentication bytes.
- Cirrus HALO program bytes are still not independently decompiled; reference-assisted repacking is not firmware reconstruction.
- Several reference-dependent verification tests are skipped without supplied original firmwares and ISA toolchains.
- The historical checkpoint-50 service source is absent from this recovered Git tree.
- No complete newly signed firmware build or hardware boot verification is claimed.
