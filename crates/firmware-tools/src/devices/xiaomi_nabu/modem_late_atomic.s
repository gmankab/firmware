// SPDX-License-Identifier: GPL-3.0-only
// Atomic compare/exchange helper from Hexagon v60 modem later executable.
// The source describes the original lock/cmp/conditional-store algorithm;
// LLVM-MC must encode instruction packet boundaries, not OEM code arrays.
.text
.globl modem_late_atomic_exchange
modem_late_atomic_exchange:
{
  r3 = memw_locked(r0)
}
{
  p0 = cmp.eq(r3,r1)
  if (!p0.new) jump:nt .Latomic_failed
}
{
  memw_locked(r0,p0) = r2
}
{ r3 = p0 }
{
  if (!p0.new) r3 = add(r2,#0)
  p0 = cmp.eq(r3,#0)
  if (p0.new) jump:t modem_late_atomic_exchange
}
{
  r0 = r3
  jumpr r31
}
.Latomic_failed:
{ memw_locked(r0,p0) = r3 }
{ r2 = p0 }
{
  r0 = r3
  jumpr r31
}
// Architecture trap sled between two independent callable functions.
{ brkpt }
{ brkpt }
.globl modem_late_flag_and_return
modem_late_flag_and_return:
{
  r2 = #1
  jumpr r31
  memb(##3249533708) = r2.new
}
