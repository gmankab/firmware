// SPDX-License-Identifier: GPL-3.0-only
// Source-defined WLAN session handler Hexagon V65-compatible V66 subset, file 0x47a08..0x47a2c.
.text
.globl agent13_session_handler_cases_450_474
agent13_session_handler_cases_450_474:

{
 call 0x1fefe8
 r1 = memuh(r19+#198)
}

{
 call 0x338
 r0 = r19
}

{
 jump 0x70
}

{
 r1 = #39 ; jump 0x64
}

{
 call 0x1f79dc
 r1:0 = combine(r18,r19)
}

{
 jump 0x60
}
