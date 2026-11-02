// SPDX-License-Identifier: GPL-3.0-only
// Agent 13: write paired byte-markers to a bounded wireless record table.
// File [0x42ed4,0x42f18), VMA [0xb0002ed4,0xb0002f18).
// The 2-element hardware loop and postincrement stores are exact V66 source.
.text
.globl agent13_runtime_byte_table_seed
agent13_runtime_byte_table_seed:
{ r2 = add(r2,#1); r6 = #255 }
seed_more:
{ r7 = addasl(r3,r0,#1); r8 = addasl(r2,r0,#2) }
{ loop0(write_pair,#2) }
write_pair:
{ memb(r8+#-1) = r6; memb(r8++#2) = r6 }
{ nop; memb(r7++#1) = r6 }:endloop0
{ r7 = addasl(r4,r0,#1); r1 = add(r1,#1); memb(r4+r0<<#1) = r6 }
{ r0 = and(r1,#255) }
{ memb(r7+#1) = #-1; r7 = memub(r5+#0) }:mem_noshuf
{ p0 = cmp.gtu(r7,r0); if (p0.new) jump:t seed_more }
{ jumpr r31 }
