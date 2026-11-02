// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V60 controller tag validation and diagnostics, [0x447b5c,0x447bc4).
.text
.globl modem_late_aux_large_controller_phase10
modem_late_aux_large_controller_phase10:
.set session_base, modem_late_aux_large_controller_phase10 - 0x438
.set skip_pending, session_base + 0x5fc
.set callback_fallback, session_base + 0x4654
.set diagnostic_console, session_base + 0x136c
.set over_limit, session_base + 0x490
.set severe_warning, session_base + 0x550
.set continue_processing, session_base + 0x568
{ jump skip_pending }
{
  r1:0 = combine(##507576841,#2)
  r2 = #316
}
{
  call callback_fallback
  r3 = ##507577202
}
{
  r3 = add(r27,r23)
  r2 = memw(r24+#0)
  if (!cmp.gt(r2.new,r3)) jump:t over_limit
}
{ r2 = memw(r29+#84) }
{
  p0 = r2
  if (!p0.new) r3 = ##507577221
  if (!p0.new) jump:t severe_warning
}
{
  call diagnostic_console
  r2 = #318
  r3 = ##507577221
}
{
  r22 = add(r21,r23)
  r27 = r18
  jump continue_processing
}
over_limit:
{
  r4 = r26
  r3 = sub(#0,r2)
  r26 = r24
  memw(r29+#60) = r4.new
}
