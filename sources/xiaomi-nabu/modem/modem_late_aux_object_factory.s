// SPDX-License-Identifier: GPL-3.0-only
// Source-authored V60 controller-object allocation and field initialization.
// MBN file [0x4475c0,0x447608), vaddr [0x1e4015c0,0x1e401608).
.text
.globl modem_late_aux_object_factory
modem_late_aux_object_factory:
.set allocate_object, modem_late_aux_object_factory + (0x8168 - 0x178)
{ r0 = #208
  r2 = memw(##507617140)
  allocframe(#8) }
{ callr r2
  memd(r29+#0) = r17:16 }
{ r16 = r0
  if (cmp.eq(r16.new,#0)) jump:nt .Lfinish }
{ call allocate_object
  r1:0 = combine(#0,r16)
  r2 = #208 }
{ r2 = add(r16,#168)
  r0 = add(r16,#176)
  memw(r16+#172) = r2.new }
{ memw(r16+#176) = #0
  memw(r16+#180) = r0 }
.Lfinish:
{ r0 = r16
  r17:16 = memd(r29+#0); dealloc_return }
