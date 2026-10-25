// ADSP V66 [file 0x3be80,0x3be9c): derive a capped halfword index
// from the count of leading zero bits in a controller parameter.
.text
.globl adsp_late_halfword_order
adsp_late_halfword_order:
{ r2 = cl0(r0); r3 = #14 }
{ r2 = sub(#19,r2) }
{ r0 = lsr(r2,#1); p0 = cmp.gtu(r3,r2) }
{ if (!p0) r0 = #6; jumpr r31 }
