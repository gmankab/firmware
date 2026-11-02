// Independently authored Hexagon assembly, two diagnostics and alignment.
// Original controller RWE segment is mapped from physical 0x96cc8000.
.text
.space 0xadc
controller_no_candidates:
{ call #0x4de8
  r1=#365
  r0=##0xc0226175 }
controller_unexpected_state:
{ call #0x4dd8
  r1=#366
  r0=##0xc0226175 }
{ nop }
