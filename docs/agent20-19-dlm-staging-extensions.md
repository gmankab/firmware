# Agent 19 — Staging-bank boundary extensions (dependent only)

The existing seven lower-to-upper DLM bank aliases reconstruct 3,136 bytes when unknown primary values are externally supplied. Two **additional** exact-copy spans sit immediately adjacent to the first 406-byte `codec-bank-staging` copy. Both supplied postmarketOS and older map220v revisions match **byte-for-byte** in each source/destination pair, even though the intervening unmatched one/four-byte boundaries remain distinct.

| Primary DLM file interval | Dependent interval | Bytes | Status |
|---|---|---:|---|
| `[0x23860,0x23878)` | `[0x27860,0x27878)` | 24 | Equal in both supplied revisions, 18 observed zeros and 6 other values; primary remains externally supplied |
| `[0x23a13,0x23a75)` | `[0x27a13,0x27a75)` | 98 | Equal in both supplied revisions, alternating nonzero field pattern; primary remains externally supplied |
| **Total** | | **122** | **Zero additional independent source credit** |

This is *not* a source-generated DLM initializer. The data content, controller operations and selector semantics remain unknown. The Rust `novatek_dlm_staging_extensions.rs` and independent Python script only copy **caller-provided** unknown primary data into the dependent bank, reject malformed input and prove that synthetic nonzero primary bytes propagate. They do not fill data from zeros or embed private OEM payload.

The sparse linker evaluates these copies after its older seven bank aliases, without overlap with existing source-authored schedule/kernel fragments or any other dependent destination. The dependent category increases from 12,984 to **13,106** bytes. External-required primary values decrease from 27,215 to **27,093** bytes across 61 seed ranges. The source-authored ledger remains exactly **9,353** DLM bytes, so **40,199 bytes still lack independently derived source**, even if 13,106 of those are reconstructible from externally supplied primary values.

The pinning manifest from checkpoint 21 was deliberately regenerated for the changed sparse plan and now contains 61 postmarketOS-specific digests. It must fail closed if a branch using the old 60-range manifest attempts to assemble the newer plan.

Independent verification with both legitimate private firmware revisions:

```sh
python3 tools/verify-novatek-dlm-staging-extensions.py \
  --postmarketos /private/postmarketos-novatek_nt36523_fw.bin \
  --map220v /private/map220v-novatek_nt36523_fw.bin

cargo run --offline --locked --bin novatek-dlm -- \
  verify-staging-extensions /private/postmarketos-novatek_nt36523_fw.bin
cargo run --offline --locked --bin novatek-dlm -- sparse-plan
cargo run --offline --locked --bin novatek-dlm -- \
  verify-sparse-rebuild /private/postmarketos-novatek_nt36523_fw.bin
```

The DLM-only output is conditionally byte-identical to the pinned original when all 27,093 external seed bytes are provided. It is not a complete, independently reconstructed or OEM-signed firmware image. The portable branch is still based on the older integration preview, not the unavailable checkpoint-50 history.
