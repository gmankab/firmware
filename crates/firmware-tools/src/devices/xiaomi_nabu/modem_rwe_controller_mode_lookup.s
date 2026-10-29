// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V60 protected controller mode lookup and handoff.
// Modem file offsets [0x3a50c,0x3a5a0); virtual +0xc0808000,
// physical +0x8e008000; the original exception return closes this interval.
// All privileged words are checked field encodings, not copied OEM bytes.
.text
.macro MODEM_CTRL_ACTION action,marker=3
 .if ((\action!=0x60)&&(\action!=0x80))||((\marker!=1)&&(\marker!=3))
  .error "Invalid privileged mode action"
 .endif
 .word (0x6c200000 | (\marker<<14) | \action)
.endm
.macro MODEM_SYS_WRITE reg,source,marker=3
 .if (\reg<0)||(\reg>63)||(\source<0)||(\source>31)||((\marker!=1)&&(\marker!=3))
  .error "Invalid protected system register write"
 .endif
 .word (0x67000000 | (\source<<16) | (\marker<<14) | \reg)
.endm
.macro MODEM_RTE marker=3
 .if (\marker!=3)
  .error "RTE must terminate a packet"
 .endif
 .word (0x57e00000 | (\marker<<14))
.endm
.globl agent11_modem_controller_error_trap
agent11_modem_controller_error_trap:
{ p3 = and(p3,and(p1,!p1)); r11 = add(r11,#-4) }
{ if(p3) r1 = #0; if(p3) r0 = #0 }
{ trap0(#10) }
.globl agent11_modem_controller_protected_lookup
agent11_modem_controller_protected_lookup:
MODEM_CTRL_ACTION 0x60,3
{ r7 = add(r3,#-1) }
{ r7 = and(r7,#127) }
{ r9 = memw(##3229988928) }
{ p3 = cmp.gtu(r7,r9) }
.set agent11_mode_failure, agent11_modem_controller_protected_lookup+0x70
{ if(p3) jump:nt agent11_mode_failure }
{ r7 = +mpyi(r7,#112) }
{ r7 = add(r7,##-1065058000) }
{ r13 = memw(r7+#88) }
{ p3 = !cmp.eq(r13,r3) }
{ if(p3) jump:nt agent11_mode_failure }
{ r13 = memw(r7+#92) }
{ p3 = cmp.eq(r13,#0) }
{ if(p3) jump:nt agent11_mode_failure }
{ r13 = memw(r7+#104) }
{
 r11 = r31
 r12 = r15
 r15 = r13
 r8 = memh(r6+#224)
}
{
 r7 = ##-1065080596
 r8 = or(r8,#2)
}
MODEM_SYS_WRITE 6,15,1
{
 r31 = ##-1065080292
 memh(r6+#224) = r8
}
agent11_mode_failure:
{ if(p3) r7 = add(r31,#0) }
MODEM_SYS_WRITE 3,7,3
MODEM_CTRL_ACTION 0x80,3
MODEM_RTE 3
