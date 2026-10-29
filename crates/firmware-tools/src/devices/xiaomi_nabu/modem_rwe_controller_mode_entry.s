// SPDX-License-Identifier: GPL-3.0-only
// Agent 11 Hexagon V60 controller-mode dispatch and status write.
// Modem file [0x3a4c4, 0x3a50c), virtual +0xc0808000, physical +0x8e008000.
// Privileged instructions are expressed as checked ISA field encodings.
.text
.macro MODEM_CTRL_ACTION mode, marker=3
 .if ((\mode!=0x40)&&(\mode!=0x60)&&(\mode!=0x80))||((\marker!=1)&&(\marker!=3))
  .error "Unexpected mode command or packet marker"
 .endif
 .word (0x6c200000 | (\marker << 14) | \mode)
.endm
.macro MODEM_SYS_READ dest,source,marker=3
 .if (\dest<0)||(\dest>31)||(\source<0)||(\source>63)||((\marker!=1)&&(\marker!=3))
  .error "Invalid V60 system read"
 .endif
 .word (0x6e800000 | (\source << 16) | (\marker << 14) | \dest)
.endm
.macro MODEM_SYS_WRITE special,source,marker=3
 .if (\special<0)||(\special>63)||(\source<0)||(\source>31)||((\marker!=1)&&(\marker!=3))
  .error "Invalid V60 system write"
 .endif
 .word (0x67000000 | (\source << 16) | (\marker << 14) | \special)
.endm
.globl agent11_modem_rwe_controller_mode_entry
agent11_modem_rwe_controller_mode_entry:
MODEM_CTRL_ACTION 0x40,3
MODEM_CTRL_ACTION 0x60,3
MODEM_SYS_READ 13,16,1
{
 r15 = clrbit(r15,#22)
 r11 = #113
 r8 = memuh(r6+#224)
}
MODEM_SYS_READ 11,3,1
{
 r15 = insert(r11,#8,#0)
 r13 = add(r13,#24)
 r8 = or(r8,#2)
}
MODEM_SYS_WRITE 6,15,1
{
 r7 = ##-1065081588
 memh(r6+#224) = r8
}
MODEM_CTRL_ACTION 0x80,3
MODEM_SYS_WRITE 3,7,3
MODEM_SYS_WRITE 4,0,3
{ jumpr r13 }
