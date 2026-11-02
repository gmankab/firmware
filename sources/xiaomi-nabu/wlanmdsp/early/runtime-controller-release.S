// SPDX-License-Identifier: GPL-3.0-only
// Controller state and optional report/release, file [0x441cc,0x44244).
// All arithmetic/condition/predicate operations are explicit. No OEM opcode data.
.text
.globl agent13_wlan_controller_release
agent13_wlan_controller_release:
L_041cc:
{
  p0 = cmp.eq(r0,#0x0); if (p0.new) jump:nt L_04210
  memd(r29+#-0x10) = r17:16; 	allocframe(#0x10)
}
L_041d4:
{
  r16 = memw(r0+#0x34)
  if (cmp.eq(r16.new,#0x0)) jump:nt L_04214
}
L_041dc:
{
  r1 = #0x4801
  immext(#0x100)
  r0 = ##0x108; 	r2 = memw(r16+#0x10)
}
L_041e8:
{
  r2 = extractu(r2,#0x1,#0x1d)
  call 0x216fcc
  memw(r29+#0x0) = r2.new
}
L_041f4:
{
  immext(#0xb02a6400)
  r0 = ##0xb02a6430; 	r2 = memw(r16+#0x10)
}
L_041fc:
{
  r1 = extractu(r2,#0x1,#0x1d)
  call -0x15b4
}
L_04204:
{
  r2 = memub(r16+#0x13)
}
L_04208:
{
  p0 = !tstbit(r2,#0x5)
  if (!p0.new) jump:t L_04238
}
L_04210:
{
  r17:16 = memd(r29+#0x8); 	dealloc_return
}
L_04214:
{
  immext(#0x100)
  r0 = ##0x108
  r1 = #0x4801
  memw(r29+#0x0) = #0x0
}
L_04224:
{
  call 0x216f90
}
L_04228:
{
  immext(#0xb02a6400)
  r1:0 = combine(#0x0,##0xb02a6430)
  r17:16 = memd(r29+#0x8); 	deallocframe
}
L_04234:
{
  jump -0x15ec
}
L_04238:
{
  r0 = r16
  r17:16 = memd(r29+#0x8); 	deallocframe
}
L_04240:
{
  jump 0xe94
}
