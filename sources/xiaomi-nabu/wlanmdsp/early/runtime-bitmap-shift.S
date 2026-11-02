// SPDX-License-Identifier: GPL-3.0-only
// Three-entry controller bitmap left shift with QuRT hardware loop0.
// File [0x445c0,0x44614).
// All arithmetic/condition/predicate operations are explicit. No OEM opcode data.
.text
.globl agent13_wlan_bitmap_shift
agent13_wlan_bitmap_shift:
L_045c0:
{
  loop0(L_045cc,#0x3)
  r4 = add(r0,#0x1c)
  r2 = memw(r0+#0x1c)
}
L_045cc:
{
  r3 = r4
}
L_045d0:
{
  r4 = add(r3,#-0x8)
  r5 = memw(r3+#-0x4)
}
L_045d8:
{
  r6 = lsr(r5,#0x1f)
}
L_045dc:
{
  r6 |= asl(r2,#0x1)
  r2 = memw(r3+#-0x8)
  memw(r3+#0x0) = r6.new
}
L_045e8:
{
  r7 = lsr(r2,#0x1f)
}
L_045ec:
{
  r7 |= asl(r5,#0x1)
  memw(r3+#-0x4) = r7.new
}:endloop0
L_045f4:
{
  r4 = memw(r3+#-0xc)
}
L_045f8:
{
  r4 = lsr(r4,#0x1f)
}
L_045fc:
{
  r4 |= asl(r2,#0x1)
  memw(r3+#-0x8) = r4.new
}
L_04604:
{
  r1 = memw(r0+#0x0)
}
L_04608:
{
  r2 = asl(r1,#0x1)
  jumpr r31
  memw(r0+#0x0) = r2.new
}
