.text
.org 0xb00
cdsp_controller_next:
{
  r2 = ##0
  memd(r29+#-16) = r17:16; allocframe(#80)
}
{
  p0 = cmp.eq(r2,#0)
  memd(r29+#64) = r19:18
  memd(r29+#56) = r21:20
}
{
  if(p0) r2 = ##-0x40000000
  memd(sp+#48) = r23:22
  memd(sp+#40) = r25:24
}
{
  if(!p0) jump:nt next_fail
  memd(sp+#32) = r27:26
}
{
  r18 = lsr(r2,#12)
}
{ nop }
.org 0xe1c
next_fail:
{ nop }
