// SPDX-License-Identifier: GPL-3.0-only
// WLAN Hexagon v66 format-state selector, shared logger and three jump stubs.
// File [0x42208,0x42340); runtime [0xb0002208,0xb0002340).
// Written as semantic instruction packets; no OEM opcode words are embedded.
.text
.globl wlan_format_state_selector
wlan_format_state_selector:
{ call 0x2a3570; allocframe(#32) }
{ r16 = r0; if (cmp.eq(r16.new,#0)) jump:nt finish }
{ r2 = memw(r16+#0); if (cmp.eq(r2.new,#2)) jump:nt state_two }
{ p0 = cmp.eq(r2,#1); if (p0.new) jump:nt state_one }
{ p0 = cmp.eq(r2,#0); if (!p0.new) jump:t finish; if (p0.new) r2 = memw(r16+#4) }
{ r3 = memw(r18=##0xb06a2dc4); if (!cmp.eq(r3.new,r2)) jump:t bad_global }
{ r1 = #0x814; r0 = ##16; r17 = memw(r16+#12) }
{ r3 = memw(r18+#56); r16 = memub(r16+#8) }
{ r19 = addasl(r17,r16,#2); r4 = memub(r17+r16<<#2) }
{ r5 = memub(r19+#1); memw(r29+#12) = r2 }
{ memw(r29+#8) = r3; memw(r29+#4) = r5 }
{ call 0x218f5c; memw(r29+#0) = r4 }
{ r1:0 = combine(#4,##0xb02a6184); r2 = memub(r17+r16<<#2); r3 = memub(r19+#1) }
{ r4 = memw(r18+#56); memw(r29+#8) = r4.new }
{ r5 = memw(r18+#0); memw(r29+#12) = r5.new }
{ call local_trampoline_a; memw(r29+#4) = r3; memw(r29+#0) = r2 }
{ r0 = memub(r19+#1); r1 = memub(r17+r16<<#2) }
{ call 0x1d3460; r2 = memw(r18+#56) }
{ call -0x168; r0 = add(r18,#40); r2 = #16; r1 = r17 }
{ jump finish }
state_two:
{ call 0x218f0c; r0 = ##40; r1 = #0x810 }
{ r0 = ##0xb02a61a8; jump common_fail }
state_one:
{ call 0x218ef0; r0 = ##32; r1 = #0x810 }
{ r0 = ##0xb02a619c }
common_fail:
{ call local_trampoline_b }
finish:
{ r0 = #0; jump ##0x2a3574 }
bad_global:
{ r1 = #0x812; r0 = ##24; memw(r29+#4) = r2 }
{ call 0x218ec0; memw(r29+#0) = r3 }
{ r0 = ##0xb02a6190; r1 = memw(r18+#0); r2 = memw(r16+#4) }
{ call local_trampoline_c }
{ jump finish }
local_trampoline_a:
{ jump ##-0x1270 }
local_trampoline_b:
{ jump ##-0x1228 }
local_trampoline_c:
{ jump ##-0x12c0 }
// Instruction NOPs at six executable addresses, not PLT linker fill.
.rept 6
{ nop }
.endr
