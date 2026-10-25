// SPDX-License-Identifier: GPL-3.0-only
// Source-level Hexagon V60 controller input validation, file
// [0x446804,0x446a68),
// VMA [0x1e400804,0x1e400a68), independently assembled.
// The routine validates a caller-selected descriptor and its mandatory
// header fields, copies or substitutes optional fields from controller
// defaults, then dispatches the resulting 84-byte record to the runtime.
// Packet boundaries correspond to the actual program; immext is assembled
// from symbolic absolute operand constants, never from vendor opcode bytes.
.text
.globl modem_late_aux_input_validator
modem_late_aux_input_validator:
.set ext_1e4007a0, modem_late_aux_input_validator + (-100)
.set ext_1e405ddc, modem_late_aux_input_validator + (21976)
.set ext_1e4095b0, modem_late_aux_input_validator + (36268)
{
  r16 = r0
  r2 = #80
  memd(r29+#-16) = r17:16
  allocframe(#104)
}
{
  r19 = add(r29,#4)
  memd(r29+#88) = r19:18
}
{
  r17 = add(r19,#4)
}
{
  call ext_1e4095b0
  r1:0 = combine(#0,r17)
}
{
  p0 = cmp.eq(r16,#0)
  memw(r29+#4) = #84
  memw(r19+#20) = ##507549520
}
{
  memw(r19+#24) = ##507548232
  memw(r19+#28) = ##507548240
}
{
  memw(r19+#32) = ##507549752
  memw(r19+#36) = ##507548248
}
{
  memw(r19+#40) = ##507547696
  memw(r19+#44) = ##507547456
}
{
  memw(r19+#48) = ##507549496
  memw(r19+#52) = ##507549504
}
{
  if (p0) jump:nt .L1e400a48
  if (!p0) r2 = memw(r16+#0)
  memw(r19+#56) = ##507549512
}
{
  p0 = cmp.gt(r2,#19)
  if (!p0.new) jump:t .L1e400a48
  r3 = memw(r16+#4)
  memw(r17+#0) = r3.new
}
{
  r4 = memw(r16+#8)
}
{
  p0 = cmp.eq(r4,#0)
  if (!p0.new) jump:t .L1e4008a8
  r3 = memw(r16+#12)
}
{
  p0 = cmp.eq(r3,#0)
  if (!p0.new) jump:t .L1e4008b4
}
{
  r3 = memw(r16+#16)
  if (cmp.eq(r3.new,#0)) jump:nt .L1e4008f0
}
{
  jump .L1e4008b4
}
.L1e4008a8:
{
  p0 = cmp.eq(r3,#0)
  if (p0.new) jump:nt .L1e4008b4
}
{
  r5 = memw(r16+#16)
  if (!cmp.eq(r5.new,#0)) jump:t .L1e4008e8
}
.L1e4008b4:
{
  r4 = memw(##507617104)
  if (cmp.eq(r4.new,#0)) jump:nt .L1e4008d8
}
{
  r1:0 = combine(##507576347,#3)
  r2 = #420
}
{
  r3 = ##507576375
  callr r4
}
.L1e4008d8:
{
  call ext_1e4007a0
  r0 = ##507576375
}
{
  r0 = #0
  jump .L1e400a60
}
.L1e4008e8:
{
  memw(r29+#12) = r4
  memw(r29+#16) = r3
}
{
  memw(r29+#20) = r5
}
.L1e4008f0:
{
  p0 = cmp.gt(r2,#57)
  if (!p0.new) jump:t .L1e400994
  if (p0.new) r3 = memw(r16+#60)
  if (p0.new) r2 = memw(r16+#64)
}
{
  p0 = cmp.eq(r3,#0)
  if (!p0.new) jump:t .L1e40092c
}
{
  p0 = cmp.eq(r2,#0)
  if (!p0.new) jump:t .L1e400950
}
{
  r2 = memw(r16+#68)
  if (!cmp.eq(r2.new,#0)) jump:t .L1e400950
}
{
  r2 = memw(r16+#72)
  if (!cmp.eq(r2.new,#0)) jump:t .L1e400950
}
{
  r2 = memw(r16+#76)
  if (!cmp.eq(r2.new,#0)) jump:t .L1e400950
}
{
  r2 = memw(r16+#80)
  if (cmp.eq(r2.new,#0)) jump:nt .L1e40099c
}
{
  jump .L1e400950
}
.L1e40092c:
{
  p0 = cmp.eq(r2,#0)
  if (p0.new) jump:nt .L1e400950
}
{
  r4 = memw(r16+#68)
  if (cmp.eq(r4.new,#0)) jump:nt .L1e400950
}
{
  r5 = memw(r16+#72)
  if (cmp.eq(r5.new,#0)) jump:nt .L1e400950
}
{
  r6 = memw(r16+#76)
  if (cmp.eq(r6.new,#0)) jump:nt .L1e400950
}
{
  r7 = memw(r16+#80)
  if (!cmp.eq(r7.new,#0)) jump:t .L1e400984
}
.L1e400950:
{
  r4 = memw(##507617104)
  if (cmp.eq(r4.new,#0)) jump:nt .L1e400974
}
{
  r1:0 = combine(##507576347,#3)
  r2 = #434
}
{
  r3 = ##507576419
  callr r4
}
.L1e400974:
{
  call ext_1e4007a0
  r0 = ##507576419
}
{
  r0 = #0
  jump .L1e400a60
}
.L1e400984:
{
  memw(r29+#64) = r3
  memw(r29+#68) = r2
}
{
  memw(r29+#72) = r4
  memw(r29+#76) = r5
}
{
  jump .L1e40099c
  memw(r29+#80) = r6
  memw(r29+#84) = r7
}
.L1e400994:
{
  p0 = cmp.gt(r2,#39)
  if (!p0.new) jump:t .L1e400a48
}
.L1e40099c:
{
  r2 = memw(r16+#20)
  r3 = memw(r16+#24)
}
{
  p1 = cmp.eq(r2,#0)
  p0 = cmp.eq(r3,#0)
  r4 = memw(r16+#28)
  r5 = memw(r16+#32)
}
{
  p2 = cmp.eq(r4,#0)
  r6 = mux(p1,##507549520,r2)
  r2 = memw(r16+#36)
}
{
  p1 = cmp.eq(r5,#0)
  r8 = mux(p0,##507548232,r3)
  memw(r29+#24) = r6
}
{
  r3 = mux(p1,##507549752,r5)
  p0 = cmp.eq(r2,#0)
  r6 = memw(r16+#40)
}
{
  r7 = mux(p2,##507548240,r4)
  p1 = cmp.eq(r6,#0)
  memw(r29+#28) = r8
}
{
  r4 = mux(p0,##507548248,r2)
  r5 = memw(r16+#44)
  memw(r29+#32) = r7
}
{
  p0 = cmp.eq(r5,#0)
  r7 = memw(r16+#48)
}
{
  r2 = mux(p1,##507547696,r6)
  p1 = cmp.eq(r7,#0)
  memw(r29+#36) = r3
}
{
  r3 = mux(p0,##507547456,r5)
  r6 = memw(r16+#52)
  memw(r29+#40) = r4
}
{
  r4 = mux(p1,##507549496,r7)
  p0 = cmp.eq(r6,#0)
  r8 = memw(r16+#56)
}
{
  p1 = cmp.eq(r8,#0)
  memw(r29+#44) = r2
  memw(r29+#48) = r3
}
{
  r2 = mux(p0,##507549504,r6)
  memw(r29+#52) = r4
}
{
  r3 = mux(p1,##507549512,r8)
  memw(r29+#56) = r2
}
{
  memw(r29+#60) = r3
}
.L1e400a48:
{
  r1:0 = combine(#84,##507617100)
  r2 = add(r29,#4)
  r3 = #84
}
{
  call ext_1e405ddc
}
{
  r0 = #1
}
.L1e400a60:
{
  r17:16 = memd(r29+#96)
  r19:18 = memd(r29+#88)
}
{
  dealloc_return
}
