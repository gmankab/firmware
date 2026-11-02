// SPDX-License-Identifier: GPL-3.0-only
// Complete global-record allocation path and original error trap,
// file [0x11928,0x119c0); one K0 unlock uses checked V66 encoding.
.text
.globl adsp_global_record_completion
adsp_global_record_completion:
{ r2=memw(r16+#256) }
{ r0=#0; if(!p0) r5=xor(r4,r6); immext(#0xf000eac0); r1=memw(gp+##0xf000ead0) }
{ immext(#0xf0036040); r6=##0xf0036060; memw(r16+#252)=r5; memd(r16+#144)=r7:6 }
{ r1:0=combine(#0,r16); immext(#0xf00250c0); r4=memw(gp+##0xf00250cc); memd(r16+#40)=r1:0 }
{ r4=#0; r7=memw(r2+#104); memw(r16+#220)=r4 }
{ immext(#0xf00250c0); r5=memw(gp+##0xf00250d0); memd(r16+#24)=r7:6 }
{ r2=memw(r16+#208); memd(r16+#176)=r5:4 }
{ call update_record_status; memw(r16+#232)=r2 }
{ call finish_record_registration; r0=r16 }
{ nop } // v66 k0unlock
{ r17:16=memd(r29+#16); dealloc_return }
// Trap path for missing required shared-controller state.
no_record:
{ immext(#0xf000e3c0); r0=##0xf000e3c9; r2=#538; memw(sp+#8)=r2.new }
{ call controller_log; immext(#0xf000ea00); memw(sp+#4)=##0xf000ea0c; memw(sp+#0)=#14 }
{ call controller_report; r0=#3589 }
record_trap:
{ jump record_trap }
.set update_record_status,adsp_global_record_completion+(0x8be0e3f0-0x8be0e928)
.set finish_record_registration,adsp_global_record_completion+(0x8be23ac4-0x8be0e928)
.set controller_log,adsp_global_record_completion+(0x8be11438-0x8be0e928)
.set controller_report,adsp_global_record_completion+(0x8be227f4-0x8be0e928)
