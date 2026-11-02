// SPDX-License-Identifier: GPL-3.0-only
// Hexagon v66 [file 0x3e9e0,0x3ea4c): bounded table index allocation,
// packed descriptor publication and failure handling.
.text
.globl adsp_late_indexed_state_init
adsp_late_indexed_state_init:
{ r16 = r0; memd(r29+#-16) = r17:16; allocframe(#16) }
{ p0 = cmp.eq(r16,#0); if (!p0.new) jump:nt nonnull; memd(r29+#0) = r19:18 }
{ r19 = #5; r2 = #5; if (cmp.gtu(r2.new,r1)) jump:t clamp_done }
nonnull:
{ r19 = r1; p0 = cmp.eq(r1,#0); if (p0.new) jump:nt restore }
clamp_done:
{ r18 = asl(r19,#2) }
{ call allocate_index; r0 = r18 }
{ r17 = r0; if (cmp.eq(r17.new,#0)) jump:nt failure }
{ call initialize_index; r1 = add(r17,r18); r0 = r17 }
{ r2 = add(r19,#-1); immext(#4030006528); memw(r16<<#3+##4030006552) = r17 }
{ immext(#4030006528); memw(r16<<#3+##4030006556) = r2 }
restore:
{ r17:16 = memd(r29+#8); r19:18 = memd(r29+#0) }
{ dealloc_return }
failure:
{ call log_failed; immext(#4028152064); r1:0 = combine(#100,##4028152095) }
.set allocate_index, adsp_late_indexed_state_init-0x3e74
.set initialize_index, adsp_late_indexed_state_init-0x1e3f0
.set log_failed, adsp_late_indexed_state_init-0xec
