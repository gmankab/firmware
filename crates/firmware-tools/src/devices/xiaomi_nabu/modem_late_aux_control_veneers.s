// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V60 controller service veneers, file [0x446bdc,0x446c18).
// One initial NOP closes the preceding function's instruction alignment.
// Three further NOPs close the MMIO helper; two close the tail-call wrapper.
.text
{ nop }
.globl modem_late_aux_sync_mmio
modem_late_aux_sync_mmio:
{
  r2 = memw(##507588608)
  memw(##507617212) = r2.new
}
{ jumpr r31 }
{ nop }
{ nop }
{ nop }
.globl modem_late_aux_zero_scalar_and_call
modem_late_aux_zero_scalar_and_call:
.set input_validator, modem_late_aux_zero_scalar_and_call + (0x0a68 - 0x0c00)
{
  r3 = #0
  jump ##input_validator
}
{ nop }
{ nop }
.globl modem_late_aux_zero_pair_and_call
modem_late_aux_zero_pair_and_call:
.set input_validator2, modem_late_aux_zero_pair_and_call + (0x0a68 - 0x0c10)
{
  r3:2 = combine(#0,#0)
  jump input_validator2
}
