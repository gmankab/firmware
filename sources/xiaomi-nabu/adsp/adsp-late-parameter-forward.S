// ADSP V66 [file 0x3bc08,0x3bc1c): forward a parameter word to a
// platform routine with two arguments in Hexagon paired registers.
.text
.globl adsp_late_parameter_forward
adsp_late_parameter_forward:
{ allocframe(#0) }
{ r2 = memw(r0+#0) }
{ call platform_forward; r1:0 = combine(r2,r1) }
{ dealloc_return }
.set platform_forward, adsp_late_parameter_forward+0x3aa0
