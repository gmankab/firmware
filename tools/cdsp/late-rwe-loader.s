.text
.global cdsp_controller_loader
cdsp_controller_loader:
.space 0x60
entry:
{ memw(gp+##0xc036488c)=r3 }
{ call ##0x5878
  sp=add(sp,#-8)
  memd(sp+#-8)=r1:0 }
{ call ##0x1710
  r1:0=memd(sp+#0) }
{ call ##0x2878
  r1:0=memd(sp+#0) }
{ r18=##0xc028c000 }
{ call ##0x148 }
{ memw(r18+#0)=r0 }
{ call ##0x224 }
{ call ##0x5984
  r1:0=memd(sp+#0) }
{ call ##0x2830
  r1:0=memd(sp+#0) }
{ call ##0xfb280
  r1:0=memd(sp++#8) }
end_spin:
{ jump end_spin }
