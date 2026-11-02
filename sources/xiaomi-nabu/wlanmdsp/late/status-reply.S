// SPDX-License-Identifier: GPL-3.0-only
// Agent20-14: file [0x102280,0x102368), runtime [0xb00c2280,0xb00c2368).
// Send a short status reply and call the interface handler.
// Typed Hexagon V66 packet operations, not opaque OEM instructions.
.text
.globl agent14_late_status_reply
agent14_late_status_reply:
// +0x0: 2 encoded words
{ call 0x1e34f8; allocframe(#32) }
// +0x8: 3 encoded words
{ r0 = ##25992; r18 = r0; r16 = r1 }
// +0x14: 3 encoded words
{ r1 = #9236; r17 = r2; r3 = memw(r16+#4); r4 = memw(r16+#8) }
// +0x20: 1 encoded words
{ memw(r29+#12) = r4; memw(r29+#8) = r3 }
// +0x24: 2 encoded words
{ call 0x158f10; memw(r29+#0) = r18; memw(r29+#4) = r17 }
// +0x2c: 3 encoded words
{ r1:0 = combine(#4,##-1339330220); r2 = memw(r16+#4); r3 = memw(r16+#8) }
// +0x38: 1 encoded words
{ memw(r29+#12) = r3; memw(r29+#8) = r2 }
// +0x3c: 2 encoded words
{ call -786348; memw(r29+#4) = r17; memw(r29+#0) = r18 }
// +0x44: 1 encoded words
{ p0 = cmp.eq(r18,#3); if (!p0.new) jump:t label_0e4 }
// +0x48: 4 encoded words
{ r1 = r17; r18 = ##26000; r0 = memw(gp+#1932) }
// +0x58: 3 encoded words
{ call -532568; r17 = ##-1339330208 }
// +0x64: 1 encoded words
{ r19 = r0 }
// +0x68: 1 encoded words
{ r2 = memuh(r19+#28) }
// +0x6c: 1 encoded words
{ p0 = tstbit(r2,#0); if (!p0.new) jump:t label_0cc }
// +0x70: 2 encoded words
{ r2 = memb(r19+#0); if (!cmp.eq(r2.new,#1)) jump:t label_0cc }
// +0x78: 1 encoded words
{ r2 = memw(r19+#20); r0 = memw(r19+#44) }
// +0x7c: 1 encoded words
{ r17 = memw(r16+#4); r16 = memw(r16+#8) }
// +0x80: 2 encoded words
{ call 0x13ea3c; r18 = memuh(r2+#68) }
// +0x88: 3 encoded words
{ r2 = +mpyi(r18,##3072); r4 = #1; r3 = r0 }
// +0x94: 1 encoded words
{ r1:0 = combine(r17,r19) }
// +0x98: 4 encoded words
{ r3 += add(r2,##9216); call -1204; r2 = r16 }
// +0xa8: 3 encoded words
{ p0 = cmp.eq(r0,#0); if (!p0.new) r17 = ##-1339330184 }
// +0xb4: 4 encoded words
{ if (!p0) r18 = ##26016; if (p0) r17 = ##-1339330196 }
// +0xc4: 2 encoded words
{ if (p0) r18 = ##26008 }
label_0cc:
// +0xcc: 3 encoded words
{ call 0x158e68; r1:0 = combine(##9232,r18) }
// +0xd8: 2 encoded words
{ call 0x1e3494; r0 = r17 }
// +0xe0: 1 encoded words
{ jump -786504 }
label_0e4:
// +0xe4: 1 encoded words
{ jump 0x1e34f0 }
