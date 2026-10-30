// SPDX-License-Identifier: GPL-3.0-only
// Call the session's registered controller translator. A mismatching
// returned handle is diagnosed and released through the original cleanup
// path; a matching handle continues in the next fragment at 0x44900c.
.text
.globl modem_late_aux_session_builder_callback
modem_late_aux_session_builder_callback:
.set validated_handle,modem_late_aux_session_builder_callback+0x64
.set no_handler,modem_late_aux_session_builder_callback+0x40
.set error_report,modem_late_aux_session_builder_callback-0x2808
.set destroy_state,modem_late_aux_session_builder_callback-0x1af8
.set shared_error_exit,modem_late_aux_session_builder_callback+0x7ec
{
  r1:0 = combine(r17,r20)
  r23 = memw(r18+#0)
  r3 = memw(r21+#32)
}
{r2 = r23}
{callr r3}
{
  r22 = r0
  if (cmp.eq(r22.new,r23)) jump:t validated_handle
}
{
  r4 = memw(r26+#0)
  if (cmp.eq(r4.new,#0)) jump:nt no_handler
}
{
  r1:0 = combine(##507576841,#3)
  r2 = memw(r18+#0)
  memw(r29+#4) = r2.new
}
{
  r2 = #632
  r3 = ##507578235
  memw(r29+#0) = r22
}
{callr r4}
no_handler:
{
  r0 = ##507578235
  r2 = memw(r18+#0)
  memw(r29+#4) = r2.new
}
{
  call error_report
  memw(r29+#0) = r22
}
{
  call destroy_state
  r16 = #0
  r0 = r16
}
{jump shared_error_exit}
