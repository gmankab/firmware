// ADSP Hexagon V66 [file 0x3aa80,0x3ab00): initialize or reuse a
// controller configuration block, bounded to its platform resource slots.
// External calls preserve the source instruction-level relocations.
.text
.globl adsp_late_configure_session
adsp_late_configure_session:
{ r17 = r0; memd(r29+#-16) = r17:16; allocframe(#8) }
{
  immext(#0xf02cb500)
  r0 = ##-265496344
  immext(#0xf01f1000)
  r16 = memw(##4029460320)
}
{ call platform_begin_config }
{ p0 = cmp.eq(r16,#0); if (p0.new) jump:nt allocate_session }
scan_existing:
{ call platform_copy_slot; r1 = add(r16,#8); r2 = #32; r0 = r17 }
{
  immext(#0xf02cb500)
  if (p0.new) r0 = ##-265496344
  p0 = cmp.eq(r0,#0); if (p0.new) jump:nt retry_fallback
}
{ r16 = memw(r16+#0); if (!cmp.eq(r16.new,#0)) jump:t scan_existing }
allocate_session:
{ call platform_allocate }
{ r2 = #32; r1 = r17; r16 = r0 }
{ call platform_copy_slot_secondary; r0 = add(r16,#8) }
{
  call platform_commit
  immext(#0xf01f1000)
  r1:0 = combine(r16,##-265506976)
  memw(r16+#40) = #0
}
{ call platform_finalize; immext(#0xf02cb500); r0 = ##-265496344 }
{ jump completion }
retry_fallback:
{ call platform_finalize }
completion:
{ r0 = r16; r17:16 = memd(r29+#0); dealloc_return }
.set platform_begin_config, adsp_late_configure_session+0xef7d8
.set platform_copy_slot, adsp_late_configure_session-0x14f0
.set platform_allocate, adsp_late_configure_session+0x4240
.set platform_copy_slot_secondary, adsp_late_configure_session-0x14c0
.set platform_commit, adsp_late_configure_session+0xebb88
.set platform_finalize, adsp_late_configure_session+0xef920
