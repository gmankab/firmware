// SPDX-License-Identifier: GPL-3.0-only
// Independent Hexagon V60 startup in the second modem R E mapping:
// file [0x446000,0x4460d8), ELF vaddr [0x1e400000,0x1e4000d8).
// No reference bytes are embedded; vendor data after this routine is excluded.
// Hexagon C2_tfrrcr checked ISA encoding for framelimit, alias C16.
// LLVM6 assembler does not accept this control-register alias.
.macro HEX_WRITE_FRAMELIMIT src
    .if (\src < 0) || (\src > 31)
      .error "Hexagon framelimit source register out of range"
    .endif
    .word (0x62200000 | (\src << 16) | (3 << 14) | 16)
.endm
.text
.globl modem_late_aux_boot
modem_late_aux_boot:
{
  r29 = add(pc,##82752)
  r30 = #0
  r0 = #0
}
HEX_WRITE_FRAMELIMIT 0
{ r0 = add(pc,##110572) }
{ r1 = add(pc,##27060) }
{ gp = r0
  callr r1 }
{ r0 = add(pc,##78612) }
{ r1 = add(pc,##109388) }
{ call modem_late_aux_memset }
{ r0 = add(pc,##110656) }
{ r1 = add(pc,##110648) }
{ call modem_late_aux_memset }
{ r0 = add(pc,##110532) }
{ memw(r0+#0) = r16 }
{ r1 = add(pc,##82656) }
{ memw(r1+#0) = r17 }
{ r2 = add(pc,##26212) }
{ callr r2 }
{ r2 = add(pc,##512) }
{ callr r2 }
.Lwait:
{ jump .Lwait }
.globl modem_late_aux_memset
modem_late_aux_memset:
{
  r3 = add(r0,#7)
  r4 = #0
}
{
  r3 = insert(r4,#3,#0)
  r5 = #0
}
{ p0 = cmp.gt(r3,r1) }
{ if (p0) r3 = add(r1,#0) }
.Lbyte_loop:
{ p0 = cmp.eq(r0,r3); if (p0.new) jump:nt .Lbyte_loop_done }
{
  jump .Lbyte_loop
  memb(r0++#1) = r4
}
.Lbyte_loop_done:
{ p0 = cmp.eq(r0,r1); if (p0.new) jump:nt .Lreturn
  r3 = r1
}
{ r3 = insert(r4,#3,#0) }
.Lword_loop:
{ p0 = cmp.eq(r0,r3); if (p0.new) jump:nt .Lmiddle }
{
  jump .Lword_loop
  memd(r0++#8) = r5:4
}
.Lword_loop_done:
.Lmiddle:
{ r3 = r1 }
.Ltail_loop:
{ p0 = cmp.eq(r0,r3); if (p0.new) jump:nt .Lreturn }
{
  jump .Ltail_loop
  memb(r0++#1) = r4
}
.Ltail_loop_done:
.Lreturn:
{ jumpr r31 }
