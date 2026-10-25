// SPDX-License-Identifier: GPL-3.0-only
// Agent20-14: file [0x101a3c,0x101a64), runtime [0xb00c1a3c,0xb00c1a64).
// Emit a counted session diagnostic and forward control to the outer error helper.
// Typed Hexagon V66 packet operations, not opaque OEM instructions.
.text
.globl agent14_late_buffer_service_warning
agent14_late_buffer_service_warning:
// +0x0: 2 encoded words
{ r16 = r1; memd(r29+#-16) = r17:16; allocframe(#16) }
// +0x8: 3 encoded words
{ r1 = #9233; r0 = ##25928; memw(r29+#0) = r16 }
// +0x14: 1 encoded words
{ call 0x159764 }
// +0x18: 3 encoded words
{ r1:0 = combine(r16,##-1339330400); r17:16 = memd(r29+#8); deallocframe }
// +0x24: 1 encoded words
{ jump -781848 }
