// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V60 controller-word/bitfield operating routine.
// File [0x3a47c,0x3a4c4), virtual [0xc084247c,0xc08424c4),
// physical [0x8e04247c,0x8e0424c4).
// Typed Hexagon assembly, protected controller actions encoded from checked
// operands. The compare/jump packet has an original ordering LLVM6 reverses.
.text
.macro AGENT11_MODE_ACTION immediate,marker=3
 .if ((\immediate!=0x20)&&(\immediate!=0x40))||((\marker!=1)&&(\marker!=3))
  .error "Invalid Hexagon V60 controller action"
 .endif
 .word (0x6c200000 | (\marker<<14) | \immediate)
.endm
.macro AGENT11_MODE_COMMIT field,gpr,marker=3
 .if (\field!=2)|| (\gpr!=3) || ((\marker!=1)&&(\marker!=3))
  .error "Invalid protected controller-state register commit"
 .endif
 .word (0x6c800000 | (\field<<16) | (\marker<<14) | \gpr)
.endm
.macro AGENT11_MODE_SELECT field,marker=3
 .if (\field!=3) || ((\marker!=1)&&(\marker!=3))
  .error "Invalid protected mode selection field"
 .endif
 .word (0x6c400000 | (\field<<16) | (\marker<<14))
.endm
.macro AGENT11_RTE marker=3
 .if (\marker != 3)
  .error "Protected mode epilogue must end exception packet"
 .endif
 .word (0x57e00000 | (\marker<<14))
.endm
.globl agent11_modem_rwe_mode_bitfield_window
agent11_modem_rwe_mode_bitfield_window:
AGENT11_MODE_ACTION 0x20,3
{ r15 = insert(r1,#7,#8)
  r3:2 = lsr(r1:0,#12)
  r12 = r15
}
AGENT11_MODE_COMMIT 2,3,3
.set mode_zero_return,agent11_modem_rwe_mode_bitfield_window+0x48
{ p0 = !cmp.gt(r3,#-1)
  if(p0.new) jump:nt mode_zero_return
  r4 = r0
}
AGENT11_MODE_SELECT 3,3
{ r5 = ct0(r0)
  r3:2 = asl(r1:0,#2)
  r7:6 = combine(#12,#28)
}
{ r3:2 = insert(r1:0,#24,#39)
  r7 = addasl(r7,r5,#1)
}
{ r3:2 = insert(r5:4,r7:6) }
{ r3:2 = lsr(r3:2,r6) }
AGENT11_MODE_ACTION 0x40,3
AGENT11_RTE 3
