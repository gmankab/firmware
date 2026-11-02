// SPDX-License-Identifier: GPL-3.0-only
// Agent13: input record-type router; table of 20 controller message opcodes
// is located in pre-existing ROM/RAM, not in this independently authored text.
// File [0x42aa4,0x42bcc), VMA [0xb0002aa4,0xb0002bcc).
// Dispatch is an indirect jump via the platform-provided pointer table.
.text
.globl agent13_runtime_format_dispatch
agent13_runtime_format_dispatch:
{ call 0x2a2cd4; allocframe(#24) }
{ r17:16 = combine(r1,r0) }
{ r3 = memw(r17+#0); if (cmp.gtu(r3.new,#19)) jump:nt invalid_case }
{ r18 = ##0xb02a62d0; r4 = memw(r3<<#2+##0xb02a61cc) }
{ jumpr r4; r0 = ##0x90; r2 = #5 }

case_classified:
{ r1 = #0x412; r0 = ##0x88; r2 = memub(r16+#3) }
{ call 0x2186d4; memw(r29+#4) = r3; memw(r29+#0) = r2 }
{ r0 = ##0xb02a62c4; r1 = memub(r16+#3) }
{ call -0x7d0; r2 = memw(r17+#0) }
{ call 0x2a2cf4; r0 = r16 }
{ jump -0xac }

case_other:
{ r18 = ##0xb02a62dc; r2 = #7 }
{ r0 = ##0x98 }
{ r3 = memw(r17+#4) }
{ p0 = cmp.eq(r3,#0); if (p0.new) r3 = #0; if (!p0.new) r3 = memub(r3+#61) }
{ r4 = memb(r16+#0); if (!cmp.eq(r4.new,#2)) jump:t invalid_case }
{ p0 = cmp.eq(r3,#1); if (p0.new) jump:nt invalid_case }
{ r3 = memub(r16+#1); if (!cmp.eq(r3.new,#0)) jump:nt invalid_case }
{ r1 = #0x412; r3 = memub(r16+#3); memw(r29+#4) = r2 }
{ call 0x218674; memw(r29+#0) = r3 }
{ r0 = r18; jump -0x58; r1 = memub(r16+#3) }
invalid_case:
{ jump 0x2a2d04 }

case_twelve:
{ r1 = #0x412; r0 = ##0xa0; r2 = memub(r16+#3) }
{ call 0x218654; memw(r29+#0) = r2; memw(r29+#4) = #18 }
{ r0 = ##0xb02a62e8; r2 = memw(r17+#0); r1 = memub(r16+#3) }
{ call 0x2a2c74 }
{ jump -0x85c }

case_thirteen:
{ r1 = #0x412; r0 = ##0xa8; r2 = memub(r16+#3) }
{ call 0x218628; memw(r29+#0) = r2; memw(r29+#4) = #19 }
{ r0 = ##0xb02a62f4; r2 = memw(r17+#0); r1 = memub(r16+#3) }
{ call -0x884 }
{ r2 = memw(r16+#40); r3 = memub(r17+#4) }
{ r2 = add(r2,#808); r4 = memub(r17+#8) }
{ r0 = memw(r2+r3<<#2) }
{ call -0x1a4; r16 = memw(r2+r4<<#2) }
{ call 0x2a2c2c; r0 = r16 }
{ jump -0x1b4 }
