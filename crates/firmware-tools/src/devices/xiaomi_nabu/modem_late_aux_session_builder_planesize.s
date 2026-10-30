// SPDX-License-Identifier: GPL-3.0-only
// Compute the active codec-plane byte size, allocate its DMA descriptor
// storage through the runtime allocator, and report allocation failures.
.text
.globl modem_late_aux_session_builder_planesize
modem_late_aux_session_builder_planesize:
.set plane_allocated,modem_late_aux_session_builder_planesize+0x48
.set no_handler,modem_late_aux_session_builder_planesize+0x38
.set shared_error_cleanup,modem_late_aux_session_builder_planesize-0xe4
.set report_error,modem_late_aux_session_builder_planesize-0x2944
{
  r22 = asl(r18,#5)
  r2 = memw(r26+#36)
}
{r0 = r22}
{callr r2}
{
  p0 = cmp.eq(r0,#0)
  if (!p0.new) jump:t plane_allocated
  memw(r16+#36) = r0
}
{
  r4 = memw(r26+#0)
  if (cmp.eq(r4.new,#0)) jump:nt no_handler
}
{
  r1:0 = combine(##507576841,#3)
  r2 = #664
}
{
  callr r4
  r3 = ##507577366
  memw(r29+#0) = r22
}
no_handler:
{
  call report_error
  r0 = ##507577366
  memw(r29+#0) = r22
}
{jump shared_error_cleanup}
