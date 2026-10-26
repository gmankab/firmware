// SPDX-License-Identifier: GPL-3.0-only
// Reconstructed Hexagon V66 source, using mnemonic-level packet semantics.
// File [0x1071a8,0x1071e0); runtime [0xb00c71a8,0xb00c71e0).
// Clear an existing record slot and return failure status. Binary equivalence is not an on-device execution test.
.text
.globl agent14_late_window_slot_reset
agent14_late_window_slot_reset:
pkt_0000:
// offset +0x0000, 1 words
{ allocframe(#0x0) }
pkt_0004:
// offset +0x0004, 2 words
{ r2 = memw(r0+#0x8); if (cmp.eq(r2.new,#0x0)) jump:nt pkt_0028 }
pkt_000c:
// offset +0x000c, 2 words
{ r3 = memw(r0+#0x8c); if (cmp.eq(r3.new,#0x0)) jump:nt pkt_001c }
pkt_0014:
// offset +0x0014, 2 words
{ jump 36; deallocframe }
pkt_001c:
// offset +0x001c, 2 words
{ memw(r2+#0x20) = #0x0; memw(r0+#0x90) = #0x0 }
pkt_0024:
// offset +0x0024, 1 words
{ dealloc_return }
pkt_0028:
// offset +0x0028, 4 words
{ call -3128; r0 = ##-0x4fd488c6; r1 = #0x2f8 }
