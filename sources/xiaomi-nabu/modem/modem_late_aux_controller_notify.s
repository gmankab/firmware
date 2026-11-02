// SPDX-License-Identifier: GPL-3.0-only
// Auxiliary Hexagon V60 controller-state notifier, file [0x4467a0,0x4467f0).
// If the latest hardware controller word differs from the current runtime
// word, notify a platform callback and update the tracked controller state.
.text
.globl modem_late_aux_controller_notify
modem_late_aux_controller_notify:
.set controller_callback, modem_late_aux_controller_notify + (0x94e8 - 0x4a8)
{
  r2 = memw(##507588608)
  r3 = memw(##507617212)
}
{
  p0 = !cmp.eq(r3,r2)
  r2 = r0
  if (p0.new) jump:t .Lreturn
}
{
  r1:0 = combine(r2,##507612032)
  r2 = #128
  allocframe(#8)
}
{
  call controller_callback
  r17 = ##507612032
  memd(r29+#0) = r17:16
}
{
  memw(##507617212) = r17
  r17:16 = memd(r29+#0)
}
{ deallocframe }
.Lreturn:
{ jumpr r31 }
{ nop }
