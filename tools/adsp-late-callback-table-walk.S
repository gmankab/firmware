// SPDX-License-Identifier: GPL-3.0-only
// Iterate a null-terminated table of controller callback entries. The
// external caller supplies both callback base and context, and the routine
// performs indirect calls with the original QuRT frame convention.
.text
.globl adsp_late_callback_table_walk
adsp_late_callback_table_walk:
{ memd(r29+#-16) = r17:16; allocframe(#8) }
{ r17 = add(r3,#4); r16 = r0; r2 = memw(r3+#0); if (cmp.eq(r2.new,#0)) jump:nt completed }
loop_callbacks:
{ r0 = r16; callr r2 }
{ r2 = memw(r17++#4); if (!cmp.eq(r2.new,#0)) jump:t loop_callbacks }
completed:
{ r17:16 = memd(r29+#0); dealloc_return }
