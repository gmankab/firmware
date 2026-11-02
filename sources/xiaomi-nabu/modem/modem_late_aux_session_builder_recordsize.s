// SPDX-License-Identifier: GPL-3.0-only
// Allocate N fixed 28-byte controller record descriptors and preserve
// the source-defined warning and error path on allocator failure.
.text
.globl modem_late_aux_session_builder_recordsize
modem_late_aux_session_builder_recordsize:
.set records_allocated,modem_late_aux_session_builder_recordsize+0x4c
.set no_handler,modem_late_aux_session_builder_recordsize+0x3c
.set shared_error_cleanup,modem_late_aux_session_builder_recordsize-0x12c
.set report_error,modem_late_aux_session_builder_recordsize-0x298c
{
  r22 = +mpyi(r18,#28)
  memw(r16+#40) = r18
  r2 = memw(r26+#36)
}
{r0 = r22}
{callr r2}
{
  p0 = cmp.eq(r0,#0)
  if (!p0.new) jump:t records_allocated
  memw(r16+#28) = r0
}
{
  r4 = memw(r26+#0)
  if (cmp.eq(r4.new,#0)) jump:nt no_handler
}
{
  r1:0 = combine(##507576841,#3)
  r2 = #668
}
{
  callr r4
  r3 = ##507577401
  memw(r29+#0) = r22
}
no_handler:
{
  call report_error
  r0 = ##507577401
  memw(r29+#0) = r22
}
{jump shared_error_cleanup}
