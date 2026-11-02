// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V60 protected controller low-level accessors, file [0x3aab4,0x3ab1c).
// C-register selector reads/writes are encoded by checked field-construction
// macros, as LLVM6 cannot disassemble their names. All other instructions
// are independently source-assembled architectural Hexagon mnemonics.
.text
.macro AGENT11_CREG_READ reg,selector,marker=1
 .if (\reg<0)||(\reg>31)||(\selector<0)||(\selector>63)||((\marker!=1)&&(\marker!=3))
  .error "Invalid Hexagon V60 controller register read selector"
 .endif
 .word (0x6e800000 | (\selector << 16) | (\marker << 14) | \reg)
.endm
.macro AGENT11_CREG_WRITE selector,reg,marker=3
 .if (\reg<0)||(\reg>31)||(\selector<0)||(\selector>63)||((\marker!=1)&&(\marker!=3))
  .error "Invalid Hexagon V60 controller register write selector"
 .endif
 .word (0x6c800000 | (\reg << 16) | (\marker << 14) | \selector)
.endm
.globl agent11_controller_special_status_update
agent11_controller_special_status_update:
AGENT11_CREG_READ 2,6,1
{ r1 = ##-1065353148 }
{ r2 = and(##133169152,asl(r2,#12)) }
{ r2 |= lsr(r1,#12) }
AGENT11_CREG_WRITE 2,2,3
{
 p0 = !cmp.gt(r2,#-1)
 if (p0.new) jumpr:nt r31
 if (!p0.new) memw(r1+#0) = r0
}
{ dccleaninva(r1) }
{ r0 = memw(r1+#0) }
{ jumpr r31 }
.globl agent11_controller_special_status_test
agent11_controller_special_status_test:
AGENT11_CREG_READ 2,6,1
{ r0 = ##-1064892928 }
{ r2 = and(##133169152,asl(r2,#12)) }
{ r2 |= lsr(r0,#12) }
AGENT11_CREG_WRITE 1,2,3
{
 p0 = tstbit(r1,#31)
 if (p0.new) jumpr:nt r31
}
{ r0 = extractu(r1,#8,#0) }
{ jumpr r31 }
{ r0 = #-1 }
{ jumpr r31 }
