// SPDX-License-Identifier: GPL-3.0-only
// Source-level Hexagon V60 auxiliary runtime initialization, file
// [0x446a68,0x446bdc),
// VMA [0x1e400a68,0x1e400bdc), independently assembled.
// Allocate a controller context, acquire its resource lock, initialize
// fixed MMIO-facing fields, dispatch optional registration callbacks,
// write packed output-state defaults and release temporary resources.
// Packet boundaries correspond to the actual program; immext is assembled
// from symbolic absolute operand constants, never from vendor opcode bytes.
.text
.globl modem_late_aux_runtime_init
modem_late_aux_runtime_init:
.set ext_1e400488, modem_late_aux_runtime_init + (-1504)
.set ext_1e400804, modem_late_aux_runtime_init + (-612)
.set ext_1e401234, modem_late_aux_runtime_init + (1996)
.set ext_1e404440, modem_late_aux_runtime_init + (14808)
.set ext_1e4048e8, modem_late_aux_runtime_init + (16000)
.set ext_1e404f4c, modem_late_aux_runtime_init + (17636)
.set ext_1e405370, modem_late_aux_runtime_init + (18696)
.set ext_1e405b98, modem_late_aux_runtime_init + (20784)
.set ext_1e406514, modem_late_aux_runtime_init + (23212)
{
  r17:16 = combine(r0,r1)
  memd(r29+#-16) = r17:16
  allocframe(#24)
}
{
  r19:18 = combine(r3,r2)
  memd(r29+#8) = r19:18
  memd(r29+#0) = r21:20
}
{
  r5 = memub(##507612024)
}
{
  p0 = r5
  if (p0.new) r18 = #1
  if (p0.new) jump:t .L1e400bc8
}
{
  r0 = ##507612016
  r20 = #1
}
{
  call ext_1e406514
  memb(##507612024) = r20
}
{
  r0 = r18
  r2 = memw(##507588608)
}
{
  call ext_1e400804
  memw(##507617212) = r2
}
{
  r18 = #0
  p0 = cmp.eq(r0,#1)
  if (!p0.new) jump:t .L1e400bc8
}
{
  r2 = ##507617184
  r3 = ##507617216
}
{
  r6 = sub(#-1,r19)
  r0 = ##507584512
  r18 = #0
}
{
  r4 = ##507576461
  r5 = #15
}
{
  r0 = ##507617216
  memw(r3+#44) = r0
  memw(r3+#4) = r4
}
{
  memw(r3+#52) = r5
  memw(r3+#28) = r2
}
{
  memw(r2+#4) = r19
  memw(r2+#12) = r6
}
{
  r2 = #3
  memw(r2+#0) = r18
  memw(r3+#32) = r20
}
{
  r3 = add(r3,#136)
  memw(r3+#40) = r18
  memw(r19=##507617072) = r3
}
{
  call ext_1e404f4c
  memb(r3+#0) |= r2
}
{
  call ext_1e405b98
  r0 = ##507617216
}
{
  r0 = ##507612020
  r1 = ##507576840
}
{
  call ext_1e405370
}
{
  r2 = ##507511944
  memw(##507617060) = r2.new
}
{
  r2 = #4096
  memb(##507617064) = r18
  r0 = memw(r19+#0)
}
{
  memw(##507617080) = r2
  memw(##507617068) = r0
}
{
  call ext_1e401234
  memw(##507617076) = r0
}
{
  r0 = ##507588612
  r1 = memw(r19+#0)
}
{
  r2 = add(r1,#136)
  memw(r1+#20) += #1
}
{
  call ext_1e404440
  memb(r2+#0) = setbit(#6)
}
{
  r2 = #16
  memb(##507617096) = r2.new
}
{
  r0 = ##65521
  memh(##507617098) = r0.new
}
{
  call ext_1e400488
  r2 = memw(r19+#0)
  memw(##507617068) = r2.new
}
{
  call ext_1e4048e8
  r1:0 = combine(r16,r17)
}
{
  p0 = cmp.eq(r0,#1)
  if (p0.new) r18 = #1
}
.L1e400bc8:
{
  r0 = r18
  r17:16 = memd(r29+#16)
  r19:18 = memd(r29+#8)
}
{
  r21:20 = memd(r29+#0)
  dealloc_return
}
{
  nop
}
{
  nop
}
