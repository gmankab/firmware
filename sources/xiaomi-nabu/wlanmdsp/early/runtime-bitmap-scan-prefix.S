// SPDX-License-Identifier: GPL-3.0-only
// Parsed mnemonic operands from reference disassembler; typed Hexagon packet source.
.text
.globl agent13_wlan_controller_initialize
agent13_wlan_controller_initialize:
L_004cc:
{
  r4 = r0 }
L_004d0:
{
  r7 = memub(r4+#2) }
L_004d4:
{
  if (!p0.new) r5 = #0
  p0 = cmp.eq(r7,#0); if (p0.new) jump:nt 0xa8 }
L_004dc:
{
  immext(#448)
  r8 = add(r4,mpyi(r1,##488))
  immext(#448)
  r6 = +mpyi(r1,##488) }
