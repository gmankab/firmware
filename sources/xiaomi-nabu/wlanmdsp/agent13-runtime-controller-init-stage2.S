// SPDX-License-Identifier: GPL-3.0-only
// Controller mode tables and per-buffer descriptor setup, semantic Hexagon source.
.text
.globl agent13_controller_initialize_stage2
agent13_controller_initialize_stage2:
L_000fc:
{
  immext(#448)
  r26 = +mpyi(r24,##488)
  r20 = #0
  r27 = #1 }
L_0010c:
{
  immext(#960)
  r26 += add(r23,##968)
  r22 = r17 }
L_00118:
{
  r16 = add(r26,#12) }
L_0011c:
{
  r3 = add(r27,#-1) }
L_00120:
{
  p0 = !tstbit(r2,r3)
  if (p0.new) jump:nt 0x3d4 }
L_00128:
{
  r2 = memub(r23+#1)
  immext(#1792)
  if (!cmp.gtu(r2.new,r20)) jump:nt 0x718 }
L_00134:
{
  r20 = add(r20,#1)
  r2 = memw(r25+#0)
  memw(r16+#0) = r2.new }
L_00140:
{
  call 0x14d808
  r0 = r20
  memw(r16+#8) = r3; 	memw(r16+#4) = r20 }
L_0014c:
{
  memw(r16+#196) = r0
  r2 = memub(r22+#-32) }:mem_noshuf
L_00154:
{
  p0 = tstbit(r2,#0); if (!p0.new) jump:t 0xc }
L_00158:
{
  memw(r16+#12) = setbit(#0) }
L_0015c:
{
  r2 = memub(r22+#-32) }
L_00160:
{
  p0 = !tstbit(r2,#1)
  if (p0.new) jump:t 0x10 }
L_00168:
{
  memw(r16+#12) = setbit(#1) }
L_0016c:
{
  r2 = memub(r22+#-32) }
L_00170:
{
  p0 = !tstbit(r2,#2)
  if (p0.new) jump:t 0x10 }
L_00178:
{
  memw(r16+#12) = setbit(#2) }
L_0017c:
{
  r2 = memub(r22+#-32) }
L_00180:
{
  p0 = !tstbit(r2,#3)
  if (p0.new) jump:t 0x10 }
L_00188:
{
  memw(r16+#12) = setbit(#3) }
L_0018c:
{
  r2 = memub(r22+#-32) }
L_00190:
{
  p0 = !tstbit(r2,#4)
  if (p0.new) jump:t 0xc }
L_00198:
{
  memw(r16+#12) = setbit(#4) }
L_0019c:
{
  r3 = #2
  memw(r16+#16) = #0
  r2 = memw(r22+#-44) }:mem_noshuf
L_001a8:
{
  p0 = !tstbit(r2,#1)
  if (!p0.new) r3 = #3
  if (!p0.new) memw(r16+#16) = #1 }
L_001b4:
{
  immext(#4294967232)
  if (!p0) r2 = memw(r22+##4294967252) }
L_001bc:
{
  p0 = tstbit(r2,#0)
  if (p0.new) memw(r16+#16) = r3
  r2 = memw(r22+#-36) }:mem_noshuf
L_001c8:
{
  memw(r16+#20) = r2 }
L_001cc:
{
  r3 = memw(r22+#-40)
  if (cmp.eq(r3.new,#2)) jump:t 0x10 }
L_001d4:
{
  r2 = #4
  p0 = cmp.eq(r3,#3); if (!p0.new) jump:t 0x33c }
L_001dc:
{
  r2 = r3 }
L_001e0:
{
  memw(r16+#24) = #1 }
