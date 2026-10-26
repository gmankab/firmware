// SPDX-License-Identifier: GPL-3.0-only
// Independently assembled modem RWE protected exception-lock helpers.
// Disjoint from the prior [file 0x38108,0x390dc) verified region.
// This executable region begins after non-executable fill and opaque data;
// never count the gap as independently authored instruction bytes.
.text
.globl agent11_modem_rwe_bit31_lock_update
agent11_modem_rwe_bit31_lock_update:
{ r18 = memw_locked(r0) }
agent11_modem_rwe_bit31_lock_retry:
{
    p0 = tstbit(r18,#31)
    if (p0.new) jump:nt agent11_modem_rwe_bit31_lock_update
}
{ r18 = setbit(r18,#31) }
{ memw_locked(r0,p0) = r18 }
{
    p1 = tstbit(r18,#30)
    if (!p0) jump:nt agent11_modem_rwe_bit31_lock_update
}
.set agent11_modem_rwe_lock_failed,agent11_modem_rwe_bit31_lock_update + (0x3994c-0x39920)
{ r18 = clrbit(r18,#31); if (!p1) jump:nt agent11_modem_rwe_lock_failed }
{ jumpr lr; r0 = #1; memw(r0+#0) = r18 }
agent11_modem_rwe_lock_failed:
{ jumpr lr; r0 = #0; memw(r0+#0) = r1 }
// Capture a full privileged modem hardware-thread snapshot. Preserve the
// architected order of SYSCFG, MODECTL, PCYCLE, EVB, DIAG, revision, SGP0,
// interrupted address-control words and four interrupt masks. The compiler
// lacks mnemonic support for some late V60 S-register classes, so these
// public ISA register fields are encoded with checked macros.
.macro MODEM_FU_SYS_READ destination, system, marker=3
    .if (\destination < 0) || (\destination > 31) || (\system < 0) || (\system > 63)
      .error "invalid Hexagon system-register snapshot operands"
    .endif
    .if (\marker < 1) || (\marker > 3)
      .error "invalid Hexagon system-read packet marker"
    .endif
    .word (0x6e800000 | (\system<<16) | (\marker<<14) | \destination)
.endm
.macro MODEM_FU_SYS_PAIR_READ gpr_low, sys_low, marker=3
    .if (\gpr_low < 0) || (\gpr_low > 30) || (\gpr_low & 1) || (\sys_low < 0) || (\sys_low > 62) || (\sys_low & 1)
      .error "invalid protected S-register pair"
    .endif
    .word (0x6f000000 | (\sys_low<<16) | (\marker<<14) | \gpr_low)
.endm
.macro MODEM_FU_GET_IMASK gpr, selector, marker=3
    .if (\gpr < 0) || (\gpr > 31) || (\selector < 0) || (\selector > 3)
      .error "invalid modem hardware interrupt mask read"
    .endif
    .if (\marker != 3)
      .error "standalone interrupt-mask read required"
    .endif
    .word (0x66000000 | (\selector<<16) | (\marker<<14) | \gpr)
.endm
.macro MODEM_FU_CONFIG_WRITE special, gpr, marker=1
    .if (\special < 0) || (\special > 31) || (\gpr < 0) || (\gpr > 31) || (\marker != 1)
      .error "invalid config write and continuation marker"
    .endif
    .word (0x62000000 | ((\gpr+32)<<16) | (\marker<<14) | \special)
.endm
.globl agent11_modem_rwe_capture_masks
agent11_modem_rwe_capture_masks:
{ r19 = p3:0; r18 = r28; allocframe(#8) }
{ memd(sp+#0) = r19:18 }
MODEM_FU_SYS_READ 18,18,3 // SYSCFG
MODEM_FU_SYS_READ 19,17,3 // MODECTL
{ memd(r0+#0) = r19:18 }
MODEM_FU_SYS_READ 18,30,3 // PCYCLELO
MODEM_FU_SYS_READ 19,31,3 // PCYCLEHI
{ memd(r0+#8) = r19:18 }
MODEM_FU_SYS_PAIR_READ 18,56,3 // S57:56
{ memd(r0+#80) = r19:18 }
MODEM_FU_SYS_READ 18,16,3 // EVB
MODEM_FU_SYS_READ 19,28,3 // DIAG
{ memd(r0+#16) = r19:18 }
MODEM_FU_SYS_READ 18,20,3
MODEM_FU_SYS_READ 19,20,3
{ memd(r0+#24) = r19:18 }
MODEM_FU_SYS_READ 18,29,3 // REV
MODEM_FU_SYS_READ 19,0,3 // SGP0
{ r18 = r1; r19 = #0; memd(r0+#40) = r19:18 }
{ r1 = #0; memd(r0+#64) = r19:18 }
MODEM_FU_GET_IMASK 28,1,3
{ r1 = #1; memw(r0+#48) = r28 }
MODEM_FU_GET_IMASK 28,1,3
{ r1 = #2; memw(r0+#52) = r28 }
MODEM_FU_GET_IMASK 28,1,3
{ r1 = #3; memw(r0+#56) = r28 }
MODEM_FU_GET_IMASK 28,1,3
{ r19:18 = memd(sp+#0); memw(r0+#60) = r28 }
MODEM_FU_SYS_READ 28,63,3
{ memw(r0+#68) = r28 }
MODEM_FU_SYS_READ 28,60,3
{ memw(r0+#72) = r28 }
MODEM_FU_SYS_READ 28,35,3
{ memw(r0+#88) = r28 }
MODEM_FU_CONFIG_WRITE 4,19,1 // P3:0
{ r28 = r18 }
{ dealloc_return }
// Republish a suspended hardware thread's register snapshot into the next
// exception frame record. Each pair is a source-described Hexagon packet.
// Fixed architectural special-register fields encode A32-unrecognized reads.
.macro MODEM_FU_Y_READ gpr, yreg, marker=1
    .if (\gpr < 0) || (\gpr > 31) || (\yreg < 0) || (\yreg > 31)
      .error "invalid Hexagon Y-register operand"
    .endif
    .if (\marker < 1) || (\marker > 3)
      .error "invalid Hexagon Y-register packet marker"
    .endif
    .word (0x6a000000 | (\yreg<<16) | (\marker<<14) | \gpr)
.endm
.macro MODEM_FU_CONTROL_PAIR_READ gpr_low, creg_low, marker=3
    .if (\gpr_low < 0) || (\gpr_low > 30) || (\gpr_low & 1) || (\creg_low < 0) || (\creg_low > 30) || (\creg_low & 1)
      .error "invalid Hexagon C-register pair"
    .endif
    .if (\marker < 1) || (\marker > 3)
      .error "invalid C-register packet marker"
    .endif
    .word (0x68000000 | (\creg_low<<16) | (\marker<<14) | \gpr_low)
.endm
.globl agent11_modem_rwe_republish_controller_frame
agent11_modem_rwe_republish_controller_frame:
{ r3:2 = memd(r0+#56); memd(r1+#8) = r3:2 }
{ r3:2 = memd(r0+#80); memd(r1+#0) = r3:2 }
{ r3:2 = memd(r0+#72); memd(r1+#120) = r3:2 }
{ r3:2 = memd(r0+#64); memd(r1+#112) = r3:2 }
MODEM_FU_Y_READ 2,4,1 // saved P3:0
{ memd(r1+#72) = r3:2 }
MODEM_FU_Y_READ 3,8,1 // USR
{ memd(r1+#104) = r27:26 }
MODEM_FU_Y_READ 2,11,1 // GP
{ memd(r1+#168) = r3:2 }
MODEM_FU_Y_READ 3,10,1 // UGP
{ memd(r1+#96) = r25:24 }
MODEM_FU_Y_READ 2,0,1 // SA0
{ memd(r1+#136) = r3:2 }
MODEM_FU_Y_READ 3,1,1 // LC0
{ memd(r1+#88) = r23:22 }
MODEM_FU_Y_READ 2,2,1 // SA1
{ memd(r1+#144) = r3:2 }
MODEM_FU_Y_READ 3,3,1 // LC1
{ memd(r1+#80) = r21:20 }
MODEM_FU_Y_READ 2,6,1 // M0
{ memd(r1+#152) = r3:2 }
MODEM_FU_Y_READ 3,7,1 // M1
{ memd(r1+#64) = r17:16 }
{ memd(r1+#160) = r3:2 }
MODEM_FU_Y_READ 2,12,1 // CS0
{ memd(r1+#56) = r15:14 }
MODEM_FU_Y_READ 3,13,1 // CS1
{ memd(r1+#48) = r13:12 }
{ memd(r1+#184) = r3:2 }
MODEM_FU_SYS_READ 2,7,1 // CCR, late V60 control register selector
{ memd(r1+#40) = r11:10 }
{ memw(r1+#192) = r2 }
{ memd(r1+#32) = r9:8 }
{ memd(r1+#24) = r7:6 }
{ memd(r1+#16) = r5:4 }
MODEM_FU_CONTROL_PAIR_READ 2,16,3 // C17:16
{ memd(r1+#216) = r3:2 }
MODEM_FU_SYS_READ 3,6,3 // SSR
MODEM_FU_SYS_READ 2,3,3 // ELR
{ memd(r1+#128) = r3:2 }
MODEM_FU_SYS_READ 2,9,3 // BADVA
MODEM_FU_SYS_READ 3,2,3 // STID
{ r3:2 = memd(r1+#8); memd(r1+#176) = r3:2 }
{ jumpr lr }
// Swap the protected SGP0 bank around a two-pair context save. The checked
// V60 special-register operation (0x65 ISA class) lacks an LLVM6 mnemonic.
.macro MODEM_FU_SWAP_SGP0 gpr, marker=3
    .if (\gpr < 0) || (\gpr > 31) || (\marker < 1) || (\marker > 3)
      .error "invalid protected SGP0 bank-swap operand"
    .endif
    .word (0x65000000 | (\gpr<<16) | (\marker<<14))
.endm
.macro MODEM_FU_NMI source, marker=3
    .if (\source < 0) || (\source > 31) || (\marker != 3)
      .error "invalid controller NMI source or packet marker"
    .endif
    .word (0x64000000 | ((0x60 | \source)<<16) | (\marker<<14) | 0x40)
.endm
.globl agent11_modem_rwe_swap_video_frame
agent11_modem_rwe_swap_video_frame:
MODEM_FU_SWAP_SGP0 9,3
{ memd(r9+#56) = r1:0; memd(r9+#64) = r19:18 }
MODEM_FU_SWAP_SGP0 9,3
{ jumpr lr }
// Mask an interrupt bit in the per-thread controller word, update the NMI
// control bit and dispatch the architected V60 non-maskable interrupt.
.globl agent11_modem_rwe_raise_controller_nmi
agent11_modem_rwe_raise_controller_nmi:
{ r1 = memw(r28+##-1065068280) }
{ r1 = clrbit(r1,r0) }
{ r2 = ##-1065077624 }
{ r0 = memw(r2+#0) }
{ r0 = setbit(r0,#28) }
{ memw(r2+#0) = r0 }
MODEM_FU_NMI 1,3
{ jumpr lr }
// Nested hardware-loop scan of the protected per-thread interrupt bit-mask.
// Loop0 scans four 32-bit mask lanes while Loop1 bounds up to 1000 retries.
.globl agent11_modem_rwe_scan_protected_masks
agent11_modem_rwe_scan_protected_masks:
{ r1 = ##-1065081900; r4 = #1000 }
{ r5 = memw(r28+##-1065068280) }
{ loop1(agent11_modem_rwe_mask_loop_setup,r4) }
agent11_modem_rwe_mask_loop_setup:
{ loop0(agent11_modem_rwe_mask_lane,#4) }
agent11_modem_rwe_mask_lane:
{ r3 = lc0; r6 = #0; r4 = #0 }
{ r7 = lc1; r3 = sub(#4,r3) }
{ p0 = cmp.eq(r3,r0); if (p0.new) jump:nt agent11_modem_rwe_mask_break }
{ r6 = lsl(#1,r3) }
{ r6 = and(r6,r5); if (cmp.eq(r6.new,#0)) jump:nt agent11_modem_rwe_mask_break }
{ r4 = add(r4,#1) }
{ p0 = cmp.eq(r4,r7); if (p0.new) jump:nt agent11_modem_rwe_mask_break }
{ r2 = memw_locked(r1) }
agent11_modem_rwe_mask_retry:
{ p0 = tstbit(r2,r3); if (!p0.new) jump:t agent11_modem_rwe_mask_retry_previous }
agent11_modem_rwe_mask_break:
{ nop; nop; nop }:endloop0:endloop1
{ jumpr lr }
.set agent11_modem_rwe_mask_retry_previous,agent11_modem_rwe_mask_retry - 0xc
// Bounded MODECTL retry/update. When masked controller status becomes zero,
// clear the shared word; otherwise update the current per-thread lane.
.globl agent11_modem_rwe_sync_controller_modectl
agent11_modem_rwe_sync_controller_modectl:
{ r5 = ##-1065081912; r4 = #10000; nop }
agent11_modem_rwe_sync_controller_poll:
{ r4 = add(r4,#-1); p0 = cmp.eq(r4,#0); if (!p0.new) jump:nt agent11_modem_rwe_sync_controller_read }
{ jump agent11_modem_rwe_sync_controller_done; memw(r5+#0) = r1 }
agent11_modem_rwe_sync_controller_read:
MODEM_FU_SYS_READ 1,17,3 // MODECTL
{ r2 = extractu(r1,#4,#0); r3 = extractu(r1,#4,#16); r4 = memw(r28+##-1065068280) }
{ r2 = xor(r2,r3) }
{ r2 = and(r2,r4) }
{ r2 = clrbit(r2,r0); if (!cmp.eq(r2.new,#0)) jump:t agent11_modem_rwe_sync_controller_poll }
{ r4 = #0; memw(r5+#0) = r4.new }
agent11_modem_rwe_sync_controller_done:
{ jumpr lr }
// Unmask privileged K0 lock and conditionally reset the frame descriptor
// fields when all controller-status words are ready. Each GP-relative read
// retains its original extender; V60 class conversion is guarded in Python.
.macro MODEM_FU_K0_UNLOCK marker=3
    .if (\marker != 3)
      .error "K0 unlock is a standalone privileged instruction"
    .endif
    .word (0x6c200080 | (\marker<<14))
.endm
.globl agent11_modem_rwe_unlock_controller
agent11_modem_rwe_unlock_controller:
MODEM_FU_K0_UNLOCK 3
.set agent11_fu_skip_frame_reset,agent11_modem_rwe_unlock_controller + (0x39c18-0x39bc0)
.set agent11_fu_resume_other_frame,agent11_modem_rwe_unlock_controller + (0x39d04-0x39bc0)
{ r0 = memw(r28+##-1065268620); if (!cmp.eq(r0.new,#0)) jump:t agent11_fu_skip_frame_reset }
{ r0 = memw(r28+##-1065076432); if (cmp.eq(r0.new,#0)) jump:t agent11_fu_skip_frame_reset }
{ r0 = memw(r28+##-1065067868); if (cmp.eq(r0.new,#0)) jump:t agent11_fu_skip_frame_reset }
{ r0 = memw(r28+##-1065265136); if (cmp.eq(r0.new,#0)) jump:nt agent11_fu_skip_frame_reset }
{ memw(r0+#36) = #0; memw(r0+#32) = #0 }
{ memw(r0+#16) = #0; memw(r0+#20) = #0 }
{ memw(r0+#64) = #0; memw(r0+#60) = #0 }
{ memw(r0+#92) = #0; memw(r0+#88) = #0 }
{ memw(r0+#120) = #0; memw(r0+#116) = #0 }
{ memw(r0+#4) = clrbit(#21) }
agent11_fu_skip_frame_reset:
{ r0 = memw(r28+##-1065077680); if (cmp.eq(r0.new,#0)) jump:t agent11_fu_resume_other_frame }
