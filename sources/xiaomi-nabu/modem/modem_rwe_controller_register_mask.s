// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V60 duplex memory shims and protected controller register mask.
// File [0x3a72c, 0x3a778). The first three duplex entries are real code,
// followed by a small protected SYS/SYSCFG register update function.
.text
.macro MODEM_SYS_READ dest,source,marker=3
 .if (\dest<0)||(\dest>31)||(\source<0)||(\source>63)||((\marker!=1)&&(\marker!=3))
  .error "Invalid protected system read"
 .endif
 .word (0x6e800000 | (\source<<16) | (\marker<<14) | \dest)
.endm
.macro MODEM_SYS_WRITE system,source,marker=3
 .if (\system<0)||(\system>63)||(\source<0)||(\source>31)||((\marker!=1)&&(\marker!=3))
  .error "Invalid protected system write"
 .endif
 .word (0x67000000 | (\source<<16) | (\marker<<14) | \system)
.endm
.globl agent11_modem_controller_duplex_copy1
agent11_modem_controller_duplex_copy1:
{ r6 = memh(r16+#0); memd(r29+#216) = r17:16 }
.globl agent11_modem_controller_duplex_copy2
agent11_modem_controller_duplex_copy2:
{ r6 = memh(r16+#0); memd(r29+#112) = r17:16 }
.globl agent11_modem_controller_duplex_copy3
agent11_modem_controller_duplex_copy3:
{ r6 = memh(r16+#0); memd(r29+#224) = r1:0 }
.globl agent11_modem_controller_mask_and_trap
agent11_modem_controller_mask_and_trap:
{ allocframe(#16) }
{ memd(r29+#0) = r25:24 }
{ memd(r29+#8) = r27:26 }
MODEM_SYS_READ 25,0,3
MODEM_SYS_READ 24,6,3
{ r27:26 = memd(r25+#24) }
{ r4 = clrbit(r24,#16) }
{ r4 = clrbit(r4,#17) }
{ r4 = clrbit(r4,#18) }
MODEM_SYS_WRITE 6,4,3
{ trap0(#0) }
MODEM_SYS_WRITE 6,24,3
{ memd(r25+#24) = r27:26 }
{ r25:24 = memd(r29+#0) }
{ r27:26 = memd(r29+#8) }
{ dealloc_return }
