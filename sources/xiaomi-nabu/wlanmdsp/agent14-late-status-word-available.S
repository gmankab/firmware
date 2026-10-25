// SPDX-License-Identifier: GPL-3.0-only
// Agent20-14 verified WLAN function: file [0x101848,0x101870)
// runtime [0xb00c1848,0xb00c1870).
// Reconstructed Hexagon packet instructions (not OEM opcode arrays).
.text
.globl agent14_late_status_word_available
agent14_late_status_word_available:
// +0x0 (1 words)
{ memd(r29+#-16) = r17:16; allocframe(#8) }
// +0x4 (2 words)
{ r0 = #0; r16 = memw(r0+#1344) }
// +0xc (2 words)
{ p0 = cmp.eq(r16,#0); if (p0.new) jump:nt L_024; if (!p0.new) r2 = memub(r16+#2) }
// +0x14 (1 words)
{ p0 = tstbit(r2,#0); if (!p0.new) jump:t L_024 }
// +0x18 (1 words)
{ call 0x15556c }
// +0x1c (1 words)
{ r2 = memw(r16+#8) }
// +0x20 (1 words)
{ r0 = sub(r2,r0) }
L_024:
// +0x24 (1 words)
{ r17:16 = memd(r29+#0); dealloc_return }
