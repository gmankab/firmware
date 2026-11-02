// SPDX-License-Identifier: GPL-3.0-only
// Record-status/control path with diagnostic callbacks, file [0x443e4,0x4443c).
// All arithmetic/condition/predicate operations are explicit. No OEM opcode data.
.text
.globl agent13_wlan_record_status_handler
agent13_wlan_record_status_handler:
L_043e4:
{
  call 0x2a1394
  allocframe(#0x28)
}
L_043ec:
{
  r17:16 = combine(r2,r0)
  r18 = r1
  memw(r29+#0x0) = r18.new
}
L_043f8:
{
  call 0x224de8
  r0 = memw(r16+#0x8)
  memw(r29+#0x4) = r0.new
}
L_04404:
{
  call 0x224e70
  r0 = memw(r16+#0x8); 	memw(r29+#0xc) = r0
}
L_0440c:
{
  r0 = r16
  r1 = add(r29,#0x0)
  r2 = memw(r16+#0x48)
  memw(r29+#0x10) = r0
}
L_0441c:
{
  call 0x544
  memw(r29+#0x8) = r17; 	memw(r29+#0x14) = r2
}
L_04424:
{
  p0 = cmp.eq(r18,#0x2); if (!p0.new) jump:t L_04438
}
L_04428:
{
  call 0x7e7fc
  r0 = memw(r16+#0x8)
}
L_04430:
{
  call 0x216454
  r1:0 = combine(#0x1,#0x1)
}
L_04438:
{
  jump 0x2a141c
}
