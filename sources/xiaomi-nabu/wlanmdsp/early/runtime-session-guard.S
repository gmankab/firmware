// SPDX-License-Identifier: GPL-3.0-only
// Agent13: reject null WLAN record and report nonzero session id.
// File [0x42d30,0x42d64), VMA [0xb0002d30,0xb0002d64).
.text
.globl agent13_runtime_session_guard
agent13_runtime_session_guard:
{ memd(r29+#-16) = r17:16; allocframe(#16) }
{ r16 = r0; if (cmp.eq(r16.new,#0)) jump:nt done }
{ r1 = #0x7611; r0 = ##0xd0; r2 = memw(r16+#0) }
{ call 0x21846c; memw(r29+#0) = r2 }
{ call -0x108; r0 = ##0xb02a6340; r1 = memw(r16+#0) }
done:
{ r0 = #0; r17:16 = memd(r29+#8); dealloc_return }
