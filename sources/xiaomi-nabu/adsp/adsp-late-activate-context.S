// ADSP v66 later executable [file 0x3ab00, 0x3ab28): activate
// a codec context, store mode 10, and notify the platform once enabled.
.text
.globl adsp_late_activate_context
adsp_late_activate_context:
{ r16 = r0; memd(r29+#-16) = r17:16; allocframe(#8) }
{
  call platform_late_context
  immext(#0xf01f1000)
  r1:0 = combine(r16,##-265506976)
  memw(r16+#4) = #10
}
{ call platform_notify; r0 = r16 }
{ r0 = #0; r17:16 = memd(r29+#0); dealloc_return }
.set platform_late_context, adsp_late_activate_context+0xed230
.set platform_notify, adsp_late_activate_context+0x41cc
