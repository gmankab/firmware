.text
.global cdsp_runtime_pic_record
cdsp_runtime_pic_record:
.space 0xd4
{ jumpr lr }
{ r0=add(pc,##0x1c3f2c) }
{ memw(r0+#0)=lr }
{ r0=add(pc,##0x1c3f24) }
{ memw(r0+#0)=sp }
{ r0=#-1 }
{ jump ##0xfbdcc }
{ nop }
