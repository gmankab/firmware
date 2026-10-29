// SPDX-License-Identifier: GPL-3.0-only
// Controller frame callback, optional platform warning and fallback to the
// record scanner. Preserve Hexagon V60 new-predicate packet membership.
.text
.globl modem_late_aux_large_controller_phase48
modem_late_aux_large_controller_phase48:
.set lookup_record,modem_late_aux_large_controller_phase48+0x72c0
.set callback_failed,modem_late_aux_large_controller_phase48+0x74
.set no_custom_callback,modem_late_aux_large_controller_phase48+0x54
.set platform_error,modem_late_aux_large_controller_phase48-0x21e0
.set previous_scan,modem_late_aux_large_controller_phase48-0xee0
{
  call lookup_record
  r0 = r26
}
{
  r20 = r0
  r2 = memw(r19+#36)
}
{r22 = add(r20,#1)}
{
  r0 = r22
  callr r2
}
{
  if (!p0.new) r2 = add(r22,#0)
  p0 = cmp.eq(r0,#0)
  if (!p0.new) jump:t callback_failed
  memw(r17+#4) = r0
}
{
  r18 = memw(r19+#0)
  if (cmp.eq(r18.new,#0)) jump:nt no_custom_callback
}
{
  call lookup_record
  r0 = r26
}
{
  r2 = add(r0,#1)
  r1:0 = combine(##507576841,#3)
  memw(r29+#0) = r2.new
}
{
  r3 = ##507578132
  callr r18
  r2 = #562
}
no_custom_callback:
{
  call lookup_record
  r0 = r26
}
{
  r2 = add(r0,#1)
  r0 = ##507578132
  memw(r29+#0) = r2.new
}
{call platform_error}
{jump previous_scan}
