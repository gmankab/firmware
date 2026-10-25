// Original Hexagon runtime thread/buffer handoff function at ADSP file
// [0x394b8,0x39540). Distinct QuRT traps, synchronized packet predicates,
// platform-buffer allocation and restore are source-authored Hexagon ISA.
.text
.globl adsp_late_buffer_handoff
adsp_late_buffer_handoff:
{ r17:16 = combine(r0,r1); memd(r29+#-16) = r17:16; allocframe(#40) }
{ memd(sp+#24) = r19:18 }
{ immext(#0xf01f1000); r2 = memw(gp+##4028567592); if (cmp.eq(r2.new,#0)) jump:nt finish }
{ r0 = r2; r19 = add(sp,#8); memw(r29+#8) = r2 }
{ call platform_buffer_allocate; memw(r19+#4) = #0 }
{ memw(sp+#16) = r0 }
{ r0 = #1 }
{ r1 = r19 }
{ trap0(#0) }
{ r2 = r0 }
{ memw(sp+#4) = r2 }
{ r2 = memw(sp+#4); if (cmp.gt(r2.new,#1)) jump:t setup_second }
{ r2 = #0; jump finish; immext(#0xf01f1000); memw(gp+##4028567592) = r2.new }
setup_second:
{ r2 = memw(sp+#4); memw(sp+#8) = r2.new }
{ memw(r29+#12) = r17; memw(r29+#16) = r16 }
{ r0 = #6 }
{ r1 = r19 }
{ trap0(#0) }
{ r2 = memw(sp+#4); memw(sp+#8) = r2.new }
{ r0 = #2 }
{ r1 = r19 }
{ trap0(#0) }
finish:
{ r17:16 = memd(r29+#32); r19:18 = memd(r29+#24) }
{ dealloc_return }
.set platform_buffer_allocate, adsp_late_buffer_handoff+0x10c288
