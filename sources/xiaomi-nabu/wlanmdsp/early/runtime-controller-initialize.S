// SPDX-License-Identifier: GPL-3.0-only
// Parsed mnemonic operands from reference disassembler; typed Hexagon packet source.
.text
.globl agent13_wlan_controller_initialize
agent13_wlan_controller_initialize:
L_00000:
{
  call 0x2a00ec
  allocframe(#32) }
L_00008:
{
  call 0x7c158
  r16 = r0 }
L_00010:
{
  r17 = add(r16,#320)
  r2 = #1 }
L_00018:
{
  call 0xcb92c
  r1:0 = combine(r17,r16) }
L_00020:
{
  r0 = r16
  r2 = memw(r16+#320)
  memw(r16+#556) = r0 }
L_0002c:
{
  r3 = memub(r16+#1)
  r1 = memw(r16+#324) }
L_00034:
{
  r2 = add(r2,r3) }
L_00038:
{
  r1 = addasl(r1,r2,#3)
  call 0xcafd8 }
L_00040:
{
  r0 = r16
  r2 = #1
  r1 = memw(r16+#320)
  memw(r16+#872) = r0 }
L_00050:
{
  call 0xd7a8c }
L_00054:
{
  call 0x10934
  r0 = r16
  r1 = memub(r16+#320)
  memw(r16+#876) = r0 }
L_00064:
{
  call 0xe1730
  r1:0 = combine(#1,r16)
  r2 = #0
  memw(r16+#688) = r0 }
L_00074:
{
  call 0xb7698
  r1:0 = combine(#1,r16)
  memw(r16+#684) = r0 }
L_00080:
{
  call 0x108b38
  r1:0 = combine(#1,r16)
  memw(r16+#576) = r0 }
L_0008c:
{
  call 0xc5ab0
  r1:0 = combine(#1,r16)
  memw(r16+#580) = r0 }
L_00098:
{
  call 0xbcf84
  r1:0 = combine(#1,r16)
  memw(r16+#584) = r0 }
L_000a4:
{
  call 0xc3948
  r0 = r16
  r1 = memw(r16+#448) }
L_000b0:
{
  r0 = add(r29,#12)
  r1 = r16
  r2 = memw(r16+#400)
  memw(r16+#680) = r0 }
L_000c0:
{
  call 0x23420
  r4 = #0
  r3 = memw(r16+#404) }
L_000cc:
{
  r0 = r16
  r1 = memw(r16+#428)
  r2 = memw(r29+#12) }
L_000d8:
{
  call -0x2354
  memw(r16+#560) = r2 }
L_000e0:
{
  call 0x22442c
  r0 = r16
  r1 = memw(r16+#428)
  memw(r16+#152) = r0 }
L_000f0:
{
  call 0x7df20
  r0 = r16
  memw(r16+#864) = r0 }
L_000fc:
{
  call 0xd8d4c
  r0 = r16
  r1 = memw(r16+#320)
  memw(r16+#920) = r0 }
L_0010c:
{
  r0 = r16
  r1 = memw(r16+#508)
  memw(r16+#880) = r0 }
L_00118:
{
  p0 = cmp.eq(r1,#0)
  r2 = memw(r16+#500)
  immext(#320)
  if (p0.new) r1 = memw(r16+##320) }
L_00128:
{
  call 0xc06bc }
L_0012c:
{
  call -0x2bec
  r0 = r16
  memw(r16+#936) = r0 }
L_00138:
{
  call 0xd8c38
  r0 = r16 }
L_00140:
{
  call -0x2b40
  r0 = r16 }
L_00148:
{
  r0 = add(r29,#4)
  r2 = memw(r16+#432)
  memw(r16+#916) = r0 }
L_00154:
{
  r1 = memw(r16+#560)
  memw(r29+#4) = r2 }
L_0015c:
{
  immext(#2956214784)
  r2 = memw(##2956214844)
  r3 = memw(r16+#436) }
L_00168:
{
  callr r2
  memw(r29+#8) = r3 }
L_00170:
{
  call 0xfb710
  memw(r16+#148) = r0 }
L_00178:
{
  r0 = r16
  r2 = memw(gp+#2020) }
L_00180:
{
  r2 = memw(r2+#0) }
L_00184:
{
  callr r2 }
L_00188:
{
  r0 = r16
  immext(#2956222080)
  r2 = memw(##2956222108)
  memw(r16+#800) = r0 }
L_00198:
{
  callr r2 }
L_0019c:
{
  call 0x21fc90
  r0 = r16
  memw(r16+#724) = r0 }
L_001a8:
{
  r1:0 = combine(#1,r16)
  r2 = memw(r16+#320)
  memw(r16+#592) = r0 }
L_001b4:
{
  call 0xf1610
  r3 = memw(r16+#560) }
L_001bc:
{
  call 0x40900
  r0 = r16
  memw(r16+#600) = r0 }
L_001c8:
{
  call 0x44830
  r0 = r16 }
L_001d0:
{
  call 0xba1a8
  r0 = r16 }
L_001d8:
{
  r1:0 = combine(#1,r16)
  immext(#2956220288)
  r2 = memw(##2956220316) }
L_001e4:
{
  callr r2 }
L_001e8:
{
  call 0x80600
  r0 = r16 }
L_001f0:
{
  r0 = r16
  immext(#2956214720)
  r2 = memw(##2956214756) }
L_001fc:
{
  callr r2 }
L_00200:
{
  call 0xc4de8
  r0 = r16
  r1 = memw(r16+#504) }
L_0020c:
{
  immext(#2956222144)
  r2 = memw(##2956222204)
  r0 = memw(r16+#560) }
L_00218:
{
  callr r2 }
L_0021c:
{
  call 0xfdf94
  r1:0 = combine(r17,r16)
  r2 = #1 }
L_00228:
{
  call 0xda9d8
  r1:0 = combine(#1,r16)
  memw(r16+#596) = r0 }
L_00234:
{
  call 0x5c60
  r1:0 = combine(#1,r16)
  memw(r16+#732) = r0 }
L_00240:
{
  call 0x223098
  r0 = r16
  memw(r16+#748) = r0 }
L_0024c:
{
  r2 = memb(r16+#1)
  memw(r16+#792) = r0 }
L_00254:
{
  r18 = #0
  r17 = add(r16,#808)
  p0 = cmp.eq(r2,#0); if (p0.new) jump:nt 0x64 }
L_00260:
{
  r2 = memw(r17+#0) }
L_00264:
{
  call -0x270
  r0 = memw(r2+#52) }
L_0026c:
{
  r2 = memw(r17+#0) }
L_00270:
{
  memw(r2+#60) = r0
  r2 = memw(r17+#0) }:mem_noshuf
L_00278:
{
  call 0xcb448
  r2 = #0
  r0 = memw(r2+#52); 	r1 = memw(r2+#60) }
L_00284:
{
  r2 = memw(r17+#0) }
L_00288:
{
  call -0x294
  r0 = memw(r2+#52) }
L_00290:
{
  r2 = memw(r17+#0) }
L_00294:
{
  memw(r2+#64) = r0
  r2 = memw(r17+#0) }:mem_noshuf
L_0029c:
{
  call -0x2a8
  r0 = memw(r2+#52) }
L_002a4:
{
  r18 = add(r18,#1)
  r2 = memw(r17++#4) }
L_002ac:
{
  memw(r2+#68) = r0 }
L_002b0:
{
  r2 = memub(r16+#1)
  if (cmp.gt(r2.new,r18)) jump:t -0x50 }
L_002b8:
{
  call 0x890f8 }
L_002bc:
{
  call 0x219b34 }
L_002c0:
{
  p0 = cmp.eq(r0,#1) }
L_002c4:
{
  immext(#533504)
  if (!p0) call 0x8240c }
L_002cc:
{
  call 0xec45c
  r0 = r16
  r1 = memw(r16+#400)
  r2 = memw(r16+#560) }
L_002dc:
{
  immext(#2956219584)
  r2 = memw(##2956219644)
  memw(r16+#796) = r0 }
L_002e8:
{
  callr r2 }
L_002ec:
{
  call 0x8754
  r0 = memw(r16+#320) }
L_002f4:
{
  call 0x76918
  r0 = r16 }
L_002fc:
{
  r0 = r16
  r2 = memw(r16+#508) }
L_00304:
{
  p0 = cmp.eq(r2,#0)
  immext(#320)
  if (p0.new) r2 = memw(r16+##320) }
L_00310:
{
  call 0x222f4c
  r1 = zxth(r2) }
L_00318:
{
  call 0x7b86c
  r0 = memub(r16+#320)
  immext(#4096)
  memw(r16+##4108) = r0 }
L_00328:
{
  call 0xf8cf8
  r0 = r16
  immext(#4096)
  memw(r16+##4112) = r0 }
L_00338:
{
  immext(#2956214400)
  r2 = memw(##2956214400)
  immext(#4544)
  memw(r16+##4556) = r0 }
L_00348:
{
  r0 = r16 }
L_0034c:
{
  callr r2 }
L_00350:
{
  call 0x10576c
  r0 = r16
  r1 = memw(r16+#320) }
L_0035c:
{
  call 0x224b98
  r0 = r16
  immext(#4544)
  memw(r16+##4580) = r0 }
L_0036c:
{
  call 0x1630
  r0 = r16
  immext(#4544)
  memw(r16+##4584) = r0 }
L_0037c:
{
  call 0x4fbb8
  r0 = r16 }
L_00384:
{
  call -0x3fc
  r0 = r16 }
L_0038c:
{
  r0 = #0; jump ##0x29fe3c }
