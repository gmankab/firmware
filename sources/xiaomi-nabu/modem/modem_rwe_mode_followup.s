// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V60 protected controller follow-up and exception return.
// Firmware file [0x3a444,0x3a47c), virtual c0842444..c084247c,
// physical 8e042444..8e04247c. Built independently using ISA source;
// the original image is never an input to the assembler.
.text
.macro AGENT11_PRIV_MODE_ACTION mode,marker=3
 .if ((\mode!=0x20)&&(\mode!=0x40))||((\marker!=1)&&(\marker!=3))
  .error "Invalid V60 protected mode action"
 .endif
 .word (0x6c200000 | (\marker<<14) | \mode)
.endm
.macro AGENT11_PRIV_MODE_COMMIT field,gpr,marker=3
 .if (\field!=3)||((\gpr!=2))||((\marker!=1)&&(\marker!=3))
  .error "Unexpected privileged controller state commit"
 .endif
 .word (0x6c800000 | (\field<<16) | (\marker<<14) | \gpr)
.endm
.macro AGENT11_PRIV_MODE_SELECT field,marker=3
 .if (\field!=2)||((\marker!=1)&&(\marker!=3))
  .error "Unrecognized V60 controller mode selector"
 .endif
 .word (0x6c400000 | (\field<<16) | (\marker<<14))
.endm
.macro AGENT11_RTE marker=3
 .if ((\marker!=1)&&(\marker!=3))
  .error "Invalid architectural RTE packet marker"
 .endif
 .word (0x57e00000 | (\marker<<14))
.endm
.globl agent11_modem_rwe_mode_controller_followup
agent11_modem_rwe_mode_controller_followup:
// Pair return-from-exception with the absolute privileged monitor load.
AGENT11_RTE 1
{r0=memw(##3249316820)}
// Inspect paired controller control bits, fall back if inactive.
.set agent11_controller_done,agent11_modem_rwe_mode_controller_followup+0x34
{
 r3:2 = asl(r1:0,#20)
 if(!p0) jump:nt agent11_controller_done
 r1:0 = combine(#0,#0)
}
AGENT11_PRIV_MODE_ACTION 0x20,3
AGENT11_PRIV_MODE_COMMIT 3,2,3
// LLVM6 puts the .new branch before its producing compare; the strict
// verifier reorders *only* these two known operand-checked instruction words
// and sets parse bits, never copying original OEM bytes.
.set agent11_controller_select,agent11_modem_rwe_mode_controller_followup+0x2c
{ p0 = !cmp.gt(r2,#-1); if (p0.new) jump:nt agent11_controller_select }
AGENT11_PRIV_MODE_SELECT 2,3
AGENT11_PRIV_MODE_ACTION 0x40,3
AGENT11_RTE 3
AGENT11_RTE 3
