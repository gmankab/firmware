// SPDX-License-Identifier: GPL-3.0-only
// ADSP controller purge wrapper [0x13828,0x13844).
.text
.globl adsp_purge_wrapper
adsp_purge_wrapper:
{ allocframe(#0) }
{ nop } // checked R2=SGP0
{ r2=memw(r2+#0) }
{ r2=memw(r2+#28) }
{ r0=extractu(r2,#7,#8); call controller_record_purge }
{ r0=#0; dealloc_return }
.set controller_record_purge,adsp_purge_wrapper+(0x8be13610-0x8be13828)
