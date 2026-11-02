// ADSP V66 [file 0x3bbe4,0x3bbf8): multiply a signed size and pass it
// to a bounded platform calculator with an eight-byte frame.
.text
.globl adsp_late_guarded_multiply
adsp_late_guarded_multiply:
{ r3 = mpyi(r3,r3); allocframe(#8) }
{ call platform_calculate; memw(r29+#0) = #0 }
{ dealloc_return }
.set platform_calculate, adsp_late_guarded_multiply+0x1f0
