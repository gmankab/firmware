// SPDX-License-Identifier: GPL-3.0-only
// Independent Hexagon V60 controller-mode release and exception return.
// The preceding [file 0x3a33c,0x3a3d8) is non-source zero/memory fill
// and not included in this executable island. Mapping: virtual 0xc08423d8,
// physical 0x8e0423d8 for the first byte at file 0x3a3d8.
.text
.macro MODEM_MODE_SYS_READ dest,sys,marker=3
 .if (\dest<0)||(\dest>31)||(\sys<0)||(\sys>63)||(\marker<1)||(\marker>3)
  .error "invalid mode selector privileged register read"
 .endif
 .word (0x6e800000 | (\sys<<16) | (\marker<<14) | \dest)
.endm
.macro MODEM_MODE_SYS_WRITE sysreg,gpr,marker=3
 .if (\sysreg<0)||(\sysreg>63)||(\gpr<0)||(\gpr>31)||(\marker<1)||(\marker>3)
  .error "invalid mode selector privileged register write"
 .endif
 .word (0x67000000 | (\gpr<<16) | (\marker<<14) | \sysreg)
.endm
.macro MODEM_MODE_RTE marker=3
 .if (\marker!=3)
  .error "return from exception must terminate packet"
 .endif
 .word (0x57e00000 | (\marker<<14))
.endm
.globl agent11_modem_rwe_release_controller_mode
agent11_modem_rwe_release_controller_mode:
MODEM_MODE_SYS_READ 7,3,3
{ r14 = clrbit(r15,#19) }
{ p1 = cmp.eq(r0,#2) }
{ if (p1) r7 = ##-1065080180 }
{ p1 = cmp.eq(r0,#3) }
{ if (p1) r7 = ##-1065080160 }
MODEM_MODE_SYS_WRITE 6,15,3
MODEM_MODE_SYS_WRITE 3,7,3
MODEM_MODE_RTE 3
