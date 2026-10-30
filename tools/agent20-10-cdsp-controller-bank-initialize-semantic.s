.text
.org 0xb4c
{ r3 = r16 }
{ r0 = add(sp,#28)
  r1 = memw(r21 = ##0xc0367850) }
{ r3 -= lsr(r2,#12) }
{ r5:4 = combine(#-1,#4)
  r2 = #0
  memw(sp+#0) = r18 }
{ memw(sp+#12) = #16 }
{ memw(sp+#8) = #0 }
{ call controller_helper }
{ memw(sp+#4) = #4 }
.org 0x6af0
controller_helper:
{ nop }
