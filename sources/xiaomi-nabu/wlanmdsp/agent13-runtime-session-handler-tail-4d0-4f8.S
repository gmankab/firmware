// SPDX-License-Identifier: GPL-3.0-only
// Source-defined WLAN session handler Hexagon V65-compatible V66 subset, file 0x47a88..0x47ab0.
.text
.globl agent13_session_handler_tail_4d0_4f8
agent13_session_handler_tail_4d0_4f8:

{
 r2 = memw(r29+#12)
}

{
 p0 = r2
 if (p0.new) jump:nt 0xe4
}

{
 r2 = #3
 if (cmp.gtu(r2.new,r16)) jump:t 0x9c
}

{
 call -0x1f54
 r0 = ##-1339396993
 r1 = #546
}

{
 r2 = memw(r29+#12)
}
