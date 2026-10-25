// SPDX-License-Identifier: GPL-3.0-only
// Agent13: short diagnostic wrapper and tailcall to record marker check.
// File [0x42a14,0x42a54), VMA [0xb0002a14,0xb0002a54).
// Saved frame is removed before the tail transfer to the record classifier.
.text
.globl agent13_runtime_record_report
agent13_runtime_record_report:
{ call -0xb4; r16 = r0; memd(r29+#-16) = r17:16; allocframe(#16) }
{ r1 = #0x402; r0 = ##0x78; r17 = r0 }
{ r2 = memub(r16+#0); memw(r29+#4) = r17 }
{ call 0x218784; memw(r29+#0) = r2 }
{ r0 = ##0xb02a62ac; r2 = r17; r1 = memub(r16+#0) }
{ call -0x724 }
{ r1:0 = combine(r17,r16); r17:16 = memd(r29+#8); deallocframe }
{ jump -0x348 }
