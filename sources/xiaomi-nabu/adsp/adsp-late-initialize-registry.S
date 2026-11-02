// ADSP late RX lazy callback-registry initialization, file [0x391e0,0x39230).
.text
.globl adsp_late_initialize_registry
adsp_late_initialize_registry:
{ memd(r29+#-16) = r17:16; allocframe(#16) }
{ r3 = memub(##4029460264); memw(r29+#4) = #0 }
{
  p0 = r3
  if (!p0.new) r3 = #1
  if (!p0.new) memb(##4029460264) = r3.new
}
{
  immext(#0xffffff00)
  if (!p0) call init_registry
  r16 = #0
}
reg_loop:
{
  r16 = add(r16,#1)
  r2 = memw(r16<<#2+##4028567568)
}
{ p0 = cmp.eq(r2,#0); if (p0.new) jump:t reg_loop }
{
  callr r2
  r1 = #4
  r0 = add(r29,#4)
}
{ r0 = memw(r29+#4); if (cmp.eq(r0.new,#0)) jump:t reg_loop }
{ r17:16 = memd(r29+#8); dealloc_return }
.set init_registry, adsp_late_initialize_registry-0xc0
