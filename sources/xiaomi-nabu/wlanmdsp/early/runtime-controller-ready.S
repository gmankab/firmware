// SPDX-License-Identifier: GPL-3.0-only
// Fast controller state probe and guarded tail callback. File 0x43f54–0x43f88.
// All arithmetic/condition/predicate operations are explicit. No OEM opcode data.
.text
.globl agent13_wlan_controller_ready
agent13_wlan_controller_ready:
L_03f54:
{
  r2 = memw(r0+#0x84)
  if (!cmp.eq(r2.new,#0x0)) jump:t L_03f64
}
L_03f5c:
{
  r2 = memw(r0+#0xe8)
  if (cmp.eq(r2.new,#0x0)) jump:nt L_03f84
}
L_03f64:
{
  r2 = memw(r0+#0x7c)
  if (cmp.eq(r2.new,#0x0)) jump:nt L_03f70
}
L_03f6c:
{
  jump 0xd68
}
L_03f70:
{
  r2 = memw(r0+#0x78)
  if (cmp.eq(r2.new,#0x0)) jump:nt L_03f84
}
L_03f78:
{
  r2 = memw(r0+#0x8)
}
L_03f7c:
{
  jump 0x63c
  r0 = memw(r2+#0x2c)
}
L_03f84:
{
  jumpr r31
}
