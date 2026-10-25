// SPDX-License-Identifier: GPL-3.0-only
// Agent 13: log WLAN per-frame metadata then delegate to common QuRT record
// serializer. Tail thunk returns via a platform helper, no OEM bytes embedded.
// File [0x42c04,0x42c50), VMA [0xb0002c04,0xb0002c50).
.text
.globl agent13_runtime_metadata_report
agent13_runtime_metadata_report:
{ r16 = r0; memd(r29+#-16) = r17:16; allocframe(#16) }
{ r1 = #0x411; r0 = ##0xb0; r2 = memub(r16+#3) }
{ call 0x21859c; memw(r29+#0) = r2 }
{ call 0x28; r0 = ##0xb02a6300; r1 = memub(r16+#3) }
{ call -0x1d8; r0 = r16 }
{ r1:0 = combine(##0xb0002aa4,r16); r2 = #0; r17:16 = memd(r29+#8) }
{ jump 0x35894; deallocframe }
.globl agent13_tail_zero_result
agent13_tail_zero_result:
{ jump ##-0x1b68 }
