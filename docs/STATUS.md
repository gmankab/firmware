# Current firmware reconstruction status

Generated from `reference/xiaomi-nabu/coverage.toml` by `python3 tools/firmware.py status`.

**Caution:** counts include reported source encoding, metadata, literal tables and some modeled layout bytes. They are *not* proof of high-level source recovery, device behavior or newly valid OEM signatures.

| Firmware | Original bytes | Reported generated/modelled | Not credited |
| --- | ---: | ---: | ---: |
| `modemuw.jsn` | 713 | 713 | 0 |
| `firmware-5.bin` | 60 | 60 | 0 |
| `a640_zap.mbn` | 14,256 | 7,856 | 6,400 |
| `venus.mbn` | 1,159,200 | 637,604 | 521,596 |
| `adsp.mbn` | 14,430,684 | 31,608 | 14,399,076 |
| `cdsp.mbn` | 9,455,252 | 11,964 | 9,443,288 |
| `modem.mbn` | 5,101,612 | 19,528 | 5,082,084 |
| `wlanmdsp.mbn` | 4,331,136 | 52,836 | 4,278,300 |
| `cs35l41-dsp1-spk-prot.wmfw` | 34,236 | 10,299 | 23,937 |
| `novatek_nt36523_fw.bin` | 196,608 | 149,097 | 47,511 |
| `BL-cs35l41-dsp1-spk-prot.bin` | 5,340 | 5,340 | 0 |
| `BR-cs35l41-dsp1-spk-prot.bin` | 5,340 | 5,340 | 0 |
| `TL-cs35l41-dsp1-spk-prot.bin` | 5,340 | 5,340 | 0 |
| `TR-cs35l41-dsp1-spk-prot.bin` | 5,340 | 5,340 | 0 |
| `a630_sqe.fw` | 32,304 | 0 | 32,304 |
| `a640_gmu.bin` | 37,680 | 2,014 | 35,666 |
| `board-2.bin` | 461,060 | 1,508 | 459,552 |
| `crbtfw32.tlv` | 127,544 | 28 | 127,516 |
| `crnv32.bin` | 5,504 | 744 | 4,760 |
| **Total** | **35,409,209** | **947,219** | **34,461,990** |

## Venus first executable segment

Reported source/layout: **637,604 / 1,059,316 bytes**; uncovered: **421,712 bytes**.

| Uncovered ARM runtime addresses | Bytes |
| --- | ---: |
| `0x7d4f8–0xa8000` | 174,856 |
| `0xc65ac–0x1029f4` | 246,856 |

## Important limits

- `modemuw.jsn` and the metadata-only `firmware-5.bin` are independently complete; the latter has no WLAN executable.
- The four 5,340-byte Cirrus WMDR coefficient configuration files are byte-identical from numeric tables, not HALO executable code.
- `board-2.bin` has 1,508 typed TLV/name framing bytes; 459,552 OEM calibration bytes remain external.
- Qualcomm Bluetooth TLV patch/NVM metadata accounts for 28 + 744 bytes; the patch executable and 61 NVM tag values are not reconstructed.
- GMU has 2,014 modeled load-header, reset, vector, startup and context-switch bytes; 35,666 payload bytes remain external.
- A640 exact whole-file reproduction requires externally supplied original OEM authentication bytes.
- Cirrus HALO program bytes are still not independently decompiled; reference-assisted repacking is not firmware reconstruction.
- Several reference-dependent verification tests are skipped without supplied original firmwares and ISA toolchains.
- Novatek's pinned postmarketOS revision differs from the same-name older map220v image; verify with --override.
- The manifest includes all 19 source files, including nine presently unreconstructed binaries.
- The historical checkpoint-50 service source is absent from this recovered Git tree.
- No complete newly signed firmware build or hardware boot verification is claimed.
