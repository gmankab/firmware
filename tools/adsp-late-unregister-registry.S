// ADSP late RX callback registry deactivation helper [file 0x391a0,0x391e0)
.text
.globl adsp_late_unregister_registry
adsp_late_unregister_registry:
{
  r2 = memw(##4029459456)
  allocframe(#8)
}
{
  p0 = cmp.eq(r2,#0)
  if (!p0.new) jump:nt finish
  memd(r29+#0) = r17:16
}
{
  r2 = memw(r16=##4028567588)
  if (cmp.eq(r2.new,#0)) jump:nt finish
}
{
  r2 = memw(##4028567572)
  if (cmp.eq(r2.new,#0)) jump:nt skip_callback
}
{
  r1:0 = combine(#4,r16)
  callr r2
}
skip_callback:
{ r2 = #0; memw(r16+#0) = r2.new }
finish:
{ r17:16 = memd(r29+#0); dealloc_return }
