// SPDX-License-Identifier: GPL-3.0-only
// Agent 11 third source-generated Hexagon V60 RWE executable island.
// The previous function ended with jumpr lr at file 0x39de4.
// First function is a privileged callback/dispatch dispatcher; subsequent
// branches are verified in later non-overlapping checkpoints.
.text
.macro MODEM_CTRL_SYS_READ dest,sys,marker=3
  .if (\dest<0)||(\dest>31)||(\sys<0)||(\sys>63)||(\marker<1)||(\marker>3)
    .error "invalid Hexagon privileged system register read"
  .endif
  .word (0x6e800000 | (\sys<<16) | (\marker<<14) | \dest)
.endm

.globl agent11_modem_rwe_controller_entry
agent11_modem_rwe_controller_entry:
// Public privileged system register S0 / per-thread controller selector.
MODEM_CTRL_SYS_READ 29,0,3
// This call shares the packet with both register moves; the packet-marker
// bit 15 must stay clear on the call, as in a legitimate 3-word packet.
.set agent11_modem_rwe_restore_previous,agent11_modem_rwe_controller_entry-0xe0
{
  call agent11_modem_rwe_restore_previous
  r30 = r0
  r0 = r29
}
// V60 GP-relative load (LLVM6 uses the generic r28 memory class):
// the generator checks all assembled operands before selecting the V60 class.
{ r1 = memw(r28+##-1065268620) }
// Far codec dispatch is a genuine compound compare-and-branch with an
// architecture immediate extender, *not* an opaque machine-word constant.
.set agent11_modem_rwe_far_codec,agent11_modem_rwe_controller_entry+0x24248
{ p0 = cmp.eq(r1,#0); if (p0.new) jump:t agent11_modem_rwe_far_codec }
.set agent11_modem_rwe_fallback,agent11_modem_rwe_controller_entry+0x28c4
{ jump agent11_modem_rwe_fallback }
