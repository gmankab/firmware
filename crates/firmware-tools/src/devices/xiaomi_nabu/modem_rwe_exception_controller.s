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
.macro MODEM_CTRL_CONTROL_PAIR_READ gpr_low,creg_low,marker=3
 .if (\gpr_low<0)||(\gpr_low>30)||(\gpr_low & 1)||(\creg_low<0)||(\creg_low>62)||(\creg_low & 1)||(\marker<1)||(\marker>3)
  .error "invalid privileged control-register pair read"
 .endif
 .word (0x68000000 | (\creg_low<<16) | (\marker<<14) | \gpr_low)
.endm
.macro MODEM_CTRL_PRIVILEGED_SELECT action,marker=3
 .if (\action!=0x40)||(\marker!=3)
  .error "invalid protected page-map selection"
 .endif
 .word (0x6c200000 | (\marker<<14) | \action)
.endm
.macro MODEM_CTRL_RETURN_FROM_EXCEPTION marker=3
 .if (\marker!=3)
  .error "RTE must be standalone end-of-packet"
 .endif
 .word (0x57e00000 | (\marker<<14))
.endm
.macro MODEM_CTRL_SYS_WRITE sysreg,gpr,marker=3
 .if (\sysreg<0)||(\sysreg>63)||(\gpr<0)||(\gpr>31)||(\marker<1)||(\marker>3)
  .error "invalid privileged system register write"
 .endif
 .word (0x67000000 | (\gpr<<16) | (\marker<<14) | \sysreg)
.endm
.macro MODEM_CTRL_EXT_CONTROL_WRITE field,gpr,marker=3
 .if (\field<0)||(\field>63)||(\gpr<0)||(\gpr>31)||(\marker<1)||(\marker>3)
  .error "invalid extended controller field write"
 .endif
 .word (0x63000000 | (\field<<16) | (\marker<<14) | \gpr)
.endm
.macro MODEM_CTRL_PRIVILEGED_ACTION action,marker=3
 .if (\action!=0x20)||(\marker!=3)
  .error "unexpected protected address-space action"
 .endif
 .word (0x6c200000 | (\marker<<14) | \action)
