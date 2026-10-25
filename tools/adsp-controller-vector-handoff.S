// SPDX-License-Identifier: GPL-3.0-only
// ADSP controller exception-vector handoff, file [0x4cc0,0x4ce8).
.text
.globl adsp_controller_vector_handoff
adsp_controller_vector_handoff:
{ r1 = add(r23,##0xf0001e00) }
{ nop } // V66 EVB = r1
{ call controller_vector_reset }
{ r25 = memw(r23+##0xf0003008) }
{
  p0 = cmp.eq(r25,#2)
  p1 = cmp.eq(r25,#0)
}
{ if (p0) jump:nt controller_resume }
{ if (!p1) jump:nt controller_fault }
.set controller_vector_reset, adsp_controller_vector_handoff - 0x630
.set controller_resume, adsp_controller_vector_handoff + 0x4f0
.set controller_fault, adsp_controller_vector_handoff + 0x500
