// SPDX-License-Identifier: GPL-3.0-only
// File [0x10567c,0x1056b4): lazily attach a shared buffer descriptor.
.text
.globl agent14_late_buffer_slot_attach
agent14_late_buffer_slot_attach:
{ r16 = r0; memd(r29+#-0x10) = r17:16; allocframe(#0x8) }
{ r0 = memw(r16+#0x1b4); if (cmp.eq(r0.new,#0x0)) jump:nt attached_null }
{ jump attached_done; r2 = memub(r16+#0x3); memb(r0+#0x0) = r2.new }
attached_null:
{ call 0x15ca94; r0 = memw(gp+#0x7d4) }
{ p0 = cmp.eq(r0,#0x0); if (!p0.new) r2 = memub(r16+#0x3); memw(r16+#0x1b4) = r0 }
{ if (!p0) memb(r0+#0x0) = r2 }
attached_done:
{ r17:16 = memd(r29+#0x0); dealloc_return }
