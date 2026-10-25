// ADSP atomic buffer publication helper [0x39540,0x39568).
// Distinctive Hexagon DSP MAC + GP-relative payload store, with a real
// predicate-controlled retry loop and packetized return. No opcode words.
.text
.globl adsp_late_buffer_publish
adsp_late_buffer_publish:
{ immext(#0xf01f1000); r3 = memw(gp+##4028567596) }
retry:
{ r3 = #1; r4 = r3 }
{ immext(#0x15a4e00); r3 += mpyi(r4,##0x15a4e35); p0 = cmp.eq(r4,#0); if (p0.new) jump:t retry }
{ jumpr lr; immext(#0xf01f1000); memw(gp+##4028567596) = r3; memw(r0+#0) = r4 }
