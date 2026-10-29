// SPDX-License-Identifier: GPL-3.0-only
// [ADSP file 0x39464,0x394a0): counter decrement and adjacent indexed slot copy.
// The return at file 0x3948c shares a Hexagon packet with the copy
// operation beginning at 0x39490. Assemble them together so the original
// parse-marker bits and .new register dependency are preserved.
// This 60-byte source supersedes the 16-byte standalone copy for emitted
// coverage, without counting the overlap twice.
.text
.globl adsp_late_controller_atomic_decrement
adsp_late_controller_atomic_decrement:
{
  immext(#4029459456)
  r3 = memw(r2=##4029459456)
  if (cmp.eq(r3.new,#0)) jump:nt no_counter
}
{ r4 = #-1 }
retry:
{ r3 = memw_locked(r2) }
{ r3 = add(r3,r4) }
{ memw_locked(r2,p0) = r3 }
{ if (!p0) jump:nt retry }
{
  if (!p0.new) r3 = #1
  p0 = cmp.gt(r3,#-1)
  if (!p0.new) jump:nt retry_add
}
// The exact cross-boundary packet -- r31 return, extended indexed load and
// dependent caller write -- is one valid Hexagon packet of four words.
{ jumpr lr; immext(#4029459456); r2 = memw(r3<<#2+##4029459464); memw(r0+#0) = r2.new }
{ jumpr lr }
.set no_counter, adsp_late_controller_atomic_decrement+0x38
.set retry_add, adsp_late_controller_atomic_decrement+0x3c
