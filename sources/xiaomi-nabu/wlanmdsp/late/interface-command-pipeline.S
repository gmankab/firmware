// SPDX-License-Identifier: GPL-3.0-only
// Recovered WLAN V66 packet semantics for original file [0x103b4c,0x104398).
// Validate tagged input records, import packed byte fields into 32-bit values,
// dispatch encoder diagnostics, and update interface status/counters.
// This reconstruction is at the instruction/packet level; exact original C
// function names and hardware behavior cannot be determined from byte equality.
// Independently reassemblable ISA mnemonics: no embedded OEM opcode words.
.text
.globl agent14_late_interface_command_pipeline
agent14_late_interface_command_pipeline:
// Prologue: allocate an output record and preserve the current interface context.
// file 0x103b4c; offset 0x0000, 2 encoded words
{ call 1973204; allocframe(#0xb0) }
// file 0x103b54; offset 0x0008, 3 encoded words
{ call -538836; r19 = r0; r21:20 = combine(r5,r4) }
// file 0x103b60; offset 0x0014, 4 encoded words
{ r2 = add(r29,#0x34); r16 = r0; r17 = #0x0; memw(r29+#0x34) = r16.new }
// file 0x103b70; offset 0x0024, 2 encoded words
{ memw(r2+#0x4) = ##-0x4ff3ca08 }
// Validate command type, descriptor pointer, signatures and callback.
// file 0x103b78; offset 0x002c, 3 encoded words
{ r2 = memb(r16+#0x0); if (cmp.eq(r2.new,#0x2)) jump:t label_0450 }
// file 0x103b84; offset 0x0038, 4 encoded words
{ r1 = #0x45d; r24 = memw(r16+#0x1b4); if (cmp.eq(r24.new,#0x0)) jump:nt label_0830 }
// file 0x103b94; offset 0x0048, 4 encoded words
{ r0 = add(r29,#0x3c); r1 = #0x0; r2 = #0x3c; r22 = memw(r29+#0xb8) }
// file 0x103ba4; offset 0x0058, 3 encoded words
{ call -793596; r17 = #0x0; r18 = add(r16,#0x4) }
// file 0x103bb0; offset 0x0064, 3 encoded words
{ call -5220; r1 = add(r22,#0xa); r0 = r18 }
// file 0x103bbc; offset 0x0070, 1 encoded words
{ p0 = cmp.eq(r0,#0x0); if (p0.new) jump:nt label_0450 }
// file 0x103bc0; offset 0x0074, 3 encoded words
{ r2 = memw(r22+#0x50); if (cmp.eq(r2.new,#0x0)) jump:nt ##label_0450 }
// file 0x103bcc; offset 0x0080, 1 encoded words
{ r2 = memh(r22+#0x8) }
// file 0x103bd0; offset 0x0084, 4 encoded words
{ p0 = cmp.eq(r2,##-0x7923); if (!p0.new) jump:t label_0450; if (p0.new) r2 = memb(r22+#0x2c) }
// file 0x103be0; offset 0x0094, 3 encoded words
{ p0 = cmp.eq(r2,#0x3a); if (!p0.new) jump:t label_0450; if (p0.new) r2 = memh(r22+#0x5a) }
// file 0x103bec; offset 0x00a0, 3 encoded words
{ p0 = cmp.eq(r2,#0x87); if (!p0.new) jump:t label_0450; if (p0.new) r2 = memub(r24+#0x3) }
// file 0x103bf8; offset 0x00ac, 2 encoded words
{ p0 = !tstbit(r2,#0x1); if (p0.new) jump:t label_0450 }
// file 0x103c00; offset 0x00b4, 3 encoded words
{ r2 = memb(r24+#0x1); if (cmp.eq(r2.new,#0x0)) jump:nt ##label_0450 }
// file 0x103c0c; offset 0x00c0, 4 encoded words
{ r1 = zxth(r21); r0 = r20; r5 = memw(##0xb0345730) }
// file 0x103c1c; offset 0x00d0, 3 encoded words
{ callr r5; r2 = #0x0; r3 = add(r29,#0x3c); r4 = r22 }
// file 0x103c28; offset 0x00dc, 3 encoded words
{ r17 = #0x1; p0 = cmp.eq(r0,#0x0); if (p0.new) jump:nt label_0450; if (!p0.new) r3 = memub(r29+#0x3c) }
// Convert packed little-endian byte fields to 32-bit record words.
// file 0x103c34; offset 0x00e8, 4 encoded words
{ r20 = ##-0x4fc9fef2; r0 = memub(r29+#0x42); r7 = memub(r29+#0x40) }
// file 0x103c44; offset 0x00f8, 2 encoded words
{ r8 = memub(r29+#0x41); r4 = memub(r29+#0x3d) }
// file 0x103c4c; offset 0x0100, 4 encoded words
{ r3 = insert(r4,#0x18,#0x8); r7 = insert(r8,#0x18,#0x8); r12 = memub(r29+#0x48); r5 = memub(r29+#0x3e) }
// file 0x103c5c; offset 0x0110, 4 encoded words
{ r3 |= asl(r5,#0x10); r7 |= asl(r0,#0x10); r1 = memub(r29+#0x49); r9 = memub(r29+#0x43) }
// file 0x103c6c; offset 0x0120, 4 encoded words
{ r7 |= asl(r9,#0x18); r12 = insert(r1,#0x18,#0x8); r26 = memub(r29+#0x45); r5 = memub(r29+#0x44) }
// file 0x103c7c; offset 0x0130, 3 encoded words
{ r5 = insert(r26,#0x18,#0x8); r6 = memub(r29+#0x3f); r0 = memub(r29+#0x4a) }
// file 0x103c88; offset 0x013c, 4 encoded words
{ r3 |= asl(r6,#0x18); r12 |= asl(r0,#0x10); r1 = memub(r29+#0x46); r0 = memub(r29+#0x4b) }
// file 0x103c98; offset 0x014c, 3 encoded words
{ r5 |= asl(r1,#0x10); r12 |= asl(r0,#0x18); r27 = memub(r29+#0x47) }
// file 0x103ca4; offset 0x0158, 3 encoded words
{ r1 = #0x3e15; r0 = ##0x6660;  r2 = memub(r16+#0x3) }
// file 0x103cb0; offset 0x0164, 3 encoded words
{ r5 |= asl(r27,#0x18); memw(r29+#0x0) = r2; memw(r29+#0x10) = r12 }
// file 0x103cbc; offset 0x0170, 1 encoded words
{ memw(r29+#0xc) = r5;  memw(r29+#0x8) = r7 }
// file 0x103cc0; offset 0x0174, 2 encoded words
{ call 1406196; memw(r29+#0x4) = r3 }
// file 0x103cc8; offset 0x017c, 2 encoded words
{ r7 = memub(r29+#0x40); r3 = memub(r29+#0x3c) }
// file 0x103cd0; offset 0x0184, 2 encoded words
{ r8 = memub(r29+#0x41); r4 = memub(r29+#0x3d) }
// file 0x103cd8; offset 0x018c, 4 encoded words
{ r3 = insert(r4,#0x18,#0x8); r7 = insert(r8,#0x18,#0x8); r1 = memub(r29+#0x42); r5 = memub(r29+#0x3e) }
// file 0x103ce8; offset 0x019c, 4 encoded words
{ r3 |= asl(r5,#0x10); r7 |= asl(r1,#0x10); r12 = memub(r29+#0x48); r9 = memub(r29+#0x44) }
// file 0x103cf8; offset 0x01ac, 2 encoded words
{ r5 = memub(r29+#0x45); r0 = memub(r29+#0x49) }
// file 0x103d00; offset 0x01b4, 4 encoded words
{ r12 = insert(r0,#0x18,#0x8); r9 = insert(r5,#0x18,#0x8); r27 = memub(r29+#0x46); r1 = memub(r29+#0x4a) }
// file 0x103d10; offset 0x01c4, 4 encoded words
{ r12 |= asl(r1,#0x10); r9 |= asl(r27,#0x10); r0 = memub(r29+#0x4b); r1 = memub(r29+#0x43) }
// file 0x103d20; offset 0x01d4, 4 encoded words
{ r12 |= asl(r0,#0x18); r7 |= asl(r1,#0x18); r0 = memub(r29+#0x47); r6 = memub(r29+#0x3f) }
// file 0x103d30; offset 0x01e4, 4 encoded words
{ r9 |= asl(r0,#0x18); r1:0 = combine(#0x5,##0xb02b7424); r2 = memub(r16+#0x3) }
// file 0x103d40; offset 0x01f4, 3 encoded words
{ r3 |= asl(r6,#0x18); memw(r29+#0x0) = r2; memw(r29+#0x10) = r12 }
// file 0x103d4c; offset 0x0200, 2 encoded words
{ memw(r29+#0xc) = r9; memw(r29+#0x8) = r7 }
// file 0x103d54; offset 0x0208, 2 encoded words
{ call -793156; memw(r29+#0x4) = r3 }
// file 0x103d5c; offset 0x0210, 4 encoded words
{ r4 = ##-0x76f07fef; r5 = memub(r29+#0x48); r2 = memub(r16+#0x3) }
// file 0x103d6c; offset 0x0220, 2 encoded words
{ r6 = memub(r29+#0x49); memw(r29+#0x0) = r4 }
// file 0x103d74; offset 0x0228, 3 encoded words
{ r5 = insert(r6,#0x18,#0x8); r1 = memub(r29+#0x4a); r3 = memuh(gp+#0x10e) }
// file 0x103d80; offset 0x0234, 4 encoded words
{ r5 |= asl(r1,#0x10); r1 = #0x2403; r2 = combine(r2.l,r3.l); r0 = memub(r29+#0x4b) }
// file 0x103d90; offset 0x0244, 3 encoded words
{ r5 |= asl(r0,#0x18); r0 = ##0x6668;  memw(r29+#0x4) = r2 }
// file 0x103d9c; offset 0x0250, 2 encoded words
{ call 1405976; memw(r29+#0x8) = r5 }
// file 0x103da4; offset 0x0258, 4 encoded words
{ r1 = ##-0x76f07fef; r3 = memub(r29+#0x48); r4 = memub(r16+#0x3) }
// file 0x103db4; offset 0x0268, 2 encoded words
{ r2 = memub(r29+#0x49); r5 = memuh(gp+#0x10e) }
// file 0x103dbc; offset 0x0270, 4 encoded words
{ r3 = insert(r2,#0x18,#0x8); r2 = combine(r4.l,r5.l); r6 = memub(r29+#0x4b); r0 = memub(r29+#0x4a) }
// file 0x103dcc; offset 0x0280, 3 encoded words
{ r3 |= asl(r0,#0x10); r0 = ##-0x4fd48bd0 }
// file 0x103dd8; offset 0x028c, 2 encoded words
{ r3 |= asl(r6,#0x18); call -792772 }
// file 0x103de0; offset 0x0294, 3 encoded words
{ r2 = add(r16,#0x7c8); r20 = add(r29,#0x24); memh(r20+#0x0) += #0x1 }
// file 0x103dec; offset 0x02a0, 1 encoded words
{ memw(r24+#0x8) += #0x1 }
// file 0x103df0; offset 0x02a4, 1 encoded words
{ memw(r2+#0x0) += #0x1 }
// file 0x103df4; offset 0x02a8, 2 encoded words
{ r23 = memub(r24+#0x1); if (cmp.eq(r23.new,#0x0)) jump:nt label_037c }
// file 0x103dfc; offset 0x02b0, 4 encoded words
{ r21 = add(r20,#0x2); r26 = add(r24,#0x1c); r25 = #-0x1; r27 = #0x0 }
// file 0x103e0c; offset 0x02c0, 2 encoded words
{ r2 = add(r29,#0x3c); r3 = memub(r29+#0x4c) }
// file 0x103e14; offset 0x02c8, 3 encoded words
{ p0 = cmpb.eq(r3,#0xff); r2 = add(r2,#0x10); memw(r29+#0x1c) = r2.new }
// file 0x103e20; offset 0x02d4, 2 encoded words
{ r0 = p0; memw(r29+#0x20) = r0.new }
label_02dc:
// file 0x103e28; offset 0x02dc, 2 encoded words
{ r22 = add(r26,mpyi(#0x1c,r27)); r2 = memw(r29+#0x20) }
// file 0x103e30; offset 0x02e4, 2 encoded words
{ p0 = r2; if (p0.new) jump:nt label_032c }
label_02ec:
// file 0x103e38; offset 0x02ec, 1 encoded words
{ r2 = memub(r22+#0x0) }
// file 0x103e3c; offset 0x02f0, 3 encoded words
{ r22 = +mpyi(r27,#0x1c); p0 = tstbit(r2,#0x2); if (p0.new) jump:nt label_083c }
// file 0x103e48; offset 0x02fc, 2 encoded words
{ r22 += add(r26,#0x4); p0 = or(p0,!p0) }
label_0304:
// file 0x103e50; offset 0x0304, 1 encoded words
{ if (!p0) jump:nt label_0320 }
// file 0x103e54; offset 0x0308, 3 encoded words
{ call -787672; r2 = #0x10; r1 = r22; r0 = add(r29,#0x3c) }
// file 0x103e60; offset 0x0314, 3 encoded words
{ p0 = and(p0,!p0); if (p1.new) r25 = add(r27,#0x0); p1 = cmp.eq(r0,#0x0); if (!p1.new) jump:t label_0304 }
label_0320:
// file 0x103e6c; offset 0x0320, 2 encoded words
{ p0 = cmp.gt(r25,#-0x1); if (!p0.new) jump:t label_0374 }
// file 0x103e74; offset 0x0328, 1 encoded words
{ jump label_0388 }
label_032c:
// file 0x103e78; offset 0x032c, 2 encoded words
{ call -794320; r2 = #0x9;  r1:0 = combine(#0,r21) }
// file 0x103e80; offset 0x0334, 4 encoded words
{ r2 = #0xff; r1 = add(r29,#0x24); r0 = memw(r29+#0x1c); memb(r29+#0x24) = r2.new }
// file 0x103e90; offset 0x0344, 2 encoded words
{ r2 = memub(r22+#0x11); memb(r29+#0x31) = r2.new }
// file 0x103e98; offset 0x034c, 2 encoded words
{ r2 = memub(r22+#0x12); memb(r29+#0x32) = r2.new }
// file 0x103ea0; offset 0x0354, 3 encoded words
{ r2 = #0x10; r3 = memub(r22+#0x13); memb(r20+#0x1) = #0x2 }
// file 0x103eac; offset 0x0360, 2 encoded words
{ memb(r29+#0x33) = r3; memb(r20+#0xb) = #0x1 }
// file 0x103eb4; offset 0x0368, 2 encoded words
{ call -787768; memb(r20+#0xc) = #-0x1 }
// file 0x103ebc; offset 0x0370, 1 encoded words
{ p0 = cmp.eq(r0,#0x0); if (p0.new) jump:nt label_02ec }
label_0374:
// file 0x103ec0; offset 0x0374, 2 encoded words
{ r27 = add(r27,#0x1); if (cmp.gt(r23,r27.new)) jump:t label_02dc }
label_037c:
// file 0x103ec8; offset 0x037c, 1 encoded words
{ r2 = add(r16,#0x7d0) }
// file 0x103ecc; offset 0x0380, 2 encoded words
{ jump label_0450; memw(r2+#0x0) += #0x1 }
label_0388:
// file 0x103ed4; offset 0x0388, 2 encoded words
{ r2 = add(r16,#0x7cc); memw(r24+#0x4) += #0x1 }
// file 0x103edc; offset 0x0390, 1 encoded words
{ memw(r2+#0x0) += #0x1 }
// file 0x103ee0; offset 0x0394, 3 encoded words
{ r2 = memb(r16+##0x787); if (cmp.eq(r2.new,#0x0)) jump:nt label_0458 }
// file 0x103eec; offset 0x03a0, 4 encoded words
{ call 1405640; r0 = ##0x6670; r1 = #0x2410 }
// file 0x103efc; offset 0x03b0, 3 encoded words
{ call -793572; r0 = ##-0x4fd48bc4 }
// file 0x103f08; offset 0x03bc, 4 encoded words
{ r0 = ##0x6678; r1 = #0x2403; r5 = memub(r29+#0x48) }
// file 0x103f18; offset 0x03cc, 4 encoded words
{ r4 = ##-0x76f07fee; r2 = memub(r16+#0x3); r6 = memub(r29+#0x49) }
// file 0x103f28; offset 0x03dc, 3 encoded words
{ r5 = insert(r6,#0x18,#0x8); r3 = memuh(gp+#0x10e); r6 = memub(r29+#0x4a) }
// file 0x103f34; offset 0x03e8, 4 encoded words
{ r5 |= asl(r6,#0x10); r2 = combine(r2.l,r3.l); r6 = memub(r29+#0x4b); memw(r29+#0x4) = r2.new }
// file 0x103f44; offset 0x03f8, 2 encoded words
{ r5 |= asl(r6,#0x18); memw(r29+#0x0) = r4 }
// file 0x103f4c; offset 0x0400, 2 encoded words
{ call 1405544; memw(r29+#0x8) = r5 }
// file 0x103f54; offset 0x0408, 4 encoded words
{ r0 = ##-0x4fd48bb8; r3 = memub(r29+#0x48); r4 = memub(r16+#0x3) }
// file 0x103f64; offset 0x0418, 2 encoded words
{ r1 = ##-0x76f07fee }
label_0420:
// file 0x103f6c; offset 0x0420, 2 encoded words
{ r2 = memub(r29+#0x49); r5 = memuh(gp+#0x10e) }
// file 0x103f74; offset 0x0428, 3 encoded words
{ r3 = insert(r2,#0x18,#0x8); r6 = memub(r29+#0x4b); r2 = memub(r29+#0x4a) }
// file 0x103f80; offset 0x0434, 2 encoded words
{ r3 |= asl(r2,#0x10); r2 = combine(r4.l,r5.l) }
// file 0x103f88; offset 0x043c, 2 encoded words
{ r3 |= asl(r6,#0x18); call -793204 }
// file 0x103f90; offset 0x0444, 2 encoded words
{ r2 = ##-0x4fc9fef2 }
// file 0x103f98; offset 0x044c, 1 encoded words
{ memh(r2+#0x0) += #0x1 }
label_0450:
// Shared normal exit after parsing; extended target is deliberate.
// file 0x103f9c; offset 0x0450, 2 encoded words
{ r0 = r17 ; jump ##1972320 }
label_0458:
// file 0x103fa4; offset 0x0458, 3 encoded words
{ call -4848; r0 = r19; r1 = memub(r16+#0x250) }
// file 0x103fb0; offset 0x0464, 2 encoded words
{ r17 = r0; if (cmp.eq(r17.new,#0x0)) jump:nt label_04c4 }
// file 0x103fb8; offset 0x046c, 3 encoded words
{ r3 = +mpyi(r25,#0x1c); r21 = add(r26,mpyi(#0x1c,r25)); r20 = memw(r17+#0x10);  r2 = memw(r16+#0xc) }
// file 0x103fc4; offset 0x0478, 2 encoded words
{ r4 = ##0x1000088;  memw(r20+#0xc) = r2 }
// file 0x103fcc; offset 0x0480, 2 encoded words
{ r1 = memub(r20+#0x1f); r0 = memub(r20+#0x1e) }
// file 0x103fd4; offset 0x0488, 3 encoded words
{ r0 |= asl(r1,#0x8); memb(r20+#0x6) = #0x0; memw(r20+#0x8) = r4 }
// file 0x103fe0; offset 0x0494, 2 encoded words
{ r5 = memub(r20+#0x1d); r19 = memub(r20+#0x1c) }
// file 0x103fe8; offset 0x049c, 2 encoded words
{ r19 |= asl(r5,#0x8); r3 = memub(r26+r3<<#0x0) }
// file 0x103ff0; offset 0x04a4, 2 encoded words
{ r19 |= asl(r0,#0x10); p0 = !tstbit(r3,#0x1) }
// file 0x103ff8; offset 0x04ac, 1 encoded words
{ if (!p0) jump:nt label_04dc }
// file 0x103ffc; offset 0x04b0, 1 encoded words
{ r2 = add(r29,#0x3c);  r1 = r18 }
// file 0x104000; offset 0x04b4, 3 encoded words
{ call -794320; r0 = add(r2,#0x30); r2 = #0x6 }
// file 0x10400c; offset 0x04c0, 1 encoded words
{ jump label_0510 }
label_04c4:
// file 0x104010; offset 0x04c4, 1 encoded words
{ call 303184 }
// file 0x104014; offset 0x04c8, 4 encoded words
{ call -6252; r17 = !cmp.eq(r0,#0x0); r1:0 = combine(#0x2,r16); r3:2 = combine(#0x1,#0x1) }
// file 0x104024; offset 0x04d8, 1 encoded words
{ jump label_0450 }
label_04dc:
// file 0x104028; offset 0x04dc, 2 encoded words
{ r2 = memw(r21+#0x14); memb(r29+#0x6c) = r2.new }
// file 0x104030; offset 0x04e4, 1 encoded words
{ r3 = memw(r21+#0x14);  r0 = memw(r21+#0x18) }
// file 0x104034; offset 0x04e8, 2 encoded words
{ r3 = lsr(r3,#0x8); memb(r29+#0x6d) = r3.new }
// file 0x10403c; offset 0x04f0, 2 encoded words
{ r3 = memuh(r21+#0x16); memb(r29+#0x6e) = r3.new }
// file 0x104044; offset 0x04f8, 2 encoded words
{ r3 = memub(r21+#0x17); memb(r29+#0x70) = r0 }
// file 0x10404c; offset 0x0500, 2 encoded words
{ r2 = memw(r21+#0x18); memb(r29+#0x6f) = r3 }
// file 0x104054; offset 0x0508, 2 encoded words
{ r2 = lsr(r2,#0x8); memb(r29+#0x71) = r2.new }
label_0510:
// Optional record-status callbacks. Order is required to retain duplex and extender.
// file 0x10405c; offset 0x0510, 3 encoded words
{ r4 = ##0xb0345730; r0 = add(r29,#0x3c); r2 = memw(r21+#0x0) }
// file 0x104068; offset 0x051c, 3 encoded words
{ r3 = extractu(r2,#0x1,#0x3); r1 = r19; r2 = #0x0; r4 = memw(r4+#0x4) }
// file 0x104074; offset 0x0528, 2 encoded words
{ r21 = add(r29,#0x3c); callr r4 }
// file 0x10407c; offset 0x0530, 3 encoded words
{ r5 = add(r20,#0x20); r2 = memub(r20+#0x1f); r3 = memub(r20+#0x1e) }
// file 0x104088; offset 0x053c, 3 encoded words
{ r3 |= asl(r2,#0x8); r4 = memub(r20+#0x1d); r19 = memub(r20+#0x1c) }
// file 0x104094; offset 0x0548, 3 encoded words
{ r19 |= asl(r4,#0x8); r2 = memub(r20+#0x1b); r1 = memub(r20+#0x1a) }
// file 0x1040a0; offset 0x0554, 2 encoded words
{ r19 |= asl(r3,#0x10); r1 |= asl(r2,#0x8) }
// file 0x1040a8; offset 0x055c, 2 encoded words
{ r2 = add(r19,#-0xe); r3 = add(r1,r0) }
// file 0x1040b0; offset 0x0564, 4 encoded words
{ r4 = lsr(r3,#0x8); p0 = !cmp.gtu(r5,r2); memb(r20+#0x1a) = r3; if (p0.new) memb(r20+#0x1c) = r2 }
// file 0x1040c0; offset 0x0574, 3 encoded words
{ if (p0) r1 = add(r21,#0x36); if (!p0) jump:nt label_0844; memb(r20+#0x1b) = r4 }
// file 0x1040cc; offset 0x0580, 3 encoded words
{ r4 = lsr(r2,#0x18); r7 = lsr(r2,#0x10); r3 = add(r3,#0xe) }
// file 0x1040d8; offset 0x058c, 2 encoded words
{ r6 = lsr(r2,#0x8); r2 = #0x6;  r0 = r2 }
// file 0x1040e0; offset 0x0594, 3 encoded words
{ r3 = lsr(r3,#0x8); memb(r20+#0x1a) = r3; memb(r20+#0x1f) = r4 }
// file 0x1040ec; offset 0x05a0, 2 encoded words
{ memb(r20+#0x1e) = r7; memb(r20+#0x1d) = r6 }
// file 0x1040f4; offset 0x05a8, 2 encoded words
{ call -794564; memb(r20+#0x1b) = r3 }
// file 0x1040fc; offset 0x05b0, 3 encoded words
{ call -794572; r0 = add(r19,#-0x8); r2 = #0x6;  r1 = r18 }
// file 0x104108; offset 0x05bc, 4 encoded words
{ r2 = #0xdd; r0 = #0x86; r4 = add(r29,#0x34); memb(r19+#-0x2) = r0.new }
// Keep the load/store packet non-shuffled to preserve store-before-load.
// file 0x104118; offset 0x05cc, 4 encoded words
{ r0 = r16; r3 = r17; memb(r19+#-0x1) = r2; r1 = memw(r16+#0xc) }:mem_noshuf
// file 0x104128; offset 0x05dc, 1 encoded words
{ r2 = memub(r1+#0xc) }
// file 0x10412c; offset 0x05e0, 3 encoded words
{ p0 = !tstbit(r2,#0x1); r2 = #0x10; if (!p0.new) memb(r20+#0x4) = #0x0 }
// file 0x104138; offset 0x05ec, 4 encoded words
{ call -5224; if (!p0) r2 = memub(##0xb035bdb0); if (!p0) r1 = memw(r16+#0xc) }
// file 0x104148; offset 0x05fc, 1 encoded words
{ p0 = cmp.eq(r0,#0x0); if (p0.new) jump:nt label_0690 }
// file 0x10414c; offset 0x0600, 4 encoded words
{ call 1405032; r0 = ##0x6690; r1 = #0x3e40 }
// file 0x10415c; offset 0x0610, 3 encoded words
{ call -794180; r0 = ##-0x4fd48b94 }
// file 0x104168; offset 0x061c, 3 encoded words
{ call -6592; r1:0 = combine(#0x2,r16); r17 = #0x1;  r3:2 = combine(#2,#0x1) }
// file 0x104174; offset 0x0628, 4 encoded words
{ r0 = ##0x6698; r1 = #0x2403; r5 = memub(r29+#0x48) }
// file 0x104184; offset 0x0638, 4 encoded words
{ r4 = ##-0x76f07fec; r2 = memub(r16+#0x3); r6 = memub(r29+#0x49) }
// file 0x104194; offset 0x0648, 3 encoded words
{ r5 = insert(r6,#0x18,#0x8); r3 = memuh(gp+#0x10e); r6 = memub(r29+#0x4a) }
// file 0x1041a0; offset 0x0654, 4 encoded words
{ r5 |= asl(r6,#0x10); r2 = combine(r2.l,r3.l); r6 = memub(r29+#0x4b); memw(r29+#0x4) = r2.new }
// file 0x1041b0; offset 0x0664, 2 encoded words
{ r5 |= asl(r6,#0x18); memw(r29+#0x0) = r4 }
// file 0x1041b8; offset 0x066c, 2 encoded words
{ call 1404924; memw(r29+#0x8) = r5 }
// file 0x1041c0; offset 0x0674, 4 encoded words
{ r0 = ##-0x4fd48b88; r3 = memub(r29+#0x48); r4 = memub(r16+#0x3) }
// file 0x1041d0; offset 0x0684, 3 encoded words
{ r1 = ##-0x76f07fec; jump label_0420 }
label_0690:
// file 0x1041dc; offset 0x0690, 3 encoded words
{ r17 = #0x1; memb(r16+##0x787) = r17.new }
// file 0x1041e8; offset 0x069c, 1 encoded words
{ memw(r24+#0xc) += #0x1 }
// file 0x1041ec; offset 0x06a0, 2 encoded words
{ r3 = memub(r29+#0x5f); r1 = memub(r29+#0x6b) }
// file 0x1041f4; offset 0x06a8, 2 encoded words
{ r4 = memub(r29+#0x5e); r9 = memub(r29+#0x6a) }
// file 0x1041fc; offset 0x06b0, 4 encoded words
{ r4 |= asl(r3,#0x8); r9 |= asl(r1,#0x8); r3 = memub(r29+#0x63); r5 = memub(r29+#0x5d) }
// file 0x10420c; offset 0x06c0, 3 encoded words
{ r1 = #0x3e15; r7 = memub(r29+#0x62); r6 = memub(r29+#0x5c) }
// file 0x104218; offset 0x06cc, 4 encoded words
{ r6 |= asl(r5,#0x8); r7 |= asl(r3,#0x8); r0 = memub(r29+#0x61); r12 = memub(r29+#0x69) }
// file 0x104228; offset 0x06dc, 3 encoded words
{ r6 |= asl(r4,#0x10); r8 = memub(r29+#0x60); r13 = memub(r29+#0x68) }
// file 0x104234; offset 0x06e8, 4 encoded words
{ r8 |= asl(r0,#0x8); r13 |= asl(r12,#0x8); r3 = memub(r29+#0x67); r2 = memub(r16+#0x3) }
// file 0x104244; offset 0x06f8, 4 encoded words
{ r13 |= asl(r9,#0x10); r8 |= asl(r7,#0x10); r4 = memub(r29+#0x66); memw(r29+#0x0) = r2 }
// file 0x104254; offset 0x0708, 3 encoded words
{ r4 |= asl(r3,#0x8); r0 = memub(r29+#0x65); memw(r29+#0x10) = r13 }
// file 0x104260; offset 0x0714, 2 encoded words
{ r27 = memub(r29+#0x64); memw(r29+#0x8) = r8 }
// file 0x104268; offset 0x071c, 3 encoded words
{ r27 |= asl(r0,#0x8); r0 = ##0x6680;  memw(r29+#0x4) = r6 }
// file 0x104274; offset 0x0728, 3 encoded words
{ r27 |= asl(r4,#0x10); call 1404736; memw(r29+#0xc) = r27.new }
// file 0x104280; offset 0x0734, 2 encoded words
{ r3 = memub(r29+#0x5f); r1 = memub(r29+#0x6b) }
// file 0x104288; offset 0x073c, 2 encoded words
{ r4 = memub(r29+#0x5e); r9 = memub(r29+#0x6a) }
// file 0x104290; offset 0x0744, 4 encoded words
{ r4 |= asl(r3,#0x8); r9 |= asl(r1,#0x8); r3 = memub(r29+#0x63); r5 = memub(r29+#0x5d) }
// file 0x1042a0; offset 0x0754, 2 encoded words
{ r7 = memub(r29+#0x62); r6 = memub(r29+#0x5c) }
// file 0x1042a8; offset 0x075c, 4 encoded words
{ r6 |= asl(r5,#0x8); r7 |= asl(r3,#0x8); r0 = memub(r29+#0x61); r12 = memub(r29+#0x69) }
// file 0x1042b8; offset 0x076c, 3 encoded words
{ r6 |= asl(r4,#0x10); r8 = memub(r29+#0x60); r13 = memub(r29+#0x68) }
// file 0x1042c4; offset 0x0778, 4 encoded words
{ r8 |= asl(r0,#0x8); r13 |= asl(r12,#0x8); r3 = memub(r29+#0x67); r2 = memub(r16+#0x3) }
// file 0x1042d4; offset 0x0788, 4 encoded words
{ r13 |= asl(r9,#0x10); r8 |= asl(r7,#0x10); r4 = memub(r29+#0x66); memw(r29+#0x0) = r2 }
// file 0x1042e4; offset 0x0798, 3 encoded words
{ r4 |= asl(r3,#0x8); r0 = memub(r29+#0x65); memw(r29+#0x10) = r13 }
// file 0x1042f0; offset 0x07a4, 2 encoded words
{ r27 = memub(r29+#0x64); memw(r29+#0x8) = r8 }
// file 0x1042f8; offset 0x07ac, 4 encoded words
{ r27 |= asl(r0,#0x8); r1:0 = combine(#0x5,##0xb02b7454); memw(r29+#0x4) = r6 }
// file 0x104308; offset 0x07bc, 3 encoded words
{ r27 |= asl(r4,#0x10); call -794616; memw(r29+#0xc) = r27.new }
// file 0x104314; offset 0x07c8, 4 encoded words
{ r0 = ##0x6688; r1 = #0x2403; r5 = memub(r29+#0x48) }
// file 0x104324; offset 0x07d8, 4 encoded words
{ r4 = ##-0x76f07fed; r2 = memub(r16+#0x3); r6 = memub(r29+#0x49) }
// file 0x104334; offset 0x07e8, 3 encoded words
{ r5 = insert(r6,#0x18,#0x8); r3 = memuh(gp+#0x10e); r6 = memub(r29+#0x4a) }
// file 0x104340; offset 0x07f4, 4 encoded words
{ r5 |= asl(r6,#0x10); r2 = combine(r2.l,r3.l); r6 = memub(r29+#0x4b); memw(r29+#0x4) = r2.new }
// file 0x104350; offset 0x0804, 2 encoded words
{ r5 |= asl(r6,#0x18); memw(r29+#0x0) = r4 }
// file 0x104358; offset 0x080c, 2 encoded words
{ call 1404508; memw(r29+#0x8) = r5 }
// file 0x104360; offset 0x0814, 4 encoded words
{ r0 = ##-0x4fd48ba0; r3 = memub(r29+#0x48); r4 = memub(r16+#0x3) }
// file 0x104370; offset 0x0824, 3 encoded words
{ r1 = ##-0x76f07fed; jump label_0420 }
label_0830:
// file 0x10437c; offset 0x0830, 3 encoded words
{ call -5820; r0 = ##-0x4fd48d41 }
label_083c:
// file 0x104388; offset 0x083c, 2 encoded words
{ r1 = #0x4a4; jump label_0830 }
label_0844:
// file 0x104390; offset 0x0844, 2 encoded words
{ r1 = #0x4f6; jump label_0830 }
