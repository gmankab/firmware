// SPDX-License-Identifier: GPL-3.0-only
// Source-reassembled Hexagon V66 controller register snapshot [0x6568,0x65f4).
// Cross-segment call targets are physical addresses, not ELF file offsets.
.text
.globl adsp_rwe_register_snapshot
adsp_rwe_register_snapshot:
{ allocframe(#16) }
{ memd(sp+#8)=r19:18 }
{ call snapshot_precheck; r1=setbit(r1,#29); r28=r0 }
{ memw(sp+#0)=r0 }
{ nop } // checked replacement: r1=SGP0
{ r0=r28; p0=cmp.eq(r0,#0); r1=memw(r1+#0) }
{ immext(#0x1ea00); if(p0) call snapshot_fallback; r0=add(r0,#8) }
{ nop } // checked replacement: r9=SGP0
{ r2=memw(r9+#0) }
{ r5:4=memd(r2+#24); r7:6=memd(r2+#32) }
{ nop } // checked replacement: ELR=r4
{ r7:6=memd(r2+#40); memd(r9+#80)=r7:6 }
{ immext(#0xf0025100); r0=##0xf0025128; memd(r9+#72)=r7:6 }
{ nop } // checked replacement: r3=HTID
{ r1=add(r0,#104); r0=r9 }
{ r1+=mpyi(r3,#224); immext(#0x1ea80); if(p0) call snapshot_notify }
{ immext(#0x201c0); if(p0) call snapshot_commit }
{ r0=memw(sp+#0) }
{ p0=cmp.eq(r0,#0); if(!p0.new) jump:nt halt }
{ r19:18=memd(sp+#8) }
{ dealloc_return }
halt:
{ jump halt }
.set snapshot_precheck,adsp_rwe_register_snapshot+(0x8be21f80-0x8be03568)
.set snapshot_fallback,adsp_rwe_register_snapshot+(0x8be21fb4-0x8be03568)
.set snapshot_notify,adsp_rwe_register_snapshot+(0x8be2206c-0x8be03568)
.set snapshot_commit,adsp_rwe_register_snapshot+(0x8be237b8-0x8be03568)
