// SPDX-License-Identifier: GPL-3.0-only
.text
.globl controller_loop_predicate_load
controller_loop_predicate_load:
{
 p0 = cmp.eq(r4,#3); r5 = memw(r3+#0)
}
