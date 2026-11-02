// SPDX-License-Identifier: GPL-3.0-only
// Parsed mnemonic operands from reference disassembler; typed Hexagon packet source.
.text
.globl agent13_wlan_controller_initialize
agent13_wlan_controller_initialize:
L_004f8:
{
  loop0(L_00514,r7)
  immext(#960)
  r6 += add(r4,##1004) }
L_00504:
{
  immext(#960)
  r2 += add(r4,##968)
  r4 = memw(r8+#972) }
L_00510:
{
  r7 = #0 }
L_00514:
{
  p2 = !tstbit(r4,r7)
  if (p2.new) jump:nt 0x4c }
L_0051c:
{
  if (p0) jump:nt 0x2c
  immext(#4294967232)
  if (p0) r8 = memw(r6+##4294967288) }
L_00528:
{
  if (!p1) jump:nt 0x38
  immext(#4294967232)
  if (p1) r8 = memub(r6+##4294967288) }
L_00534:
{
  p2 = !tstbit(r8,#1)
  if (p2.new) jump:t 0x2c }
L_0053c:
{
  r8 = +mpyi(r7,#240) }
L_00540:
{
  r8 += add(r2,#68)
  jump 0x14 }
L_00548:
{
  p2 = tstbit(r8,#0)
  r8 = r6
  if (!p2.new) jump:t 0x18 }
L_00554:
{
  r0 = #0
  r8 = memw(r8+#0) }
L_0055c:
{
  r5 = maxu(r5,r8) }
L_00560:
{
  r7 = add(r7,#1)
  r6 = add(r6,#240) }:endloop0
L_00568:
{
  p0 = cmp.eq(r3,#0); 	if (p0.new) jumpr:nt r31 }
L_0056c:
{
  p0 = cmp.eq(r0,#0)
  if (p0.new) r0 = #0
  if (p0.new) memw(r3+#0) = r5 }
L_00578:
{
  jumpr r31 }
L_0057c:
{
  r0 = #-1; 	jumpr r31 }
