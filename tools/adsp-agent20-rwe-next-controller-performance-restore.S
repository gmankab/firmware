// SPDX-License-Identifier: GPL-3.0-only
// V66 performance monitor controller restore [0x138f0,0x139c8).
// Source NOPs reserve unsupported privileged register transfers only.
.text
.globl adsp_performance_restore
adsp_performance_restore:
{ immext(#0xf001c500); r2=memw(gp+##0xf001c500); if(cmp.eq(r2.new,#0)) jump:nt exit_early }
{ immext(#0xf0021f40); r2=##0xf0021f70; immext(#0xf0021f40); r3=##0xf0021f70 }
acquire_lock:
{ r0=memw_locked(r2) }
{ p0=cmp.eq(r0,#0) }
{ if(!p0) jump:nt acquire_lock }
{ memw_locked(r2,p0)=r2 }
{ if(!p0) jump:nt acquire_lock }
{ nop } // R0=SYSCFG
{ r0=clrbit(r0,#9) }
{ nop } // SYSCFG=R0
{ isync }
{ memw(r2+#0)=#0 }
{ immext(#0xf0010dc0); r4=memw(r2=##0xf0010df0) }
{ nop } // PMUCNT0=R4
{ r4=memw(r2+#4) }
{ nop } // PMUCNT1=R4
{ r4=memw(r2+#8) }
{ nop } // PMUCNT2=R4
{ r4=memw(r2+#12) }
{ nop } // PMUCNT3=R4
{ r4=memw(r2+#16) }
{ nop } // S44=R4
{ r4=memw(r2+#20) }
{ nop } // S45=R4
{ r4=memw(r2+#24) }
{ nop } // S46=R4
{ r4=memw(r2+#28) }
{ nop } // S47=R4
{ r4=memw(r2+#32) }
{ nop } // PMUCFG=R4
{ r4=memw(r2+#36) }
{ nop } // S55=R4
{ r4=memw(r2+#40) }
{ nop } // PMUEVTCFG=R4
{ r2=memw(r2+#44) }
{ nop } // S54=R2
acquire_second:
{ r0=memw_locked(r3) }
{ p0=cmp.eq(r0,#0) }
{ if(!p0) jump:nt acquire_second }
{ memw_locked(r3,p0)=r3 }
{ if(!p0) jump:nt acquire_second }
{ nop } // R0=SYSCFG
{ r0=setbit(r0,#9) }
{ nop } // SYSCFG=R0
{ isync }
{ memw(r3+#0)=#0 }
{ jumpr lr }
exit_early:
{ jumpr lr }
