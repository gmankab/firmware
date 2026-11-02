// ADSP V66 [file 0x3be70,0x3be80): wrapper forwarding two arguments
// with an unchanged stack frame, using the platform's parameter mapper.
.text
.globl adsp_late_parameter_combine
adsp_late_parameter_combine:
{ r2 = r0; allocframe(#0) }
{ call platform_map; r1:0 = combine(r2,r1) }
{ dealloc_return }
.set platform_map, adsp_late_parameter_combine+0x3838
