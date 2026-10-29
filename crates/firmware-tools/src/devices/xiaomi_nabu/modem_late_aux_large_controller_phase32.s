// SPDX-License-Identifier: GPL-3.0-only
// Translate controller record descriptors into the selected buffer slot.
// Hexagon V60: scaled record indexes, 64-bit descriptor copy and control
// flags, using symbolic register semantics rather than OEM instruction words.
.text
.globl modem_late_aux_large_controller_phase32
modem_late_aux_large_controller_phase32:
{
  r6 = add(r2,mpyi(#28,r8))
  r3 = +mpyi(r8,#28)
  r5 = memw(r17+#36)
  r12 = memw(r18+#4)
}
{
  r23 = memw(r18+#-12)
  r24 = memw(r18+#0)
}
{
  r0 = memw(r6+#4)
  r27 = memw(r2+r3<<#0)
}
{
  r2 = add(r0,r26)
  r1 = memw(r6+#8)
  memw(r29+#44) = r1.new
}
{
  r6 = sub(r2,r27)
  memw(r5+r4<<#0) = r6.new
}
{
  r0 = memw(r17+#36)
  r2 = addasl(r0,r7,#5)
}
{r1:0 = memd(r29+#64)}
{
  r13 = r1
  r25 = r1
}
{
  memd(r29+#32) = r13:12
  memd(r2+#8) = r13:12
}
{
  r2 = memw(r18+#8)
  r3 = memw(r17+#36)
}
{
  p0 = tstbit(r2,#0)
  r4 = clrbit(r2,#1)
}
{
  r5 = extractu(r2,#1,#2)
  r3 = addasl(r3,r7,#5)
  if (!p0) r4 = add(r2,#0)
}
{r4 = or(r5,and(r4,#2))}
{
  r4 = insert(r2,#1,#2)
  memw(r3+#16) = r4.new
}
{
  r2 = memw(r17+#28)
  r3 = memw(r17+#36)
}
{
  r2 = add(r2,mpyi(#28,r8))
  r3 = addasl(r3,r7,#5)
}
{
  r2 = memw(r2+#20)
  memw(r3+#20) = r2.new
}
{
  r2 = memw(r17+#36)
}
{r2 = addasl(r2,r7,#5)}
{
  memw(r2+#24) = r20
  memw(r29+#52) = r6
}
