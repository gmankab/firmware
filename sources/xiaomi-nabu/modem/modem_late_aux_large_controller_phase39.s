// SPDX-License-Identifier: GPL-3.0-only
// Controller callback resolution, result propagation, and resource retry.
// All addresses are symbolic relative to this fragment's place in the
// original executable ELF section. No OEM instruction words embedded.
.text
.globl modem_late_aux_large_controller_phase39
modem_late_aux_large_controller_phase39:
.set no_result_callback, modem_late_aux_large_controller_phase39 + 0x2b0
.set no_private_callback, modem_late_aux_large_controller_phase39 + 0x40
.set platform_notifier, modem_late_aux_large_controller_phase39 - 0x1ec8
.set resource_scanner, modem_late_aux_large_controller_phase39 - 0xbc8
.set registry_resolver, modem_late_aux_large_controller_phase39 - 0x15f0
{
  r1 = r22
  r3:2 = combine(r24,r23)
  r4 = memw(r16+#12)
  r0 = memw(r29+#88)
}
{callr r4}
{
  p0 = cmp.eq(r0,#0)
  if (p0.new) jump:t no_result_callback
}
{
  r4 = memw(r19+#0)
  if (cmp.eq(r4.new,#0)) jump:nt no_private_callback
}
{
  r3:2 = memd(r29+#64)
  memw(r29+#0) = r22
}
{
  r1:0 = combine(r3,r24)
  r2 = #499
  r3 = ##507577890
}
{
  r1:0 = combine(##507576841,#3)
  callr r4
  memd(r29+#8) = r1:0
}
no_private_callback:
{
  r1:0 = memd(r29+#64)
  memw(r29+#0) = r22
}
{
  r25 = r1
  r0 = ##507577890
}
{
  call platform_notifier
  memd(r29+#8) = r25:24
}
{jump resource_scanner}
{
  call registry_resolver
  r1 = memw(r18+#-8)
}
