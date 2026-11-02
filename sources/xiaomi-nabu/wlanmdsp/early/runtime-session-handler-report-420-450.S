// SPDX-License-Identifier: GPL-3.0-only
// Source-defined WLAN session handler Hexagon V65-compatible V66 subset, file 0x479d8..0x47a08.
.text
.globl agent13_session_handler_report_420_450
agent13_session_handler_report_420_450:

{
 call -0x4bb8
 r4 = r22
}

{
 jump 0xa8
}

{
 r1 = #20 ; jump 0x9c
}

{
 r3 = r18
 r0 = r19
 r2 = memw(gp+#1932)
}

{
 r4 = memw(r2+#932)
}

{
 r4 = clrbit(r4,#0)
}

{
 r3 = or(r4,and(r3,#1))
 r2 = r18
 memw(r2+#932) = r3.new
}
