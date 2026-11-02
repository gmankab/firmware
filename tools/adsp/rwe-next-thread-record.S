// SPDX-License-Identifier: GPL-3.0-only
// ADSP V66 thread-record registration and optional controller lock handling.
// Source executable file [0x10dc4,0x10eb4); begins with a one-word return
// stub, then a separate function at file 0x10dc8; its epilogue at 0x10eb0.
.text
.globl adsp_rwe_thread_record_registration
adsp_rwe_thread_record_registration:
{ jumpr lr }
{ r16=r0; memd(r29+#-16)=r17:16; allocframe(#24) }
{ memd(r29+#8)=r19:18; memd(r29+#0)=r21:20 }
{ nop } // checked R18=HTID
{ nop } // checked R21:20 = S57:56
{ call register_thread; immext(#0xf0026880); r0=##0xf0026890 }
{ immext(#0xf0026880); r0=##0xf0026890; immext(#0xf0025480); r2=memw(##0xf00254a8) }
{ r2=+mpyi(r2,#24); r4=add(r2,#1); r3=memub(r16+#244) }
{ p0=cmp.eq(r4,#5); immext(#0xf0025400); memw(r2<<#0+##0xf0025434)=r3 }
{ r5=mux(p0,#0,r4); immext(#0xf0025400); memw(r2<<#0+##0xf0025438)=r18 }
{ immext(#0xf0025400); memw(r2<<#0+##0xf0025430)=r16 }
{ immext(#0xf0025440); memd(r2<<#0+##0xf0025440)=r21:20; immext(#0xf0025480); memw(##0xf00254a8)=r5 }
{ call prepare_thread_context }
{ immext(#0xf0025100); r2=memw(##0xf0025120); if(cmp.eq(r2.new,#0)) jump:nt state_done }
{ r2=memub(r16+#224) }
{ p0=tstbit(r2,#4); if(p0.new) jump:t special_hardware; if(p0.new) r2=memub(r16+#21) }
{ nop } // typed v66 k0lock
{ r2=add(r16,#224); immext(#0xeeeeffc0); r3=##0xeeeeffff; memw(r16+#12)=r3.new }
{ memh(r2+#0)=setbit(#8) }
{ nop } // typed v66 k0unlock
{ call notify_hardware; r0=r16 }
{ jump state_done }
special_hardware:
{ immext(#0xf0025100); r0=##0xf0025128; immext(#0x40051400); r1=or(r2,##0x40051400) }
{ call handle_thread_state }
state_done:
{ immext(#0xf0025100); r0=##0xf0025128; r2=memub(r16+#21); r17:16=memd(sp+#16) }
{ immext(#0x40051300); r1=or(r2,##0x40051300); r19:18=memd(r29+#8); r21:20=memd(r29+#0) }
{ jump handle_thread_state; deallocframe }
.set register_thread,adsp_rwe_thread_record_registration+(0x261bc-0x10dc4)
.set prepare_thread_context,adsp_rwe_thread_record_registration+(0x261e8-0x10dc4)
.set notify_hardware,adsp_rwe_thread_record_registration+(0x14b20-0x10dc4)
.set handle_thread_state,adsp_rwe_thread_record_registration+(0x254a0-0x10dc4)
