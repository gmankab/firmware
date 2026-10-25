// SPDX-License-Identifier: GPL-3.0-only
// Agent13: auxiliary WLAN record reporter and diagnostic tailcall wrapper.
// File [0x42a54,0x42aa4), VMA [0xb0002a54,0xb0002aa4).
.text
.globl agent13_runtime_record_forward
agent13_runtime_record_forward:
{ r16 = r0; memd(r29+#-16) = r17:16; allocframe(#16) }
{ r1 = #0x402; r0 = ##0x80; r2 = memw(r16+#40) }
{ r3 = memub(r16+#592) }
{ r2 = addasl(r2,r3,#2); r4 = memub(r16+#3) }
{ r17 = memw(r2+#808) }
{ r2 = memub(r17+#0); memw(r29+#4) = r2.new }
{ call 0x218734; memw(r29+#0) = r4 }
{ r0 = ##0xb02a62b8; r1 = memub(r16+#3); r2 = memub(r17+#0) }
{ call -0x774 }
{ r0 = r17; r17:16 = memd(r29+#8); deallocframe }
{ jump -0x8c }
