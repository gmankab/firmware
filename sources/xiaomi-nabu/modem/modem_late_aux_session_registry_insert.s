// SPDX-License-Identifier: GPL-3.0-only
// Independently assembled source for Hexagon V60 controller registry
// insertion/validation, modem ELF file [0x447100,0x447234).
// VMA [0x1e401100,0x1e401234), physical [0x90001100,0x90001234).
// Disjoint from earlier unsupported 'framekey' instruction, which remains
// explicitly unclaimed. All calls use symbolic link-time relative targets.
.text
.globl modem_late_aux_session_registry_insert
modem_late_aux_session_registry_insert:
.set ext_1e4007a0, modem_late_aux_session_registry_insert + (-2400)
.set ext_1e401028, modem_late_aux_session_registry_insert + (-216)
.set ext_1e401078, modem_late_aux_session_registry_insert + (-136)
{
  r17:16 = combine(r1,r0)
  memd(r29+#-16) = r17:16
  allocframe(#32)
}
{
  p0 = cmp.eq(r16,#0)
  memd(r29+#16) = r19:18
  memd(r29+#8) = r21:20
}
{
  if (p0) jump:nt .L1e401120
  memd(r29+#0) = r23:22
}
{
  if (!p0.new) r0 = add(r16,#0)
  p0 = cmp.eq(r17,#0)
  if (!p0.new) jump:t .L1e401154
}
.L1e401120:
{
  r4 = memw(##507617104)
  if (cmp.eq(r4.new,#0)) jump:nt .L1e401144
}
{
  r1:0 = combine(##507576347,#3)
  r2 = #722
}
{
  r3 = ##507576661
  callr r4
}
.L1e401144:
{
  call ext_1e4007a0
  r0 = ##507576661
  r18 = #0
}
{
  jump .Ldone
}
.L1e401154:
{
  call ext_1e401028
}
{
  r19 = r0
  if (!cmp.eq(r19.new,#0)) jump:t .L1e401194
}
{
  r4 = memw(##507617104)
  if (cmp.eq(r4.new,#0)) jump:nt .L1e401184
}
{
  r1:0 = combine(##507576347,#3)
  r2 = #728
}
{
  r3 = ##507576696
  callr r4
}
.L1e401184:
{
  call ext_1e4007a0
  r0 = ##507576696
  r18 = #0
}
{
  jump .Ldone
}
.L1e401194:
{
  r18 = #1
  r2 = memw(r19+#4)
  memw(r17+#0) = r2.new
}
{
  r2 = memw(r19+#28)
}
{
  r2 = memw(r2+#4)
  memw(r17+#4) = r2.new
}
{
  memw(r17+#12) = #0
  memw(r17+#8) = #0
}
{
  r2 = memw(r19+#116)
  if (cmp.eq(r2.new,#0)) jump:nt .Ldone
}
{
  r21:20 = combine(#12,#0)
  r22 = ##65522
}
.Lloop:
{
  r2 = memw(r19+#92)
}
{
  r23 = add(r2,r21)
}
{
  r2 = memuh(r23+#2)
  if (cmp.eq(r2.new,#0)) jump:nt .Literate
}
{
  p0 = cmp.eq(r2,r22)
  if (p0.new) jump:t .Literate
  if (!p0.new) r1 = memw(r23+##4294967288)
}
{
  call ext_1e401078
  r0 = r19
}
{
  p0 = cmp.gtu(r0,r16)
  if (p0.new) jump:t .Literate
}
{
  r2 = memw(r17+#12)
  if (cmp.gtu(r2.new,r0)) jump:t .Literate
}
{
  p0 = !cmp.eq(r0,r16)
  r2 = memw(r19+#96)
  r3 = memw(r23+#-12)
}
{
  r2 = add(r2,r3)
  memw(r17+#8) = r2.new
}
{
  if (p0) jump:nt .Literate
  memw(r17+#12) = r0
}
{
  r2 = memub(r23+#0)
  if (cmp.gtu(r2.new,#15)) jump:nt .Ldone
}
.Literate:
{
  r21 = add(r13,#16)
  r20 = add(r20,#1)
}
{
  r2 = memw(r19+#116)
  if (cmp.gtu(r2.new,r20)) jump:t .Lloop
}
.Ldone:
{
  r0 = r18
  r17:16 = memd(r29+#24)
  r19:18 = memd(r29+#16)
}
{
  r21:20 = memd(r29+#8)
  r23:22 = memd(r29+#0)
}
{
  dealloc_return
}
