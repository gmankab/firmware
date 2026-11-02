// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V60 188-byte auxiliary heap/SRAM page-slot allocator. File
// [0x4462f8,0x4463b4); VMA [0x1e4002f8,0x1e4003b4).
// Reserve fixed 0x800-byte slots for small blocks, or a bounded portion
// of the shared 32-bit heap for larger requests. Executable instructions
// only; no OEM authentication or private reference byte arrays.
.text
.globl modem_late_aux_heap
modem_late_aux_heap:
{ r2 = add(r0,#-2049) }
{ r2 = lsr(r2,#11); if (!cmp.eq(r2.new,#0)) jump:t .Llarge }
{ r3 = ##507595600 }
{ r2 = memh(r3+#0); if (!cmp.eq(r2.new,#0)) jump:t .Lslot1 }
{ r2 = #0 }
.Lselect_small:
{ r4 = #4104; memh(r3+#0) = #1 }
{ r2 = add(##507595608,mpyi(r2,r4)) }
{ r0 = r2; jumpr r31 }
.Lslot1:
{ r2 = ##507595600 }
{ r4 = memh(r2+##4104); if (!cmp.eq(r4.new,#0)) jump:t .Lslot2 }
{ r2 = #1; jump .Lselect_small; r3 = add(r3,#4104) }
.Lslot2:
{ r4 = memh(r2+##8208); if (!cmp.eq(r4.new,#0)) jump:t .Lslot3 }
{ r2 = #2; jump .Lselect_small; r3 = add(r3,#8208) }
.Lslot3:
{ r2 = memh(r2+##12312) }
{ r2 = #0; p0 = cmp.eq(r2,#0); if (!p0.new) jump:t .Lcannot }
{ r2 = #3; jump .Lselect_small; r3 = add(r3,#12312) }
.Llarge:
{ r3:2 = combine(##507595592,#0) }
{ r4 = memw(r3+#4); if (cmp.gtu(r0,r4.new)) jump:t .Lcannot }
{
  r7:6 = bitsplit(r0,#3)
  r0 = add(#1,lsr(r0,#3))
  r2 = memw(r3+#0)
}
{ p0 = cmp.eq(r6,#0); if (p0.new) r0 = add(r7,#0) }
{ r4 -= asl(r0,#3); r0 = addasl(r2,r0,#3); memw(r3+#4) = r4.new }
{ jumpr r31; r0 = r2; memw(r3+#0) = r0 }
.Lcannot:
{ r0 = r2; jumpr r31 }
{ jumpr r31 }
