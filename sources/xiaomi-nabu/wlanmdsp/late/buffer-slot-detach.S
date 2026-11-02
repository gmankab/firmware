// SPDX-License-Identifier: GPL-3.0-only
// File [0x1056b4,0x1056e0): detach a shared buffer descriptor safely.
.text
.globl agent14_late_buffer_slot_detach
agent14_late_buffer_slot_detach:
{ r16 = r0; memd(r29+#-0x10) = r17:16; allocframe(#0x8) }
{ r2 = memw(r16+#0x1b4); if (cmp.eq(r2.new,#0x0)) jump:nt detached }
{ memb(r2+#0x0) = #-0x1; r0 = memw(gp+#0x7d4) }:mem_noshuf
{ call 0x15c978; r1 = memw(r16+#0x1b4) }
{ r2 = #0x0; memw(r16+#0x1b4) = r2.new }
detached:
{ r17:16 = memd(r29+#0x0); dealloc_return }
