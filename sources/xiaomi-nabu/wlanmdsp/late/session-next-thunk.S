// SPDX-License-Identifier: GPL-3.0-only
// WLAN V66 source reconstructed from instruction semantics and control-flow.
// File [0x106638,0x106644); runtime [0xb00c6638,0xb00c6644).
// Small session continuation thunk.
// This is instruction-level reconstruction, not evidence of on-target behavioral equivalence.
.text
.globl agent14_late_session_next_thunk
agent14_late_session_next_thunk:
pkt_0000:
// offset +0x0000, 1 encoded words
{ r2 = memw(r1+#0x0) }
pkt_0004:
// offset +0x0004, 2 encoded words
{ jump 8; r1 = memw(r2+#0x4) }
