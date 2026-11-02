// SPDX-License-Identifier: GPL-3.0-only
.text
.globl controller_loop_combine
controller_loop_combine:
{
 p0 = not(p0)
 p1 = cmp.eq(r5,#0)
}
{
 p0 = and(p0,!p1)
 if (p0.new) r2 = add(r2,#1)
}:endloop0
