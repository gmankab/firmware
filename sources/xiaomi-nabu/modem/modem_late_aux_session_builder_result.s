// SPDX-License-Identifier: GPL-3.0-only
// Compare returned controller state with the selected session record,
// report the diagnostic and invoke the already reconstructed record parser.
.text
.globl modem_late_aux_session_builder_result
modem_late_aux_session_builder_result:
.set log_alternate,modem_late_aux_session_builder_result+0x24
.set diagnostic_done,modem_late_aux_session_builder_result+0x40
.set buffer_factory,modem_late_aux_session_builder_result-0x57c
.set alternate_log,modem_late_aux_session_builder_result+0x2d6c
.set record_guard,modem_late_aux_session_builder_result-0x53c
.set alternate_error,modem_late_aux_session_builder_result-0xc
{
  r2 = memw(r18+#0)
  r3 = memw(r29+#40)
}
{
  p0 = r3
  if (!p0.new) jump:t log_alternate
  if (p0.new) memw(r29+#0) = r17
}
{
  r2 = #633
  r3 = ##507578280
  memw(r29+#4) = r2
}
{call buffer_factory}
{jump diagnostic_done}
log_alternate:
{
  r1:0 = combine(##507576841,#2)
  r2 = #633
  memw(r29+#4) = r2
}
{
  call alternate_log
  r3 = ##507578280
  memw(r29+#0) = r17
}
diagnostic_done:
{call record_guard}
{
  r1 = add(r29,#44)
  r0 = r17
}

{
  p0 = cmp.eq(r0,#0)
  if (!p0.new) jump:t alternate_error
  if (p0.new) r2 = memw(r29+#40)
}
