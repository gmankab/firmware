// SPDX-License-Identifier: GPL-3.0-only
// Independently authored Hexagon V60 controller-return paths. Original V66
// packet arrangement is preserved by typed source emitter and decoded Rizin.
.text
.space 0x848
success_path:
{ call #0xf8264 }
{ r1:0=combine(#0,#1) }
{ r2=memw(r16+#0)
  r17:16=memd(r29+#40) }
{ r2=asl(r2,#12) }
{ r19:18=memd(r29+#32)
  r21:20=memd(r29+#24) }
{ r23:22=memd(sp+#16)
  r25:24=memd(sp+#8) }
{ jump #0xf8248
  r27:26=memd(sp+#0)
  deallocframe }
empty_path:
{ r1:0=combine(#0,#1) }
{ r2=#0 }
{ r17:16=memd(r29+#40)
  r19:18=memd(r29+#32) }
{ r21:20=memd(r29+#24)
  r23:22=memd(r29+#16) }
{ r25:24=memd(sp+#8)
  r27:26=memd(sp+#0) }
{ jump #0xf8224
  deallocframe }
