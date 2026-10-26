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
