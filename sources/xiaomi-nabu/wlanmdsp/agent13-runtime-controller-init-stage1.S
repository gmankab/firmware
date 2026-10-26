// SPDX-License-Identifier: GPL-3.0-only
// Controller initialization phase 1. Semantic packet source only; no OEM words.
.text
.globl agent13_wlan_controller_initialize_stage1
agent13_wlan_controller_initialize_stage1:
L_00014:
{
  call 0x29fb00
  r0 = #1136
  allocframe(#64) }
L_00020:
{
  call 0x838 }
L_00024:
{
  call -0x3c88
  r2 = #1136
  r1 = #0; 	r18 = r0 }
L_00030:
{
  r2 = memw(gp+#1932)
  if (!cmp.eq(r2.new,#0)) jump:t 0x14 }
L_00038:
{
  call 0x820
  r0 = #4664 }
L_00040:
{
  memw(gp+#1932) = r0 }
L_00044:
{
  call 0x1ff18c
  r0 = r18 }
L_0004c:
{
  r2 = memw(gp+#1932)
  r3 = memw(r18+#52) }
L_00054:
{
  memb(r2+#1) = r3
  r2 = memw(gp+#1932) }:mem_noshuf
L_0005c:
{
  r3 = memw(r18+#56)
  memb(r2+#2) = r3.new }
L_00064:
{
  r2 = memw(gp+#1932)
  memw(r29+#8) = r0 }
L_0006c:
{
  call 0x1f82c0
  r0 = add(r2,#4560)
  r1 = add(r2,#4561) }
L_00078:
{
  r23 = memw(gp+#1932) }
L_0007c:
{
  r2 = memb(r23+#1)
  if (cmp.eq(r2.new,#0)) jump:nt 0x40 }
L_00084:
{
  immext(#768)
  r17:16 = combine(##808,#0)
  nop
  nop }
L_00094:
{
  call 0x7c4
  r0 = #248 }
L_0009c:
{
  r16 = add(r16,#1)
  memb(r0+#0) = r16
  r23 = memw(gp+#1932) }:mem_noshuf
L_000a8:
{
  r17 = add(r17,#4)
  memw(r0+#40) = r23
  memw(r23+r17<<#0) = r0 }
L_000b4:
{
  r2 = memub(r23+#1)
  if (cmp.gtu(r2.new,r16)) jump:t -0x20 }
L_000bc:
{
  r21 = add(r18,#60)
  r17 = add(r18,#144)
  r19 = #0
  r24 = #0 }
L_000cc:
{
  memw(r29+#12) = r18 }
L_000d0:
{
  r2 = +mpyi(r24,#168)
  immext(#448)
  r3 = add(r23,mpyi(r24,##488)) }
L_000dc:
{
  r25 = add(r21,mpyi(#168,r24)) }
L_000e0:
{
  r2 = memw(r21+r2<<#0)
  memw(r3+#968) = r2.new }:mem_noshuf
L_000e8:
{
  r2 = memb(r23+#2) }
