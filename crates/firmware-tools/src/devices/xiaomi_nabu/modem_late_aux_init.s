// SPDX-License-Identifier: GPL-3.0-only
// Auxiliary executable's initialization callback, reconstructed from
// Hexagon v60 semantics, file [0x446200,0x446254).
.text
.globl modem_late_aux_init
modem_late_aux_init:
{ allocframe(#8) }
{ memw(r29+#0) = r27 }
{ r0.h = #7745 }
{ r0.l = #7584 }
{ r1 = memw(r0+#0) }
{ p0 = cmp.eq(r1,#0); if (p0.new) jump:nt .Lskip_system_init }
{ r2 = #0 }
{ r3 = #0; r1 = #0 }
{ r28.h = #7744 }
{ r28.l = #40684 }
{ callr r28 }
.Lskip_system_init:
{ r27.h = #7745 }
{ r27.l = #13080 }
.Lconstructor_loop:
{ r27 = add(r27,#-4) }
{ r0 = memw(r27+#0) }
{ p0 = cmp.eq(r0,#0); if (p0.new) jump:nt .Lconstructor_done }
{ callr r0 }
{ jump .Lconstructor_loop }
.Lconstructor_done:
{ r27 = memw(r29+#0); deallocframe }
{ jumpr r31 }
