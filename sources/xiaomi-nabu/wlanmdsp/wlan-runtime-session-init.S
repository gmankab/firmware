// SPDX-License-Identifier: GPL-3.0-only
// WLAN Hexagon v66 session startup and table seeding, runtime
// [0xb0002340,0xb0002494), ELF offsets [0x42340,0x42494).
// Independently re-encoded Hexagon packets with a two-iteration hardware loop.
.text
.globl wlan_runtime_session_init
wlan_runtime_session_init:
{ call 0x2a3414; allocframe(#64) }
{ r19:18 = combine(r3,r4); r20 = r2; r17:16 = combine(r0,r1); r6 = memw(gp+#0x78c) }
{ r21 = #-1; r5 = memw(r6+#0x2b0); if (cmp.eq(r5.new,#0)) jump:nt restore_exit }
{ r8 = #0; r22 = ##0xb06a2db8; r3 = #2 }
{ r2 = add(r22,#12); r4 = add(r22,#44); memw(r22+#68) = r20 }
{ memw(r2+#0) += #1 }
{ r3 = memw(r22+#0); memw(r22+#24) = r3 }
{ memb(r22+#50) = r19; memw(r22+#20) = r3 }
{ r3 = memb(r6+#1); if (cmp.eq(r3.new,#0)) jump:nt after_seed_loop }
{ r3 = #0; r7 = #255 }
start_seed:
{ loop0(seed_step,#2); r9 = and(r3,#255) }
{ r9 = add(##0xb06a2dd4,asl(r9,#2)); r13:12 = combine(#0,r9) }
{ r12 = add(##0xb06a2de0,asl(r12,#1)); nop; nop }
seed_step:
{ p0 = cmp.eq(r13,#0); r13 = add(r13,#1) }
{ r14 = mux(p0,r17,#-1); r15 = mux(p0,r16,#-1); r28 = mux(p0,r18,#-1); memb(r9+#1) = r14.new }
{ memb(r9++#2) = r15; memb(r12++#1) = r28 }:endloop0
{ r9 = addasl(r4,r8,#1); r3 = add(r3,#1); memb(r4+r8<<#1) = r7 }
{ r8 = and(r3,#255) }
{ memb(r9+#1) = #-1; r9 = memub(r6+#1) }:mem_noshuf
{ p0 = cmp.gtu(r9,r8); if (p0.new) jump:t start_seed }
after_seed_loop:
{ call 0x20324; r1:0 = combine(#1,r5) }
{ r21 = r0; r2 = memw(r22+#12); if (cmp.eq(r21.new,#0)) jump:nt no_resource }
{ r1 = #0x817; r0 = ##8; memw(r29+#24) = r20 }
{ memw(r29+#20) = r2; memw(r29+#16) = r19 }
{ memw(r29+#12) = r18; memw(r29+#8) = r17 }
{ call 0x218d8c; memw(r29+#4) = r16; memw(r29+#0) = r21 }
{ r1:0 = combine(#7,##0xb02a6178); r2 = memw(r22+#12); memw(r29+#24) = r20 }
{ memw(r29+#20) = r2; memw(r29+#16) = r19 }
{ memw(r29+#12) = r18; memw(r29+#8) = r17 }
{ call -0x134; memw(r29+#4) = r16; memw(r29+#0) = r21 }
{ jump restore_exit }
no_resource:
{ r1 = #0x816; r0 = ##0; memw(r29+#20) = r20 }
{ memw(r29+#16) = r2; memw(r29+#12) = r19 }
{ memw(r29+#8) = r18; memw(r29+#4) = r17 }
{ call 0x218d50; memw(r29+#0) = r16 }
{ r1:0 = combine(#6,##0xb02a616c); r2 = memw(r22+#12); memw(r29+#20) = r20 }
{ memw(r29+#16) = r2; memw(r29+#12) = r19 }
{ memw(r29+#8) = r18; memw(r29+#4) = r17 }
{ call -0x170; r21 = #0; memw(r29+#0) = r16 }
restore_exit:
{ r0 = r21; jump ##0x2a33a8 }
{ jump 0x1d5bf8 }
