// SPDX-License-Identifier: GPL-3.0-only
// Terminal controller-global status path, original function epilogue.
// ADSP file [0x1167c,0x11698); physical [0x8be0e67c,0x8be0e698).
.text
.globl adsp_controller_global_complete
adsp_controller_global_complete:
{ r1=#1; immext(#0xf0025540); r2=memw(gp+##0xf0025560); r17:16=memd(sp+#8) }
{ r0=lsr(r2,#12); r19:18=memd(r29+#0); deallocframe }
{ jump final_controller_handoff }
.set final_controller_handoff,adsp_controller_global_complete+(0x8be1e910-0x8be0e67c)
