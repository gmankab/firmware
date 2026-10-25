// SPDX-License-Identifier: GPL-3.0-only
// Source-level Hexagon V60 controller registration routine, file
// [0x446c18,0x446e1c),
// VMA [0x1e400c18,0x1e400e1c). Check and lock the controller state,
// resolve registry objects, notify diagnostics for invalid callback
// descriptions, adjust a reference count, and release its resource lock.
// Packet boundaries correspond to the actual program; immext is assembled
// from symbolic absolute operand constants, never from vendor opcode bytes.
.text
.globl modem_late_aux_controller_registration
modem_late_aux_controller_registration:
.set ext_1e400488, modem_late_aux_controller_registration + (-1936)
.set ext_1e400490, modem_late_aux_controller_registration + (-1928)
.set ext_1e4007a0, modem_late_aux_controller_registration + (-1144)
.set ext_1e400e20, modem_late_aux_controller_registration + (520)
.set ext_1e400e60, modem_late_aux_controller_registration + (584)
.set ext_1e401448, modem_late_aux_controller_registration + (2096)
.set ext_1e4037e0, modem_late_aux_controller_registration + (11208)
.set ext_1e4049b0, modem_late_aux_controller_registration + (15768)
.set ext_1e404c10, modem_late_aux_controller_registration + (16376)
.set ext_1e405d78, modem_late_aux_controller_registration + (20832)
.set ext_1e4065d0, modem_late_aux_controller_registration + (22968)
.set ext_1e4065d8, modem_late_aux_controller_registration + (22976)
{
  r16 = r1
  memd(r29+#-16) = r17:16
  allocframe(#56)
}
{
  r19:18 = combine(##507534712,r0)
  memd(r29+#40) = r19:18
}
{
  call ext_1e4065d0
  r0 = ##507612016
  memd(r29+#24) = r23:22
}
{
  r2 = memw(##507588608)
  memw(##507617212) = r2.new
}
{
  r0 = #1
  r22 = memw(##507617076)
}
{
  call ext_1e400488
  memb(##507617064) = r0
}
{
  p0 = cmp.eq(r18,#0)
  if (!p0.new) jump:t .L1e400ca0
  if (!p0.new) r1 = memw(##507617072)
}
{
  r4 = memw(##507617104)
  if (cmp.eq(r4.new,#0)) jump:nt .L1e400c90
}
{
  r1:0 = combine(##507576347,#3)
  r2 = #520
}
{
  r3 = ##507576477
  callr r4
}
.L1e400c90:
{
  call ext_1e4007a0
  r0 = ##507576477
  r17 = #0
}
{
  jump .L1e400dac
}
.L1e400ca0:
{
  r0 = r18
  r2 = r16
}
{
  call ext_1e404c10
}
{
  r17 = r0
  if (!cmp.eq(r17.new,#0)) jump:t .L1e400cb4
}
{
  r17 = #0
  jump .L1e400dac
}
.L1e400cb4:
{
  r2 = memw(r22+#0)
  if (cmp.eq(r2.new,#0)) jump:nt .L1e400da8
}
{
  p0 = cmp.eq(r2,r17)
  if (p0.new) jump:nt .L1e400cfc
}
{
  r4 = memw(##507617104)
  if (cmp.eq(r4.new,#0)) jump:nt .L1e400ce4
}
{
  r1:0 = combine(##507576347,#3)
  r2 = #538
}
{
  r3 = ##507576506
  callr r4
}
.L1e400ce4:
{
  call ext_1e4007a0
  r0 = ##507576506
}
.L1e400cf0:
{
  call ext_1e400490
  r17 = #0
  r1:0 = combine(#0,r17)
}
{
  jump .L1e400dac
}
.L1e400cfc:
{
  call ext_1e4049b0
  r1:0 = combine(#2,r17)
}
{
  if (!p0.new) r1:0 = combine(r17,r17)
  p0 = cmp.eq(r0,#-1)
  if (p0.new) jump:nt .L1e400cf0
}
{
  call ext_1e401448
}
{
  r2 = memw(r17+#12)
  if (cmp.eq(r2.new,#0)) jump:nt .L1e400d20
}
{
  r2 = memw(r17+#16)
  if (cmp.eq(r2.new,#0)) jump:nt .L1e400da0
}
.L1e400d20:
{
  p0 = cmp.eq(r19,#0)
}
{
  r2 = p0
  if (!p0) jump:nt .L1e400d44
  memw(r29+#12) = r2.new
}
{
  r2 = #558
  r3 = ##507576528
  memw(r29+#0) = r18
}
{
  call ext_1e400e20
}
{
  jump .L1e400d5c
}
.L1e400d44:
{
  r1:0 = combine(##507576347,#2)
  r2 = #558
}
{
  call ext_1e405d78
  r3 = ##507576528
  memw(r29+#0) = r18
}
.L1e400d5c:
{
  call ext_1e4037e0
  r1:0 = combine(#2,r17)
}
{
  p0 = cmp.eq(r0,#-1)
  if (p0.new) jump:nt .L1e400cf0
  if (!p0.new) r2 = memw(r29+#12)
}
{
  p0 = r2
  if (!p0.new) jump:t .L1e400d88
}
{
  r2 = #567
  r3 = ##507576528
  memw(r29+#0) = r18
}
{
  call ext_1e400e20
}
{
  jump .L1e400da0
}
.L1e400d88:
{
  r1:0 = combine(##507576347,#2)
  r2 = #567
}
{
  call ext_1e405d78
  r3 = ##507576528
  memw(r29+#0) = r18
}
.L1e400da0:
{
  call ext_1e400e60
  r0 = r17
}
.L1e400da8:
{
  memw(r17+#24) += #1
}
.L1e400dac:
{
  call ext_1e400488
  r2 = #0
  memb(##507617064) = r2.new
}
{
  call ext_1e4065d8
  r0 = ##507612016
}
{
  p0 = !tstbit(r16,#2)
  if (p0.new) jump:nt .L1e400e10
}
{
  p0 = cmp.eq(r17,#0)
  if (p0.new) jump:nt .L1e400e10
}
{
  r16 = memw(r17+#48)
  if (cmp.eq(r16.new,#0)) jump:nt .L1e400e10
}
{
  p0 = cmp.eq(r19,#0)
  if (!p0.new) jump:t .L1e400df4
}
{
  r2 = #598
  r3 = ##507576552
  memw(r29+#0) = r16
}
{
  call ext_1e400e20
}
{
  jump .L1e400e0c
}
.L1e400df4:
{
  r1:0 = combine(##507576347,#2)
  r2 = #598
}
{
  call ext_1e405d78
  r3 = ##507576552
  memw(r29+#0) = r16
}
.L1e400e0c:
{
  callr r16
}
.L1e400e10:
{
  r0 = r17
  r17:16 = memd(r29+#48)
  r19:18 = memd(r29+#40)
}
{
  r23:22 = memd(r29+#24)
  dealloc_return
}
