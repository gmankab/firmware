// SPDX-License-Identifier: GPL-3.0-only
// Source-defined WLAN session handler Hexagon V65-compatible V66 subset, file 0x47a58..0x47a88.
.text
.globl agent13_session_handler_callbacks_4a0_4d0
agent13_session_handler_callbacks_4a0_4d0:

{
 call 0x1f7364
}

{
 jump 0x2c
}

{
 call 0x166080
 r1:0 = combine(#16,r19)
}

{
 jump 0x20
}

{
 call 0x169a54
 r1:0 = combine(r18,r19)
}

{
 jump 0x14
}

{
 r1 = #32 ; jump 0x8
}

{
 r1 = #33
}

{
 callr r4
 r0 = r19; 	r2 = r18
}
