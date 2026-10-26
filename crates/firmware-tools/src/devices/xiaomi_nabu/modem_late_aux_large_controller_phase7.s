// SPDX-License-Identifier: GPL-3.0-only
// Controller failure reporting/cleanup path, file [0x447a24,0x447aac).
// This returns to a later continuation inside the same session function.
.text
.globl modem_late_aux_large_controller_phase7
modem_late_aux_large_controller_phase7:
.set controller_base, modem_late_aux_large_controller_phase7 - 0x300
.set missing_session_log, controller_base + 0x3c0
.set handler_failed, controller_base + 0x350
.set callback_failed, controller_base + 0x340
.set cleanup, controller_base + 0x37c
.set last_state, controller_base + 0x1338
.set diagnostic_console, controller_base - 0xf84
.set release_resources, controller_base - 0x274
{
  p0 = cmp.eq(r23,#-1)
  r1:0 = combine(r21,r26)
  r4 = r22
  r2 = memw(r24+#0)
}
{
  r5 = add(r29,#88)
  if (p0) r23 = #0
}
{
  r3 = r23
  callr r6
}
{
  p0 = cmp.eq(r0,#0)
  if (p0.new) jump:nt missing_session_log
}
{
  r4 = memw(r19+#0)
  if (cmp.eq(r4.new,#0)) jump:nt callback_failed
}
{
  r1:0 = combine(##507576841,#3)
  r2 = #300
}
{
  r3 = ##507577126
  callr r4
}
callback_failed:
{
  call diagnostic_console
  r0 = ##507577126
}
{ jump cleanup }
handler_failed:
{
  r4 = memw(r19+#0)
  if (cmp.eq(r4.new,#0)) jump:nt .Lskip_secondary_log
}
{
  r1:0 = combine(##507576841,#3)
  r2 = #293
}
{
  r3 = ##507577098
  callr r4
}
.Lskip_secondary_log:
{
  call diagnostic_console
  r0 = ##507577098
}
cleanup:
{
  call release_resources
  r17 = #0
  r0 = r17
}
{ jump last_state }
