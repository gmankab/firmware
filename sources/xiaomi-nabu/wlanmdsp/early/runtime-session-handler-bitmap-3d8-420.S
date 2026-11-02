// SPDX-License-Identifier: GPL-3.0-only
// Source-defined WLAN session handler Hexagon V65-compatible V66 subset, file 0x47990..0x479d8.
.text
.globl agent13_session_handler_bitmap_3d8_420
agent13_session_handler_bitmap_3d8_420:

{
 r2 = memw(r29+#12)
}

{
 p0 = r2
 if (!p0.new) r2 = add(r16,#-1)
 if (p0.new) jump:t 0x14
}

{
 r2 = lsl(#1,r2)
 jump 0x28
}

{
 r2 = memub(r17+#1)
}

{
 if (p0.new) r2 = #0
 p0 = cmp.eq(r2,#0); if (p0.new) jump:nt 0x1c
}

{
 loop0(L_bitmap_loop,r2)
 r3:2 = combine(#0,#0)
}

L_bitmap_loop:
{
 r2 = and(r2,#255)
}

{
 r2 = setbit(r2,r3)
 r3 = add(r3,#1)
}:endloop0

{
 r1 = and(r18,#255)
 r2 = and(r2,#255)
 r3 = r21
 r0 = memw(r17+#916)
}
