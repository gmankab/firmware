// SPDX-License-Identifier: GPL-3.0-only
// Parsed mnemonic operands from reference disassembler; typed Hexagon packet source.
.text
.globl agent13_wlan_controller_table_create
agent13_wlan_controller_table_create:
L_00394:
{
  r16 = r0
  r0 = #80
  memd(r29+#-16) = r17:16; 	allocframe(#8) }
L_003a0:
{
  call 0x21c398
  immext(#256)
  r3:2 = combine(##256,#9)
  r1 = memw(r16+#320) }
L_003b0:
{
  p0 = cmp.eq(r0,#0); if (p0.new) jump:nt 0xc
  memw(r16+#156) = r0 }
L_003b8:
{
  r17:16 = memd(r29+#0); 	dealloc_return }
L_003bc:
{
  immext(#2955568576)
  r0 = ##2955568608; 	r17:16 = memd(r29+#0) }
L_003c4:
{
  // Source order forces the OEM slot assignment: r2 then immediate extension.
  immext(#2955568576)
  r1 = ##2955568619; deallocframe
  r2 = #25 }
L_003d0:
{
  jump 0x217568 }
L_003d4:
{
  r2 = memw(r0+#40) }
L_003d8:
{
  jump 0x21c6c8
  r0 = memw(r2+#156) }
L_003e0:
{
  r2 = memw(r0+#40) }
L_003e4:
{
  jump 0x21c5d4
  r0 = memw(r2+#156) }
L_003ec:
{
  call 0x29fd00
  allocframe(#16) }
L_003f4:
{
  call 0x48
  r17:16 = combine(r1,r2)
  r18 = r0 }
L_00400:
{
  r3 = add(r18,#4532)
  immext(#4480)
  r2 = memw(r18+##4536)
  if (cmp.eq(r2.new,#2)) jump:nt 0x28 }
L_00410:
{
  r2 = addasl(r3,r2,#3)
  r0 = #0
  r3 = add(r18,#4536) }
L_0041c:
{
  memw(r2+#8) = r17; 	memw(r2+#12) = r16 }
L_00420:
{
  jump 0x29fda8
  memw(r3+#0) += #1 }
L_00428:
{
  call 0x29fd38
  immext(#2955569856)
  r0 = ##-1339397417
  r1 = #259 }
L_00438:
{
  jump 0x84 }
