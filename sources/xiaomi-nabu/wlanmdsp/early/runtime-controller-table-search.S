// SPDX-License-Identifier: GPL-3.0-only
// Parsed mnemonic operands from reference disassembler; typed Hexagon packet source.
.text
.globl agent13_wlan_controller_table_search
agent13_wlan_controller_table_search:
L_0043c:
{
  r8 = #0
  r6 = add(r0,#4532)
  r4 = #4536
  r3 = #10 }
L_0044c:
{
  r5 = memw(r0+r4<<#0) }
L_00450:
{
  p0 = cmp.eq(r5,#0); if (p0.new) jump:nt 0x58 }
L_00454:
{
  loop0(L_00460,r5)
  r7 = add(r6,#8)
  r6 = add(r6,#12) }
L_00460:
{
  r9 = memw(r6+#-4)
  if (!cmp.eq(r9.new,r1)) jump:t 0x10 }
L_00468:
{
  r9 = memw(r6+#0)
  if (cmp.eq(r9.new,r2)) jump:nt 0x14 }
L_00470:
{
  r8 = add(r8,#1)
  r6 = add(r6,#8) }:endloop0
L_00478:
{
  jump 0x30 }
L_0047c:
{
  r2 = add(r5,#-1)
  if (cmp.eq(r2.new,r8)) jump:t 0x20 }
L_00484:
{
  r3 = addasl(r7,r2,#3)
  r2 = memw(r7+r2<<#3)
  memw(r6+#-4) = r2.new }
L_00490:
{
  r2 = memw(r3+#4)
  memw(r6+#0) = r2.new }
L_00498:
{
  r5 = memw(r0+r4<<#0) }
L_0049c:
{
  r2 = add(r5,#-1)
  r3 = #0
  memw(r0+r4<<#0) = r2.new }
L_004a8:
{
  r0 = r3; 	jumpr r31 }
L_004ac:
{
  r3:2 = combine(r0,r1) }
L_004b0:
{
  immext(#2955569856)
  r1:0 = combine(r3,##-1339397428)
  jump 0x217488 }
L_004bc:
{
  r3:2 = combine(r0,r1) }
L_004c0:
{
  immext(#2955569856)
  r1:0 = combine(r3,##-1339397428)
  jump 0x217478 }
