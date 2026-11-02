// SPDX-License-Identifier: GPL-3.0-only
// Semantic Hexagon source for file 0x56448..0x56464, physical 0x96cc8448.
.text
.space 0x448
.global cdsp_controller_seed_handoff
cdsp_controller_seed_handoff:
{ call #0xfa464 }
{ r0=##0xc0364130
  allocframe(#0) }
{ r1:0=combine(#1,##0xc0226160) }
{ deallocframe }
{ jump #0x62ec }