.endm
.macro MODEM_CTRL_PRIVILEGED_STATE_COMMIT field,gpr,marker=3
 .if (\field!=2)||(\gpr<0)||(\gpr>31)||(\marker!=3)
  .error "invalid privileged controller-state commit field or register"
 .endif
 .word (0x6c800000 | (\field<<16) | (\marker<<14) | \gpr)
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
// Restore the controller's exception address base, then construct its
// mode-dependent frame-slot pointer. The compare/new-branch is in one packet.
.globl agent11_modem_rwe_select_controller_slot
agent11_modem_rwe_select_controller_slot:
MODEM_CTRL_EXT_CONTROL_WRITE 0,2,3
MODEM_CTRL_SYS_READ 0,6,1
{ r1 = ##50331648 }
.set modem_ctrl_no_frame_slot,agent11_modem_rwe_controller_entry+0x2dc
{
  r0 = sub(##620756992,asl(r0,#24))
  r1 = r0
  if (cmp.gtu(r0.new,r1)) jump:nt modem_ctrl_no_frame_slot
}
// Publish an exception-frame pointer, load the current controller S3 state
// and shift a packed pair to identify the next controller-page candidate.
MODEM_CTRL_PRIVILEGED_ACTION 0x20,3
{ memd(##-1065082048) = r3:2 }
MODEM_CTRL_SYS_READ 0,3,1
{ r1 = extractu(r1,#7,#8) }
{ r3:2 = lsr(r1:0,#12); r0 = add(r0,#4) }
// Check alignment and validity of the selected table entry before an
// atomic controller-state update. Branches preserve references to later
// register restore code without importing any unreconstructed instructions.
MODEM_CTRL_PRIVILEGED_STATE_COMMIT 2,3,3
.set modem_ctrl_mask_lock_retry,agent11_modem_rwe_controller_entry+0x248
{ r3 = and(r3,#-256); if (!cmp.eq(r3.new,#0)) jump:nt modem_ctrl_mask_lock_retry }
{ r3 = ##1809833343; r2 = memw(r0+#-4) }
.set modem_ctrl_format_counter_fallback,agent11_modem_rwe_controller_entry+0x214
{
  r3 = extractu(r2,#2,#14)
  r2 = or(r2,##5037116)
  if (cmp.eq(r2.new,r3)) jump:nt modem_ctrl_format_counter_fallback
}
// Count down the codec-format slot index with two explicitly decoded
// packet-local .new compares before submitting the protected frame.
.set modem_ctrl_revisit_format,agent11_modem_rwe_controller_entry+0x1d8
{
  r3 = add(r3,#-1)
  if (!cmp.gtu(r3.new,#1)) jump:nt modem_ctrl_revisit_format
}
.set modem_ctrl_commit_frame,agent11_modem_rwe_controller_entry+0x2d0
{ jump modem_ctrl_commit_frame }
.set modem_ctrl_retry_frame_slot,agent11_modem_rwe_controller_entry+0x210
{ r3 = add(r3,#-1); if (cmp.gtu(r3.new,#1)) jump:nt modem_ctrl_retry_frame_slot }
{ r3:2 = lsr(r1:0,#12); r0 = add(r0,#4) }
MODEM_CTRL_PRIVILEGED_STATE_COMMIT 2,3,3
.set modem_ctrl_update_format_counter,agent11_modem_rwe_controller_entry+0x248
{ r3 = and(r3,#-256); if (!cmp.eq(r3.new,#0)) jump:nt modem_ctrl_update_format_counter }
{ r2 = memw(r0+#-4) }
{ r3 = extractu(r2,#2,#14) }
.set modem_ctrl_test_format_again,agent11_modem_rwe_controller_entry+0x21c
{ r3 = add(r3,#-1); if (!cmp.gtu(r3.new,#1)) jump:nt modem_ctrl_test_format_again }
MODEM_CTRL_SYS_WRITE 3,0,1
{ p3 = cmp.eq(r0,r0) }
// Acquire and release the shared controller-frame lock with a bounded
// architectural locked-word retry. Publish the mask only for a valid lane.
.set modem_ctrl_shared_lock_retry,agent11_modem_rwe_controller_entry+0x250
{ r2 = ##-1065090812 }
{ r3 = memw_locked(r2) }
{
  p0 = cmp.eq(r3,#0)
  if (!p0.new) jump:nt modem_ctrl_shared_lock_retry
  if (p0.new) r3 = #1
}
{ memw_locked(r2,p0) = r3 }
{ if (!p0) jump:nt modem_ctrl_shared_lock_retry }
MODEM_CTRL_SYS_READ 2,6,1
{ r3 = ##65536 }
.set modem_ctrl_release_shared_owner,agent11_modem_rwe_controller_entry+0x2b0
{ p0 = bitsclr(r2,r3); if (p0.new) jump:nt modem_ctrl_release_shared_owner }
MODEM_CTRL_SYS_READ 2,8,1
{ r3 = memw(##-1065090816) }
{ trace(r3); r3 = clrbit(r3,r2) }
{
  p0 = !cmp.eq(r3,#0)
  if (p0.new) jump:t modem_ctrl_release_shared_owner
  memw(##-1065090816) = r3
}
// Finish protected interrupt-mask synchronization. Register selector S63 is
// the per-thread interrupt status, not a general-purpose register index.
MODEM_CTRL_SYS_READ 2,63,3
{ trace(r2); r2 = clrbit(r2,#16) }
MODEM_CTRL_SYS_WRITE 63,2,3
{ trace(r2) }
modem_ctrl_release_shared_owner:
{ r3 = #0; memw(##-1065090812) = r3.new }
// Snapshot the masked interrupt controller state and finish the normal
// protected exception re-entry branch through its architectural RTE.
MODEM_CTRL_CONTROL_PAIR_READ 0,34,1
{ r3:2 = memd(##-1065082048) }
MODEM_CTRL_PRIVILEGED_SELECT 0x40,3
MODEM_CTRL_RETURN_FROM_EXCEPTION 3
