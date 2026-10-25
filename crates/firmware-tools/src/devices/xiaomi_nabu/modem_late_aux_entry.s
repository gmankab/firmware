// SPDX-License-Identifier: GPL-3.0-only
// Auxiliary controller entry veneer at original modem file [0x4467f0,0x446804).
// Preserve Hexagon V60 packet grouping and symbolic offsets.
.text
.globl modem_late_aux_entry
modem_late_aux_entry:
.set controller_call, modem_late_aux_entry + (0x4870 - 0x7f0)
.set controller_tail, modem_late_aux_entry + (0x6584 - 0x7f0)
{
  call controller_call
  allocframe(#0)
}
{
  r0 = ##507612016
  deallocframe
}
{ jump controller_tail }
