// ADSP V66 [file 0x3bbf8,0x3bc08): small stack-backed calculator wrapper.
.text
.globl adsp_late_platform_calculate
adsp_late_platform_calculate:
{ allocframe(#8) }
{ call platform_calculate; memw(r29+#0) = #0 }
{ dealloc_return }
.set platform_calculate, adsp_late_platform_calculate+0x1dc
