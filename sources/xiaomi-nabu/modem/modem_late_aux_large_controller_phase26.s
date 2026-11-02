// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V60 indexed controller-resource field copying and scaled strides.
.text
.globl modem_late_aux_large_controller_phase26
modem_late_aux_large_controller_phase26:
{
  r2 = +mpyi(r18,#28)
  r5 = memw(r17+#28)
  r6 = memw(r2+#0)
}
{
  r5 = add(r5,mpyi(#28,r18))
  r1 = r2
}
{memw(r5+#16) = r6}
{r5 = memw(r17+#28)}
{
  r5 = add(r5,mpyi(#28,r18))
  r4 = memw(r4+#0)
}
{memw(r5+#20) = r4}
{r4 = memw(r17+#28)}
{
  r4 = add(r4,mpyi(#28,r18))
  r5 = memw(r22+#-4)
}
{memw(r4+#12) = r5}
{r4 = memw(r17+#28)}
{
  r5 = memw(r22+#-16)
  memw(r4+r2<<#0) = r5.new
}
{r4 = memw(r17+#28)}
{r4 = add(r4,mpyi(#28,r18))}
{memw(r4+#24) = r20}
{r4 = memw(r17+#28)}
{
  r6 = add(r4,mpyi(#28,r18))
  r1 += add(r4,#4)
}
{r2 += add(r4,#8)}
{
  callr r3
  r0 = memw(r6+#12)
}
