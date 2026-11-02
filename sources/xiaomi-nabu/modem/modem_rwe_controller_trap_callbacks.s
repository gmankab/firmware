// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V60 controller exception traps and callback-frame preservation.
// Source-derived modem file [0x3aa1c,0x3aa8c), virtual/physical offsets
// +0xc0808000 and +0x8e008000. Intentional self-loops are retained.
.text
.globl agent11_modem_trap_class10
agent11_modem_trap_class10:
{ trap0(#10) }
agent11_trap10_loop:
{ jump agent11_trap10_loop }
.globl agent11_modem_trap_class8
agent11_modem_trap_class8:
{ trap0(#8) }
agent11_trap8_loop:
{ jump agent11_trap8_loop }
.globl agent11_modem_trap_class9
agent11_modem_trap_class9:
{ trap0(#9) }
agent11_trap9_loop:
{ jump agent11_trap9_loop }
.globl agent11_modem_controller_callback_frame
agent11_modem_controller_callback_frame:
{ allocframe(#48) }
{
 memd(r30+#-48) = r17:16
 memd(r30+#-40) = r19:18
}
{
 memd(r30+#-32) = r21:20
 memd(r30+#-24) = r23:22
}
{
 memd(r30+#-16) = r25:24
 memd(r30+#-8) = r27:26
}
{ p3 = !cmp.eq(r2,#0) }
.set agent11_callback_restore,agent11_modem_controller_callback_frame+0x34
.set agent11_callback_test,agent11_modem_controller_callback_frame+0x1c
{ if(!p3) jump:nt agent11_callback_restore }
{ callr r4 }
{ if(p3) jump:nt agent11_callback_restore }
{ callr r5 }
{ if(!p3) jump:nt agent11_callback_test }
agent11_callback_restore:
{
 r17:16 = memd(r30+#-48)
 r19:18 = memd(r30+#-40)
}
{
 r21:20 = memd(r30+#-32)
 r23:22 = memd(r30+#-24)
}
{
 r25:24 = memd(r30+#-16)
 r27:26 = memd(r30+#-8)
}
{ r0 = mux(p3,#-1,#0); deallocframe }
{ jumpr r31 }
