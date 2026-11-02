// SPDX-License-Identifier: GPL-3.0-only
// Independent leaf helper at ADSP file [0x11898,0x118b8), physical
// [0x8be0e898,0x8be0e8b8). One SGP0 read uses checked V66 encoding.
.text
.globl adsp_sram_leaf_publish
adsp_sram_leaf_publish:
{ call read_active_controller; allocframe(#0) }
{ call update_global_state }
{ immext(#0xf020a280); r2=memw(gp+##0xf020a29c) }
{ nop } // r10=SGP0
{ memw(r10+#0)=r2 }
{ dealloc_return }
.set read_active_controller,adsp_sram_leaf_publish+(0x8be1e9f0-0x8be0e898)
.set update_global_state,adsp_sram_leaf_publish+(0x8be0dc40-0x8be0e898)
