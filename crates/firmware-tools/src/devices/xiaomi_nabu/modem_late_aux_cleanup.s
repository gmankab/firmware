// SPDX-License-Identifier: GPL-3.0-only
// Independently reconstructed V60 controller callback/resource cleanup.
// Original modem MBN file [0x4474b0,0x4475c0), virtual 0x1e4014b0.
.text
.globl modem_late_aux_cleanup
modem_late_aux_cleanup:
{ r17:16 = combine(##507617144,r0)
  memd(r29+#-16) = r17:16; allocframe(#16) }
{ memd(r29+#0) = r19:18 }
{ r0 = memw(r16+#4)
  if (cmp.eq(r0.new,#0)) jump:nt .Lskip_first }
{ r2 = memw(r17+#0) }
{ callr r2 }
{ memw(r16+#4) = #0 }
.Lskip_first:
{ r0 = memw(r16+#124)
  if (!cmp.eq(r0.new,#0)) jump:t .Lregister_chain }
.Llist_begin:
{ r0 = memw(r16+#168)
  if (cmp.eq(r0.new,#0)) jump:nt .Lnext_list }
{ r18 = add(r16,#168) }
.Lfirst_loop:
{ r2 = memw(r0+#0)
  memw(r16+#168) = r2.new }
{ p0 = cmp.eq(r2,#0)
  r3 = memw(r17+#0)
  if (p0.new) memw(r16+#172) = r18 }
{ callr r3 }
{ r0 = memw(r16+#168)
  if (!cmp.eq(r0.new,#0)) jump:t .Lfirst_loop }
.Lnext_list:
{ r0 = memw(r16+#176)
  if (cmp.eq(r0.new,#0)) jump:nt .Literate_descriptors }
{ r18 = add(r16,#176) }
.Lsecond_loop:
{ r2 = memw(r0+#0)
  memw(r16+#176) = r2.new }
{ p0 = cmp.eq(r2,#0)
  r3 = memw(r17+#0)
  if (p0.new) memw(r16+#180) = r18 }
{ callr r3 }
{ r0 = memw(r16+#176)
  if (!cmp.eq(r0.new,#0)) jump:t .Lsecond_loop }
.Literate_descriptors:
{ r2 = memw(r16+#32) }
{ r2 = add(r2,#-1)
  p0 = cmp.gt(r2,#0); if (!p0.new) jump:nt .Lafter_descriptors
  memw(r16+#32) = r2.new }
.Ldescriptor_loop:
{ r3 = memw(r16+#28) }
{ r3 = add(r3,mpyi(#28,r2)) }
{ r0 = memw(r3+#4)
  if (cmp.eq(r0.new,#-1)) jump:nt .Lskip_descriptor }
{ r4 = memw(r3+#16)
  r1 = memw(r3+#12) }
{ callr r4
  r2 = memw(r3+#24) }
{ r2 = memw(r16+#32) }
.Lskip_descriptor:
{ r2 = add(r2,#-1)
  p0 = cmp.gt(r2,#0); if (p0.new) jump:t .Ldescriptor_loop
  memw(r16+#32) = r2.new }
.Lafter_descriptors:
{ r0 = memw(r16+#28)
  if (cmp.eq(r0.new,#0)) jump:nt .Lafter_array }
{ r2 = memw(r17+#0) }
{ callr r2 }
{ memw(r16+#28) = #0 }
.Lafter_array:
{ r0 = memw(r16+#36)
  if (cmp.eq(r0.new,#0)) jump:nt .Lafter_staging }
{ r2 = memw(r17+#0) }
{ callr r2 }
{ memw(r16+#36) = #0 }
.Lafter_staging:
{ r0 = r16
  r2 = memw(r17+#0) }
{ callr r2 }
{ r17:16 = memd(r29+#8); r19:18 = memd(r29+#0) }
{ dealloc_return }
.Lregister_chain:
{ r2 = memw(r17+#0)
  r3 = memw(r0+#0) }
{ callr r2
  memw(r16+#124) = r3 }
.Lregister_again:
{ r0 = memw(r16+#124)
  if (!cmp.eq(r0.new,#0)) jump:t .Lregister_chain }
{ jump .Llist_begin }
