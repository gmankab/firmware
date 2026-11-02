// SPDX-License-Identifier: GPL-3.0-only
// Guard shared controller peripheral status and poll until two MMIO
// registers clear their bit-zero ready state. Executable ADSP file
// [0x10430,0x10460), 48 bytes / 12 V66 instruction words.
.text
.globl adsp_rwe_controller_status_poll
adsp_rwe_controller_status_poll:
{ immext(#0xf061e740); r0=memw(##0xf061e740); if(cmp.eq(r0.new,#0)) jump:t done }
{ immext(#0xf0017d00); r1=memw(##0xf0017d38); if(cmp.eq(r1.new,#0)) jump:t done }
{ memw(r0+#0)=clrbit(#2) }
poll_first:
{ r1=memw(r0+#0); if(tstbit(r1.new,#0)) jump:nt poll_first }
{ memw(r0+#12)=clrbit(#2) }
poll_second:
{ r1=memw(r0+#12); if(tstbit(r1.new,#0)) jump:nt poll_second }
done:
