// SPDX-License-Identifier: GPL-3.0-only
// Agent20-14: file [0x101a64,0x101b20), runtime [0xb00c1a64,0xb00c1b20).
// Probe descriptor mode and conditionally enable a descriptor through global state.
// Typed Hexagon V66 packet operations, not opaque OEM instructions.
.text
.globl agent14_late_session_descriptor_probe
agent14_late_session_descriptor_probe:
// +0x0: 3 encoded words
{ r16 = r1; r17 = r0; memd(r29+#-16) = r17:16; allocframe(#32) }
// +0xc: 3 encoded words
{ r1 = #9236; r0 = ##25936; r3 = memw(r16+#8) }
// +0x18: 1 encoded words
{ r2 = memw(r16+#4); r4 = memw(r16+#12) }
// +0x1c: 1 encoded words
{ memw(r29+#12) = r4; memw(r29+#8) = r3 }
// +0x20: 1 encoded words
{ memw(r29+#4) = r2; memw(r29+#0) = r17 }
// +0x24: 2 encoded words
{ call 0x15972c; memw(r29+#20) = #0 }
// +0x2c: 3 encoded words
{ r1:0 = combine(#4,##-1339330388); r3 = memw(r16+#8); r2 = memw(r16+#4) }
// +0x38: 2 encoded words
{ r4 = memw(r16+#12); memw(r29+#12) = r4.new }
// +0x40: 1 encoded words
{ memw(r29+#8) = r3; memw(r29+#4) = r2 }
// +0x44: 2 encoded words
{ call -784280; memw(r29+#0) = r17 }
// +0x4c: 1 encoded words
{ p0 = cmp.eq(r17,#4); if (!p0.new) jump:t label_0b8 }
// +0x50: 3 encoded words
{ call -530484; r0 = memw(gp+#1932); r1 = memw(r16+#4) }
// +0x5c: 3 encoded words
{ r4 = r0; r5 = ##-1341384132; r2 = memw(r16+#8) }
// +0x68: 3 encoded words
{ call -604; r1:0 = combine(r4,#2); r4 = add(r29,#20); r3 = memw(r16+#12) }
// +0x74: 1 encoded words
{ p0 = cmp.eq(r0,#0); if (p0.new) jump:nt label_09c }
// +0x78: 3 encoded words
{ r1 = #9233; r0 = ##25952; r16 = memw(r29+#20) }
// +0x84: 2 encoded words
{ call 0x1596cc; memw(r29+#0) = r16 }
// +0x8c: 3 encoded words
{ call -781992; r1:0 = combine(r16,##-1339330364) }
// +0x98: 1 encoded words
{ jump label_0b8 }
label_09c:
// +0x9c: 4 encoded words
{ call 0x1596b4; r0 = ##25944; r1 = #9232 }
// +0xac: 3 encoded words
{ call -784376; r0 = ##-1339330376 }
label_0b8:
// +0xb8: 1 encoded words
{ r17:16 = memd(r29+#24); dealloc_return }
