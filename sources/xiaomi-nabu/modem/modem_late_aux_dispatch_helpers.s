// SPDX-License-Identifier: GPL-3.0-only
// Two Hexagon V60 controller helper routines, modem file [0x446650,0x446694).
// Read the current controller-global pointer, then walk the linked callback
// registry until an invoked callback asks to terminate the traversal.
.text
.globl modem_late_aux_read_active
modem_late_aux_read_active:
{ jumpr r31; r0 = memw(##507617212) }
{ nop }
.globl modem_late_aux_walk_callbacks
modem_late_aux_walk_callbacks:
{
  r17:16 = combine(r1,r2)
  memd(r29+#-16) = r17:16
  allocframe(#16)
}
{ memd(r29+#0) = r19:18 }
{ r18 = memw(r0+#0); if (cmp.eq(r18.new,#0)) jump:nt .Ldone }
{ jump .Lcallback }
.Lnext:
{ r18 = memw(r18+#0); if (cmp.eq(r18.new,#0)) jump:nt .Ldone }
.Lcallback:
{ callr r17; r1 = r16; r0 = memw(r18+#4) }
{ p0 = cmp.eq(r0,#0); if (p0.new) jump:t .Lnext }
.Ldone:
{ r17:16 = memd(r29+#8); r19:18 = memd(r29+#0) }
{ dealloc_return }
