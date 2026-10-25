// SPDX-License-Identifier: GPL-3.0-only
// WLAN Hexagon v66 runtime registration and lifecycle teardown.
// Runtime [0xb00025a0,0xb0002660), ELF [0x425a0,0x42660).
.text
.globl wlan_runtime_registration
wlan_runtime_registration:
{ r1:0 = combine(#0,##0xb06a2e00); r3 = memw(gp+#0x78c); allocframe(#8) }
{ call -0x608; r2 = #76; memd(r29+#0) = r17:16; r16 = memw(r3+#688) }:mem_noshuf
{ r2 = #4; r1:0 = combine(#4,r16); memw(##0xb06a2e04) = r2.new }
{ r4 = ##0xb06a2e00; r3:2 = combine(##0xb000260c,#1) }
{ call 0x2006c; r5 = #1; memw(##0xb06a2e08) = r5.new }
{ r16 = r0; if (!cmp.eq(r16.new,#0)) jump:t registration_done }
{ call 0x1d5a80; r1:0 = combine(#0,##0xb0002660) }
registration_done:
{ r0 = r16; r17:16 = memd(r29+#0); dealloc_return }
.globl wlan_runtime_unregister
wlan_runtime_unregister:
{ p0 = cmp.eq(r0,#0); if (p0.new) jump:nt unregister_done; memd(r29+#-16) = r17:16; allocframe(#8) }
{ r2 = memw(r0+#0); if (!cmp.eq(r2.new,#0)) jump:t unregister_done }
{ r2 = memw(r0+#4) }
{ r3 = memw(r17=##0xb06a2e10); if (!cmp.eq(r3.new,r2)) jump:nt unregister_done }
{ r2 = memw(r17+#56); if (cmp.eq(r2.new,#0)) jump:nt unregister_done }
{ r2 = #0; memw(r17+#56) = r2.new }
{ r16 = memw(r0+#12); r2 = memub(r0+#9) }
{ r2 = addasl(r16,r2,#2) }
{ call 0x1d5a40; r0 = memub(r2+#3); r1 = memub(r2+#2) }
{ call -0x51c; r0 = add(r17,#40); r2 = #16; r1 = r16 }
unregister_done:
{ r0 = #0; r17:16 = memd(r29+#0); dealloc_return }
.globl wlan_runtime_register_table
wlan_runtime_register_table:
{ r4 = memw(gp+#0x78c) }
{ r3 = memw(r4+#688); if (cmp.eq(r3.new,#0)) jump:nt table_not_ready }
{ r6 = ##0xb06a2e04; r5 = #6; r7 = #1 }
{ r2 = add(r6,#12); r8 = #0 }
{ memw(r2+#0) += #1 }
{ r5 = memw(r6+#0); memw(r6+#24) = r5 }
{ memw(r6+#68) = r7; memw(r6+#20) = r5 }
{ r5 = memb(r4+#1); if (cmp.eq(r5.new,#0)) jump:nt table_seed_done }
{ r7 = #255; r6 = add(r6,#44); r5 = #0 }
table_seed:
{ loop0(table_seed_body,#2); r9 = and(r5,#255) }
{ r9 = add(##0xb06a2e20,asl(r9,#2)); r13:12 = combine(r9,#2) }
{ r13 = add(##0xb06a2e2c,asl(r13,#1)); nop }
table_seed_body:
{ p0 = cmp.eq(r12,#1); r12 = add(r12,#-1) }
{ r14 = mux(p0,r0,#-1); r15 = mux(p0,r1,#-1); memb(r9+#1) = r14.new }
{ memb(r9++#2) = r15; memb(r13++#1) = r7 }:endloop0
{ r9 = addasl(r6,r8,#1); r5 = add(r5,#1); memb(r6+r8<<#1) = r7 }
{ r8 = and(r5,#255) }
{ memb(r9+#1) = #-1; r9 = memub(r4+#1) }:mem_noshuf
{ p0 = cmp.gtu(r9,r8); if (p0.new) jump:t table_seed }
table_seed_done:
{ r1:0 = combine(#1,r3); jump 0x20028 }
table_not_ready:
{ r0 = #-1; jumpr r31 }
