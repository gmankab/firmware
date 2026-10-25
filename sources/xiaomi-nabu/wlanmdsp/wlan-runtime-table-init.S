// SPDX-License-Identifier: GPL-3.0-only
// WLAN runtime table initialization at [0xb00021a0,0xb0002208).
// Initializes shared QuRT globals, performs one runtime initialization call,
// maintains two counters and returns the initialized state or a debug path.
// The original memory-store/load ordering requires Hexagon :mem_noshuf.
// LLVM6 supports this packet modifier under hexagonv65, not hexagonv62.
.text
.globl wlan_runtime_table_init
wlan_runtime_table_init:
{ r1:0 = combine(#0,##2959748532); r3 = memw(gp+#1932); allocframe(#8) }
{ call -520; r2 = #76; memd(r29+#0) = r17:16; r16 = memw(r3+#688) }:mem_noshuf
{ r5 = #1; r1:0 = combine(#1,r16); r3:2 = combine(##-1342168568,#1) }
{ r4 = ##-1335218764; memw(##2959748536) = r5 }
{ call 0x2046c; memw(##2959748540) = r5 }
{ r16 = r0; if (!cmp.eq(r16.new,#0)) jump:t ready }
{ call 0x1d2e18; r1:0 = combine(#0,##2952799040) }
ready:
{ r0 = r16; r17:16 = memd(r29+#0); dealloc_return }
