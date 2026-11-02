// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V60 auxiliary codec-buffer type configuration, file
// [0x446694,0x4467a0); executable logic through 0x446794, then three
// explicit architectural NOP instructions for function alignment.
// Parse and validate two channel formats (kind 1 or 2), reject invalid
// indices and call the prior registry lookup when a new buffer is needed.
.text
.globl modem_late_aux_type_config
modem_late_aux_type_config:
.set registry_lookup, modem_late_aux_type_config + (0x128 - 0x39c)
.set service_notify, modem_late_aux_type_config + (0x4a8 - 0x39c)
{
  p0 = cmp.eq(r1,#2)
  r16 = r2
  memd(r29+#-16) = r17:16
  allocframe(#8)
}
{ if (p0) jump:nt .Lmode2 }
{ p0 = cmp.eq(r1,#1); if (!p0.new) jump:nt .Linvalid }
{ r3:2 = bitsplit(r0,#8) }
{ r4 = ##16777215; if (cmp.eq(r4.new,r3)) jump:nt .Lvalid1 }
{ p0 = cmp.eq(r3,#0); if (!p0.new) jump:nt .Llookup1 }
{ p0 = cmp.eq(r2,#0); if (p0.new) jump:nt .Lzero1 }
.Llookup1:
{ call registry_lookup }
{ p0 = cmp.eq(r0,#0); if (!p0.new) jump:t .Lsuccess1 }
{ r0 = #-1 }
{ r17:16 = memd(r29+#0); dealloc_return }
.Lvalid1:
{ r2 = add(r2,#-253); if (cmp.gtu(r2.new,#2)) jump:t .Llookup1 }
.Lzero1:
{ r0 = #0; r17:16 = memd(r29+#0); memw(r16+#0) = #0 }
{ dealloc_return }
.Lsuccess1:
{ r0 = #0; r2 = memw(r0+#8); memw(r16+#0) = r2.new }
{ r17:16 = memd(r29+#0); dealloc_return }
.Lmode2:
{ r3:2 = bitsplit(r0,#8) }
{ r4 = ##16777215; if (cmp.eq(r4.new,r3)) jump:nt .Lvalid2 }
{ p0 = cmp.eq(r3,#0); if (!p0.new) jump:nt .Llookup2 }
{ p0 = cmp.eq(r2,#0); if (p0.new) jump:nt .Lzero2 }
.Llookup2:
{ call registry_lookup }
{ p0 = cmp.eq(r0,#0); if (!p0.new) jump:t .Lsuccess2 }
{ r0 = #-1 }
{ r17:16 = memd(r29+#0); dealloc_return }
.Lvalid2:
{ r2 = add(r2,#-253); if (cmp.gtu(r2.new,#2)) jump:t .Llookup2 }
.Lzero2:
{ r0 = #0; r17:16 = memd(r29+#0); memw(r16+#0) = #0 }
{ dealloc_return }
.Lsuccess2:
{ r0 = #0; r2 = memw(r0+#176) }
 .Lcheck2:
{ p0 = cmp.eq(r2,#0); if (p0.new) jump:nt .Lfail_return }
{ jump .Lgot_node }
.Lfind_node:
{ jump .Lcheck2; r2 = memw(r2+#0) }
.Lgot_node:
{ r3 = memw(r2+#4) }
{ r3 = memw(r3+#8); if (cmp.eq(r3.new,#0)) jump:t .Lfind_node }
{ r17:16 = memd(r29+#0); memw(r16+#0) = #1 }
{ dealloc_return }
.Linvalid:
{ r4 = memw(##507617104); if (cmp.eq(r4.new,#0)) jump:nt .Lnotify }
{ r1:0 = combine(##507576347,#3); r2 = #376 }
{ r3 = ##507576354; callr r4 }
.Lnotify:
{ call service_notify; r0 = ##507576354 }
{ r0 = #-1 }
.Lfail_return:
{ r17:16 = memd(r29+#0); dealloc_return }
{ nop }
{ nop }
{ nop }
