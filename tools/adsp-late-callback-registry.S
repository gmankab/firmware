// Source-authored 128-byte registration routine [file 0x39120,0x391a0).
// Traverse up to 200 callback slots, calling installed descriptors and
// compacting the high-address control table.
.text
.globl adsp_late_callback_registry
adsp_late_callback_registry:
{ memd(r29+#-16) = r17:16; allocframe(#16) }
{ memd(r29+#0) = r19:18 }
{
  r2 = memw(r16=##4029459456)
  if (!cmp.eq(r2.new,#0)) jump:t done
}
{ r17 = ##4029459464; r19 = #1 }
lookup:
{
  r2 = memw(r19<<#2+##4028567568)
  if (cmp.eq(r2.new,#0)) jump:nt no_handler
}
{
  r0 = ##-265507832
  callr r2
  r1 = #800
}
{ r2 = memw(r16+#0) }
{ p0 = cmp.gtu(r2,#199); if (!p0.new) jump:t count_limit }
{ jump done }
no_handler:
{ r2 = #0 }
count_limit:
{ r3 = addasl(r17,r2,#2); r4 = sub(#200,r2) }
{ loop0(copy_loop,r4) }
copy_loop:
{ r4 = memw(r3+#0); if (cmp.eq(r4.new,#0)) jump:nt skip_restore }
{
  r2 = add(r2,#1)
  r3 = add(r3,#4)
  memw(r16+#0) = r2.new
}:endloop0
{ jump done }
skip_restore:
{ r19 = add(r19,#1); p0 = cmp.eq(r2,#0); if (p0.new) jump:t lookup }
done:
{ r17:16 = memd(r29+#8); r19:18 = memd(r29+#0) }
{ dealloc_return }
