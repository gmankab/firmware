// SPDX-License-Identifier: GPL-3.0-only
// Source-defined WLAN session handler Hexagon V65-compatible V66 subset, file 0x47a2c..0x47a58.
.text
.globl agent13_session_handler_flags_474_4a0
agent13_session_handler_flags_474_4a0:

{
 r1 = #1
}

{
 immext(#2954884096)
 r4 = ##-1340083200
 jump 0x50
}

{
 r1 = #34 ; jump 0x44
}

{
 r1 = #30 ; jump 0x40
}

{
 r1 = #61 ; jump 0x3c
}

{
 r2 = #49 ; jump 0xc0
}

{
 p0 = cmp.eq(r18,#0)
 r1:0 = combine(#22,r19)
}

{
 r2 = mux(p0,#0,#1)
}
