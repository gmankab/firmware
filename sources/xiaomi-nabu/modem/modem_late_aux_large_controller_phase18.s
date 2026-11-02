// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V60 64-bit buffer-length accumulation and descriptor-packet flags.
.text
.globl modem_late_aux_large_controller_phase18
modem_late_aux_large_controller_phase18:
{
  r5:4 = combine(#0,#-1)
  r9 = #0
  r8 = memw(r24+#0)
  r6 = memw(r17+#28)
}
{
  r3:2 = add(r3:2,r5:4)
  r4 = sub(#0,r8)
  r7 = ##262143
}
{r3:2 = sub(r3:2,r1:0)}
{r3:2 = add(r3:2,r9:8)}
{r2 = and(r2,r4)}
{
  p0 = bitsclr(r2,r7)
  r3 = add(r2,##262144)
}
{r3 = insert(r9,#18,#0)}
{if (p0) r3 = add(r2,#0)}
{memw(r6+#12) = r3}
{r2 = memw(r17+#28)}
{memw(r2+#0) = r0}
{r2 = memw(r17+#28)}
{
  r3 = memw(r16+#52)
  memw(r2+#16) = r3.new
}
{
  r2 = memw(r17+#28)
  r3 = memw(r16+#56)
}
{memw(r2+#20) = r3}
{r2 = memw(r17+#28)}
{memw(r2+#24) = r20}
{r2 = memw(r17+#28)}
{r3 = memw(r29+#84)}
