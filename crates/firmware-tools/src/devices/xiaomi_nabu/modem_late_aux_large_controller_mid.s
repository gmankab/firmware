// SPDX-License-Identifier: GPL-3.0-only
// Source-reconstructed Hexagon V60 controller session handler continuation.
// Starts at modem file 0x447724; 0x447724 + delta is in vaddr 0x1e401724.
// The function remains incomplete; symbols preserve out-of-fragment targets.
.text
.globl modem_late_aux_large_controller_mid
modem_late_aux_large_controller_mid:
.set diagnostic_console, modem_late_aux_large_controller_mid - 0xf84
.set handler_resource, modem_late_aux_large_controller_mid + 0x7e8c
.set later_controller_tail, modem_late_aux_large_controller_mid + 0x1338
.set early_controller_exit, modem_late_aux_large_controller_mid + 0x37c
.set fail_session, modem_late_aux_large_controller_mid + 0x44
.set missing_controller, modem_late_aux_large_controller_mid + 0x34
.set continue_session, modem_late_aux_large_controller_mid + 0xd8
{
  r0 = #208
  r2 = memw(r19+#36)
}
{ callr r2 }
{
  r17 = r0
  if (!cmp.eq(r17.new,#0)) jump:t fail_session
}
{
  r4 = memw(r19+#0)
  if (cmp.eq(r4.new,#0)) jump:nt missing_controller
}
{
  r1:0 = combine(##507576841,#3)
  r2 = #256
}
{
  r3 = ##507576878
  callr r4
}
missing_controller:
{
  call diagnostic_console
  r0 = ##507576878
}
{ jump later_controller_tail }
fail_session:
{
  call handler_resource
  r1:0 = combine(#0,r17)
  r2 = #200
}
{
  r2 = add(r17,#168)
  r1:0 = combine(#0,#0)
  memw(r17+#172) = r2.new
}
{
  r2 = add(r17,#176)
  memw(r17+#176) = #0
}
{
  r0 = #0
  memw(r17+#180) = r2
  memd(r17+#200) = r1:0
}
{
  r3:2 = combine(#1,#3)
  r6 = memw(r16+#48)
  r1 = memw(r24=##507617080)
}
{
  callr r6
  r5:4 = combine(#0,r20)
  memw(r29+#0) = r20
}
{
  r21 = r0
  if (!cmp.eq(r21.new,#-1)) jump:t continue_session
}
{
  r4 = memw(r19+#0)
  if (cmp.eq(r4.new,#0)) jump:nt .Lno_callback
}
{
  r1:0 = combine(##507576841,#3)
  r2 = memw(r24+#0)
  memw(r29+#0) = r2.new
}
{
  r3 = ##507576899
  r2 = #265
  memw(r29+#4) = #0
}
{ callr r4 }
.Lno_callback:
{
  r0 = ##507576899
  r2 = memw(r24+#0)
  memw(r29+#0) = r2.new
}
{
  call diagnostic_console
  memw(r29+#4) = #0
}
{ jump early_controller_exit }
continue_session:
