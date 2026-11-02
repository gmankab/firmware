// Source-authored Hexagon registry allocation and callback scan, ADSP
// original file [0x39230,0x392c8). The control loop and pointer restoration
// operate on real QuRT/ADSP runtime registers; external calls are symbolic.
.text
.globl adsp_late_registry_alloc
adsp_late_registry_alloc:
{
  p0 = cmp.eq(r0,#-1)
  if (!p0.new) r17 = add(r0,#1)
  memd(r29+#-16) = r17:16
  allocframe(#32)
}
{
  if (p0) jump:nt special_case
  memd(r29+#16) = r19:18
  memd(r29+#8) = r21:20
}
{
  call platform_allocate
  r1:0 = combine(r17,#-1)
}
{ r19 = #1; r16 = r0 }
{ r17 = mpyi(r16,r17) }
retry:
{ r2 = memub(##4029460264); memw(r29+#4) = #0 }
{
  p0 = r2
  if (!p0.new) memb(##4029460264) = r19
}
{
  immext(#0xfffffe80)
  if (!p0) call init_registry
  r20 = ##-266399728
}
next_callback:
{ r2 = memw(r20++#4); if (cmp.eq(r2.new,#0)) jump:t next_callback }
{ callr r2; r1 = #4; r0 = add(r29,#4) }
{ r2 = memw(r29+#4); if (cmp.eq(r2.new,#0)) jump:t next_callback }
{ p0 = cmp.gtu(r2,r17); if (p0.new) jump:t retry }
{ call platform_allocate; r1 = r16; r0 = add(r2,#-1) }
{ jump complete }
special_case:
{ call adsp_late_registry_alloc; r0 = ##65535 }
{ call adsp_late_registry_alloc; r0 = ##65535; r16 = r0 }
{ r0 += asl(r16,#16) }
complete:
{ r17:16 = memd(r29+#24); r19:18 = memd(r29+#16) }
{ r21:20 = memd(r29+#8); dealloc_return }
.set platform_allocate, adsp_late_registry_alloc+0x12acb0
.set init_registry, adsp_late_registry_alloc-0x110
