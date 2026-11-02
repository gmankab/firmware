// SPDX-License-Identifier: GPL-3.0-only
// File [0x118c0,0x11928): ADSP v66 controller global record allocation.
// Four privileged register read slots are encoded with checked V66 fields.
.text
.globl adsp_global_record_prefix
adsp_global_record_prefix:
{ memd(r29+#-16)=r17:16; allocframe(#24) }
{ nop } // R2=SGP0
{ r16=memw(r2+#0); if(cmp.eq(r16.new,#0)) jump:nt no_record }
{ nop } // R1=REV
{ nop } // R2=GP
{ r3=#0; immext(#0xf02ca2c0); r4=memub(gp+##0xf02ca2c4); memb(r16+#227)=r4.new }
{ r0=#0; immext(#0xf02d1340); r4=memw(gp+##0xf02d1364); memb(r16+#20)=r4 }
{ immext(#0xf000eac0); r6=memw(gp+##0xf000eacc); immext(#0xf0021100); memw(r16+#236)=##0xf002111c }
{ p0=cmp.eq(r4,#0); memd(r16+#48)=r3:2; memw(r16+#248)=r6 }
{ nop } // R4=PCYCLELO
{ if(p0) r5=add(r3,#0); immext(#0xf020a040); r7=memw(gp+##0xf020a060); memd(r16+#136)=r1:0 }
.set no_record,adsp_global_record_prefix+(0x8be0e994-0x8be0e8c0)
