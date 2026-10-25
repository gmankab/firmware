// ADSP v66 late executable [file 0x3a86c, 0x3a8cc):
// Encode a 32-bit codec status word as eight zero-prefixed digit-like
// ASCII nibbles followed by a NUL byte. All source operations, no .word.
.text
.globl adsp_late_hex_prefix
adsp_late_hex_prefix:
{
  r3 = extractu(r1,#4,#24)
  r4 = extractu(r1,#4,#20)
  r2 = r1
  memb(r0+#8) = #0
}
{
  r2 = or(#48,lsr(r2,#28))
  r6 = extractu(r1,#4,#4)
  memb(r0+#0) = r2.new
}
{
  r3 = extractu(r1,#4,#12)
  r2 = or(r3,#48)
  memb(r0+#1) = r2.new
}
{
  r2 = or(r4,#48)
  r3 = or(r3,#48)
  memb(r0+#2) = r2.new
}
{
  r2 = extractu(r1,#4,#16)
  r3 = #48
  memb(r0+#4) = r3
}
{
  r2 = or(r2,#48)
  memb(r0+#3) = r2.new
}
{
  r2 = extractu(r1,#4,#8)
  r1 = or(r3,and(r1,#15))
  memb(r0+#7) = r1.new
}
{
  r2 = or(r2,#48)
  memb(r0+#5) = r2.new
}
{
  r2 = or(r6,#48)
  jumpr r31
  memb(r0+#6) = r2.new
}
