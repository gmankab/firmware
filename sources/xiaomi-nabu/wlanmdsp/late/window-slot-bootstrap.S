// SPDX-License-Identifier: GPL-3.0-only
// WLAN Agent20-14: file [0x107568,0x107598), runtime [0xb00c7568,0xb00c7598).
// Reset a descriptor pair and invoke controller slot initialization.
// Hexagon mnemonic/packet reconstruction; no OEM opcode arrays or binary includes.
.text
.globl agent14_late_window_slot_bootstrap
agent14_late_window_slot_bootstrap:
pkt_0000:
// +0x0: 3 encoded word(s)
{ r3 = #0x0; memw(r29+#-0x14) = r0; allocframe(#0x20) }
pkt_000c:
// +0xc: 2 encoded word(s)
{ r4 = add(r29,#0x4); r5 = add(r29,#0x10); memw(r29+#0x18) = r1 }
pkt_0014:
// +0x14: 3 encoded word(s)
{ r1:0 = combine(##0x3b001,#0x0); r2 = add(r29,#0x4); 	memw(r29+#0x4) = r5 }
pkt_0020:
// +0x20: 1 encoded word(s)
{ memw(r4+#0x8) = #0; 	memw(r4+#0x4) = #0 }
pkt_0024:
// +0x24: 2 encoded word(s)
{ call 7964; memw(r5+#0xc) = #-0x1 }
pkt_002c:
// +0x2c: 1 encoded word(s)
{ dealloc_return }
