// SPDX-License-Identifier: GPL-3.0-only
// Independent Hexagon V60 session state continuation, file [0x4477fc,0x4478a4).
// The whole original routine begins at file 0x447724. All external targets
// are expressed relative to that base rather than as unknown OEM code.
.text
.globl modem_late_aux_large_controller_phase2
modem_late_aux_large_controller_phase2:
.set session_base, modem_late_aux_large_controller_phase2 - 0xd8
.set diagnostic_console, session_base - 0xf84
.set controller_log_cb, session_base + 0x136c
.set fallback_log, session_base + 0x4654
.set check_session, session_base + 0x13ac
.set shared_error_exit, session_base + 0x37c
.set pending_registry_exit, session_base + 0x388
.set state_ready, session_base + 0x178
.set state_after_callback, session_base + 0x134
.set no_callback_log, session_base + 0x110
.set report_log_join, session_base + 0x178
.set skip_extra_log, session_base + 0x168
{ r2 = ##507534712 }
{
  p0 = cmp.eq(r2,#0)
  r2 = memw(r24+#0)
}
{
  r3 = p0
  if (!p0) jump:nt no_callback_log
  memw(r29+#84) = r3.new
}
{
  r2 = #266
  r3 = ##507576933
  memw(r29+#8) = r2
}
{
  call controller_log_cb
  memw(r29+#4) = r21
  memw(r29+#0) = #0
}
{ jump state_after_callback }
no_callback_log:
{
  r1:0 = combine(##507576841,#2)
  r2 = #266
  memw(r29+#8) = r2
}
{
  r3 = ##507576933
  memw(r29+#4) = r21
  memw(r29+#0) = #0
}
{ call fallback_log }
state_after_callback:
{
  call check_session
  r1 = add(r29,#92)
  r0 = r21
}
{
  p0 = cmp.eq(r0,#0)
  if (!p0.new) jump:t shared_error_exit
  if (p0.new) r2 = memw(r29+#84)
}
{
  p0 = r2
  if (!p0.new) r3 = ##507576966
  if (!p0.new) jump:t skip_extra_log
}
{
  call controller_log_cb
  r2 = #269
  r3 = ##507576966
}
{ jump report_log_join }
skip_extra_log:
{
  r1:0 = combine(##507576841,#2)
  r2 = #269
}
{ call fallback_log }
report_log_join:
{
  r6 = memw(r16+#8)
  if (cmp.eq(r6.new,#0)) jump:nt pending_registry_exit
}
