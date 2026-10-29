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

.macro MODEM_CTRL_SWAP_SGP0 gpr,marker=3
 .if (\gpr<0)||(\gpr>31)||(\marker!=3)
  .error "invalid SGP0 swap register or packet marker"
 .endif
 .word (0x65000000 | (\gpr<<16) | (\marker<<14))
.endm
.macro MODEM_CTRL_Y_READ gpr,yreg,marker=1
 .if (\gpr<0)||(\gpr>31)||(\yreg<0)||(\yreg>31)||(\marker<1)||(\marker>3)
  .error "invalid Y-register read"
 .endif
 .word (0x6a000000 | (\yreg<<16) | (\marker<<14) | \gpr)
.endm
.macro MODEM_CTRL_Y_WRITE special,gpr,marker=1
 .if (\special<0)||(\special>31)||(\gpr<0)||(\gpr>31)||(\marker<1)||(\marker>3)
  .error "invalid register-to-Y write or packet marker"
 .endif
 .word (0x62000000 | ((\gpr+32)<<16) | (\marker<<14) | \special)
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
// Initialize per-thread handler state; preserve the externally generated GP
// pointer and call earlier exception-frame helpers without inlining them.
.globl agent11_modem_rwe_dispatch_per_thread
agent11_modem_rwe_dispatch_per_thread:
{ allocframe(#0) }
.set modem_prev_capture,agent11_modem_rwe_controller_entry-0x4c8
{
  call modem_prev_capture
  r1 = setbit(r1,#29)
  r28 = r0
}
MODEM_CTRL_SYS_READ 1,0,3
.set modem_prev_dispatch_handler,agent11_modem_rwe_controller_entry-0x494
{
  r0 = r28
  p0 = cmp.eq(r0,#0)
  r1 = memw(r1+#0)
  if (p0) call modem_prev_dispatch_handler
}
{ r0 = add(r0,#8) }
MODEM_CTRL_SYS_READ 28,8,1
{ r1 = add(r28,#104) }
MODEM_CTRL_SYS_READ 0,0,3
.set modem_prev_service,agent11_modem_rwe_controller_entry-0x3dc
{
  r1 += mpyi(r28,#224)
  call modem_prev_service
}
.set modem_diag_success,agent11_modem_rwe_controller_entry+0x11bc
{ if (p0) call ##modem_diag_success }
// The controller routes a zero-status callback to a direct fast-exit.
.set modem_ctrl_call_done,agent11_modem_rwe_controller_entry+0xa0
{ if (p0) jump:nt modem_ctrl_call_done }
MODEM_CTRL_SYS_READ 18,28,3   // privileged status selector S28
{ r0 = memw(##-1065077624); memw(r1=##-1065081904) = r18 }
{ dccleana(r1) }
MODEM_CTRL_SYS_READ 1,8,1
{ r0 = extract(r0,#3,#0) }
.set modem_ctrl_error_return,agent11_modem_rwe_controller_entry+0xb4
{ p1 = cmp.eq(r0,r1); if (!p1.new) jump:nt modem_ctrl_error_return }
{ r1 = memw(r0=##-1065081908) }
{ r1 = add(r1,#1) }
{ memw(r0+#0) = r1 }
{ r0 = #2; dealloc_return }
modem_ctrl_call_done:
MODEM_CTRL_SYS_READ 0,8,1
.set modem_ctrl_cleanup_0,agent11_modem_rwe_controller_entry-0x2fc
{ call modem_ctrl_cleanup_0 }
MODEM_CTRL_SYS_READ 0,8,1
.set modem_ctrl_cleanup_1,agent11_modem_rwe_controller_entry-0x2d4
{ call modem_ctrl_cleanup_1 }
{ r0 = #0; dealloc_return }
modem_ctrl_error_return:
{ r0 = #1; dealloc_return }
// Probe three per-core controller pointers and zero the no-owner frame.
// The compare/new-branch V60 duplex form is assembled from symbolic operands.
.globl agent11_modem_rwe_probe_controller_owner
agent11_modem_rwe_probe_controller_owner:
.set modem_ctrl_owner_done,agent11_modem_rwe_controller_entry+0xe8
{ r18 = memw(##-1065076432); if (cmp.eq(r18.new,#0)) jump:t modem_ctrl_owner_done }
{ r18 = memw(##-1065067868); if (!cmp.eq(r18.new,#0)) jump:nt modem_ctrl_owner_done }
{ r18 = memw(##-1065265136); if (cmp.eq(r18.new,#0)) jump:t modem_ctrl_owner_done }
{ memw(r18+#36) = #0; memw(r18+#32) = #0 }
{ memw(r18+#16) = #0; memw(r18+#20) = #0 }
{ memw(r18+#4) = clrbit(#21) }
modem_ctrl_owner_done:
{ r18 = memw(##-1065268620) }
.set modem_ctrl_far_handler,agent11_modem_rwe_controller_entry+0x242a4
{ p0 = cmp.eq(r18,#0); if (p0.new) jump:t modem_ctrl_far_handler }
.set modem_ctrl_fallback_next,agent11_modem_rwe_controller_entry+0xee0
{ jump ##modem_ctrl_fallback_next }
// Capture the current video context into the protected exception frame.
.globl agent11_modem_rwe_begin_video_context
agent11_modem_rwe_begin_video_context:
MODEM_CTRL_SWAP_SGP0 9,3
MODEM_CTRL_SYS_READ 9,8,3
{ r9 = asl(r9,#18) }
{ r9.l = #22528 }
{ r9 |= lsr(r9,#16) }
{ r9.h = #49281 }
{ r9 = memw(r9+#0) }
MODEM_CTRL_SYS_READ 30,6,1
{ memd(r9+#8) = r31:30; memd(r9+#16) = r29:28 }
MODEM_CTRL_Y_READ 31,4,1
{ r28 = ##-805306368; memd(r9+#24) = r23:22 }
// Validate restored video frame boundaries before replacing frame registers.
.set modem_ctrl_video_frame_restore,agent11_modem_rwe_controller_entry+0x190
{
  p0 = !tstbit(r30,#16)
  if (p0.new) jump:nt modem_ctrl_video_frame_restore
  r30 = and(r30,#255)
  r29.l = #0
}
MODEM_CTRL_SYS_READ 22,9,1
.set modem_ctrl_frame_control_accept,agent11_modem_rwe_controller_entry+0x16c
{
  p0 = cmp.eq(r30,#35)
  if (p0.new) jump:nt modem_ctrl_frame_control_accept
  r29.h = #53248
}
MODEM_CTRL_SYS_READ 22,3,1
{
  p0 = cmp.eq(r30,#29)
  if (p0.new) jump:nt modem_ctrl_frame_control_accept
}
{ jump modem_ctrl_video_frame_restore }
// Bounds-check the active video frame; restore the protected SGP0 bank
// and tail-transfer to the saved video-thread dispatcher.
// Bounds check starts at 0x39f50; both earlier branches deliberately
// address the second instruction in the original packet.
{
  p0 = boundscheck(r23:22,r29:28):raw:lo
  if (!p0.new) jump:nt modem_ctrl_video_frame_restore
  memw(r9+#4) = r22
}
MODEM_CTRL_Y_WRITE 4,31,1
{ r23:22 = memd(r9+#24) }
{ r29:28 = memd(r9+#16); r31:30 = memd(r9+#8) }
MODEM_CTRL_SWAP_SGP0 9,3
.set modem_ctrl_video_thread_resume,agent11_modem_rwe_controller_entry+0x3fd14
{ jump ##modem_ctrl_video_thread_resume }
modem_ctrl_video_frame_restore:
MODEM_CTRL_Y_WRITE 4,31,1
{ r23:22 = memd(r9+#24) }
{ r29:28 = memd(r9+#16); r31:30 = memd(r9+#8) }
MODEM_CTRL_SWAP_SGP0 9,3
