// SPDX-License-Identifier: GPL-3.0-only
// Agent 13: secondary input metadata report + dispatch to QuRT handler.
// File [0x42c50,0x42c8c), VMA [0xb0002c50,0xb0002c8c).
.text
.globl agent13_runtime_metadata_report_aux
agent13_runtime_metadata_report_aux:
{ r16 = r0; memd(r29+#-16) = r17:16; allocframe(#16) }
{ r1 = #0x411; r0 = ##0xb8; r2 = memub(r16+#3) }
{ call 0x218550; memw(r29+#0) = r2 }
{ call -0x24; r0 = ##0xb02a630c; r1 = memub(r16+#3) }
{ r1:0 = combine(##0xb0002aa4,r16); r2 = #0; r17:16 = memd(r29+#8) }
{ jump 0x36144; deallocframe }
