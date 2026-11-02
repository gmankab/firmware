// SPDX-License-Identifier: GPL-3.0-only
// Check the global optional tracing switch and select structured firmware
// diagnostic call route before the controller frame/format callback.
.text
.globl modem_late_aux_session_builder_debug
modem_late_aux_session_builder_debug:
.set logging_disabled,modem_late_aux_session_builder_debug+0x2c
.set diagnostics_complete,modem_late_aux_session_builder_debug+0x44
.set factory,modem_late_aux_session_builder_debug-0x4d4
.set alternate_log,modem_late_aux_session_builder_debug+0x2e14
{r2 = ##507534712}
{p0 = cmp.eq(r2,#0)}
{
  r2 = p0
  if (!p0) jump:nt logging_disabled
  memw(r29+#40) = r2.new
}
{
  r2 = #629
  r3 = ##507578199
  memw(r29+#0) = r17
}
{call factory}
{jump diagnostics_complete}
logging_disabled:
{
  r1:0 = combine(##507576841,#2)
  r2 = #629
}
{
  call alternate_log
  r3 = ##507578199
  memw(r29+#0) = r17
}
