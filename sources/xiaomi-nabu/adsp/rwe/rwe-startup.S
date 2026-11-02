// SPDX-License-Identifier: GPL-3.0-only
// The ADSP RWE PT_LOAD begins at file 0x6000, but the first 0x30 bytes are
// boot metadata, not Hexagon instructions. This separate executable entry
// starts at file 0x6030 / physical 0x8be03030.
.text
.globl adsp_agent20_rwe_startup
adsp_agent20_rwe_startup:
{ allocframe(#8) }
{
 immext(#0xc0c0)
 call initial_platform_start
 memd(sp+#0)=r1:0
}
{ p0 = cmp.eq(r0,#0); if (!p0.new) jump:nt after_syscfg }
{ nop } // v66 r1=SYSCFG
{ r1 = clrbit(r1,#4) }
{ nop } // v66 SYSCFG=r1
after_syscfg:
{ nop } // v66 r0=SGP0
{ r0 = memw(r0+#0) }
{
 immext(#0x100)
 r0 = memw(r0+##0x100)
 if (cmp.eq(r0.new,#0)) jump:t fail
}
{
 immext(#0)
 r0 = memw(r0+##0x10)
 if (cmp.eq(r0.new,#0)) jump:t fail
}
{ r1:0 = memd(sp+#0) }
{ deallocframe }
{ r0 = r18 }
{ jump continue_setup }
fail:
{ r1:0 = memd(sp+#0) }
{ deallocframe }
{ jump external_failure }
continue_setup:
.set initial_platform_start, adsp_agent20_rwe_startup+(0xf118-0x3030)
.set external_failure, adsp_agent20_rwe_startup+(0xddc8-0x3030)
