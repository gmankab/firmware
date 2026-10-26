// SPDX-License-Identifier: GPL-3.0-only
// File [0x1180c,0x11894): final SRAM controller setup, K0 lock and tail call.
// R0=SYSCFG, SYSCFG=R0, and R2=SGP0 are V66-only and appear as NOP slots.
.text
.globl adsp_sram_state_finalization
adsp_sram_state_finalization:
{ immext(#0xf0000000); r3=##0xf0000000; immext(#0xf02d1480); r2=memw(gp+##0xf02d1494) }
{ r3=lsr(r3,#2); r4=#0; p0=cmp.eq(r2,#0) }
{ r4=insert(r3,#30,#2); call protect_controller; if(!p0) memw(r2+#0)=r4.new }
{ immext(#0xf0021f40); r2=##0xf0021f70 }
{ r0=memw_locked(r2) }
{ p0=cmp.eq(r0,#0) }
{ if(!p0) jump:nt lock_retry }
{ memw_locked(r2,p0)=r2 }
{ if(!p0) jump:nt lock_retry }
{ nop } // R0=SYSCFG
{ r0=setbit(r0,#4) }
{ nop } // SYSCFG=R0
{ isync }
{ memw(r2+#0)=#0 }
{ nop } // R2=SGP0
{ r2=memw(r2+#0); immext(#0xf01eafc0); r1=memb(gp+##0xf01eafe4) }
{ call finalize_controller; r0=memw(r2+#256) }
{ nop } // R2=SGP0
{ r1=#0; r2=memw(r2+#0) }
{ call close_controller; r0=memw(r2+#256) }
{ r17:16=memd(r29+#8); r19:18=memd(r29+#0) }
{ jump cleanup_handoff; deallocframe }
.set protect_controller,adsp_sram_state_finalization+(0x8be150f4-0x8be0e80c)
.set lock_retry,adsp_sram_state_finalization+(0x8be0e838-0x8be0e80c)
.set finalize_controller,adsp_sram_state_finalization+(0x8be1fb50-0x8be0e80c)
.set close_controller,adsp_sram_state_finalization+(0x8be1f248-0x8be0e80c)
.set cleanup_handoff,adsp_sram_state_finalization+(0x8be1d684-0x8be0e80c)
