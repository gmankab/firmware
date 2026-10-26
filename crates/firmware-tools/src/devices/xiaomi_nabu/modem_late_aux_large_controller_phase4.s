// SPDX-License-Identifier: GPL-3.0-only
// Session-controller callback/error selection, modem [0x44795c,0x4479a8).
// Source uses scalar r22=0 plus a branch; checked V60 duplex folding
// expresses the equivalent original compact architectural A/A packet.
.text
.globl modem_late_aux_large_controller_phase4
modem_late_aux_large_controller_phase4:
.set session_base, modem_late_aux_large_controller_phase4 - 0x238
.set log_callback, session_base + 0x136c
.set error_callback, session_base + 0x4654
.set no_resource, session_base + 0x300
.set diagnostic_else, session_base + 0x268
.set begin_main_call, session_base + 0x284
{
  r22 = #0
  jump no_resource
}
{ r2 = memw(r29+#84) }
{
  p0 = r2
  if (!p0.new) jump:t diagnostic_else
  if (!p0.new) memw(r29+#0) = r25
  if (p0.new) memw(r29+#4) = r23
}
{
  r2 = #288
  r3 = ##507577029
  memw(r29+#8) = r18
}
{
  call log_callback
  memw(r29+#0) = r25
}
{ jump begin_main_call }
diagnostic_else:
{
  r1:0 = combine(##507576841,#2)
  r2 = #288
}
{
  r3 = ##507577029
  memw(r29+#8) = r18
  memw(r29+#4) = r23
}
{ call error_callback }
