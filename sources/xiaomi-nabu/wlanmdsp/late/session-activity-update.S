// SPDX-License-Identifier: GPL-3.0-only
// Agent20-14: file [0x1026bc,0x102754), runtime [0xb00c26bc,0xb00c2754).
// Update live WLAN service activity under the same context lock.
// The function checks descriptor kind 1/2, compares the current controller
// identity and conditionally marks registration or increments a counter.
// The two-word far diagnostic jump follows the return but belongs to this
// bounded firmware interval and is not interpreted as a new function body.
.text
.globl agent14_late_session_activity_update
agent14_late_session_activity_update:
// +0x0000: 2 encoded words
{ r17 = r0; memd(r29+#-0x10) = r17:16; 	allocframe(#0x10) }
// +0x0008: 3 encoded words
{ call -533572; r0 = memw(gp+#0x78c); r1 = memub(r17+#0x0) }
// +0x0014: 1 encoded words
{ r16 = r0 }
// +0x0018: 2 encoded words
{ r2 = memw(r16+#0x788); if (!cmp.eq(r2.new,#0x1)) jump:t label_008c }
// +0x0020: 2 encoded words
{ r2 = memw(r17+#0x24); if (cmp.eq(r2.new,#0x2)) jump:nt label_0078 }
// +0x0028: 1 encoded words
{ p0 = cmp.eq(r2,#0x1); if (!p0.new) jump:nt label_008c }
// +0x002c: 3 encoded words
{ r1 = #0x3e11; r0 = ##0x65e8; 	r2 = memw(r17+#0x4) }
// +0x0038: 2 encoded words
{ call 1411776; memw(r29+#0x0) = r2 }
// +0x0040: 3 encoded words
{ call -785076; r0 = ##0xb02b7210; 	r1 = memw(r17+#0x4) }
// +0x004c: 1 encoded words
{ r2 = memw(r17+#0x4) }
// +0x0050: 2 encoded words
{ r3 = memw(r16+#0x60c); if (!cmp.eq(r3.new,r2)) jump:t label_008c }
// +0x0058: 3 encoded words
{ call label_0090; r0 = add(r17,#0x18); r1 = add(r16,#0x4) }
// +0x0064: 4 encoded words
{ p0 = cmp.eq(r0,#0x0); if (!p0.new) r2 = #0x1; if (!p0.new) memw(r16+##0x790) = r2.new }
// +0x0074: 1 encoded words
{ jump label_008c }
label_0078:
// +0x0078: 1 encoded words
{ r2 = memw(r17+#0x4) }
// +0x007c: 2 encoded words
{ r3 = memw(r16+#0x78c); if (!cmp.eq(r3.new,r2)) jump:t label_008c }
// +0x0084: 1 encoded words
{ r2 = add(r16,#0x794) }
// +0x0088: 1 encoded words
{ memw(r2+#0x0) += #0x1 }
label_008c:
// +0x008c: 1 encoded words
{ r17:16 = memd(r29+#0x8); 	dealloc_return }
label_0090:
// +0x0090: 2 encoded words
{ r2 = #0x6 ; jump ##-792140 }
