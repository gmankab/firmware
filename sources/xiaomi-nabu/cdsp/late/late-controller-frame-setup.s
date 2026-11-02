// Original source-level operations for the next CDSP callback/controller routine.
// Distinct from the word-array used only for independent disassembly.
.text
.space 0x8f0
next_controller:
{ r0=##0xc0364b98
  r3:2=combine(#0,#0); allocframe(#96) }
{ r4=##0x53594850
  r5=##0x4c4f4f50 }
{ r6=##0x41464544
  r7=##0x5f544c55 }
{ memd(r29+#80)=r19:18
  memd(r29+#88)=r17:16 }
{ r19=#0
  memd(r29+#72)=r21:20
  memd(r29+#64)=r23:22 }
{ memd(r29+#48)=r3:2
  memd(r29+#40)=r3:2 }
{ memd(r29+#32)=r5:4
  memd(r29+#24)=r7:6 }
