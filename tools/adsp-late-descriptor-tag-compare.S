// ADSP Hexagon V66 [file 0x3bfb8,0x3bfcc): compare the low 20 bits
// of two hardware descriptor type/size words and return the difference.
.text
.globl adsp_late_descriptor_tag_compare
adsp_late_descriptor_tag_compare:
{ r2 = memw(r0+#4); r3 = memw(r1+#4) }
{ r2 = extractu(r2,#20,#0); r3 = extractu(r3,#20,#0) }
{ r0 = sub(r2,r3); jumpr r31 }
