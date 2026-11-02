// SPDX-License-Identifier: GPL-3.0-only
// Agent 20-14: file [0x100240,0x100410),
// runtime [0xb00c0240,0xb00c0410).
// Acquire a descriptor, pack byte fields, adjust lengths, and submit it.
// Length assertions and allocation/error paths retain their original calls.
// The callee at 0xb021cfc4 is a terminal assertion path in this recovered CFG;
// its implementation and non-returning behavior are not reconstructed here.
// Source order below deliberately controls duplex selection. Packet operands
// read old values unless marked .new, even when the same register is written.
// Explicit V66 mnemonic/packet source; no reference bytes are used at build time.
.text
.globl wlan_late_descriptor_submit
wlan_late_descriptor_submit:
{ call 0x1e5528; allocframe(#0x20) }
{ r1 = #0x1; r19 = r1; r16 = r0 }
{ r2 = memw(r16+#0x0); r3 = memw(r16+#0x4) }
{ r2 = memw(r2+#0x0) }
{ r2 = addasl(r2,r3,#0x2) }
// r17 reads the old r2 table pointer; zero is a later call argument.
{ r2 = #0x0; r17 = memw(r2+#0x54) }
{ call 0x1593dc; r0 = memw(r17+#0x28); r3 = memub(r17+#0x250) }
{ r18 = r0; if (cmp.eq(r18.new,#0x0)) jump:nt allocation_failed }
{ r1 = add(r16,#0x38); r20 = memw(r18+#0x10) }
{ r2 = memub(r20+#0x1f); memw(r20+#0x8) = #0x1 }
{ r3 = memub(r20+#0x1e); memb(r20+#0x4) = #0x3 }
{ r3 |= asl(r2,#0x8); r4 = memub(r20+#0x1d); memb(r20+#0x1) = #0x4 }
{ r5 = memw(r17+#0xc); memw(r20+#0xc) = r5.new }
{ r0 = memub(r20+#0x1c); r2 = memw(r16+#0x1c) }
{ r0 |= asl(r4,#0x8); r2 = add(r2,#-0x18) }
{ r0 |= asl(r3,#0x10); call -0xbe184 }
{ r2 = memw(r16+#0x1c) }
{ r2 = add(r2,##0xffe8) }
{ p0 = cmph.gtu(r2,##0x300); if (p0.new) jump:nt assert_length; if (!p0.new) r3 = memub(r20+#0x1e) }
{ r4 = memub(r20+#0x1f); memb(r20+#0x1a) = r2 }
{ r5 = memub(r20+#0x1d); r6 = memub(r20+#0x1c) }
{ r3 |= asl(r4,#0x8); r6 |= asl(r5,#0x8) }
{ r6 |= asl(r3,#0x10); r3 = add(r20,#0x20) }
{ r0 = add(r6,#-0x18) }
// Compare the OLD buffer+0x20 pointer, while the byte store uses r3.new.
{ r3 = lsr(r2,#0x8); p0 = !cmp.gtu(r3,r0); if (!p0.new) jump:nt assert_capacity; memb(r20+#0x1b) = r3.new }
{ r3 = lsr(r0,#0x18); r4 = lsr(r0,#0x8); r2 = add(r2,#0x18); memb(r20+#0x1f) = r3.new }
// The low length byte store reads OLD r2 before the simultaneous shift.
{ r3 = lsr(r0,#0x10); r2 = lsr(r2,#0x8); memb(r20+#0x1d) = r4; memb(r20+#0x1a) = r2 }
// The high length byte store reads OLD r2 before pointer reassignment.
{ r2 = add(r17,#0x4); r5:4 = combine(r19,#0x0); memb(r20+#0x1b) = r2; memb(r20+#0x1c) = r0 }
{ r19 = #0x0; memb(r20+#0x1e) = r3; r1 = memuh(r17+#0x18) }:mem_noshuf
{ r1 = and(##0xfff0,asl(r1,#0x4)); memw(r29+#0x4) = r2; memw(r29+#0x0) = r2 }
{ call -0x491ac; r3:2 = combine(#0x50,#0x0) }
{ r1 = #0x3; r2 = memuh(r17+#0x18); r0 = memw(r17+#0x2c) }
{ r2 = add(r2,#0x1) }
{ r2 = extractu(r2,#0xb,#0x0); call 0xa3058; memh(r17+#0x18) = r2.new }
{ r5 = #0x0; r1:0 = combine(#0x0,r17); r3:2 = combine(r18,#0x13); r4 = r0 }
{ r4 = or(##0x200003,asl(r4,#0x8)); call -0xb1c7c }
{ r17 = r0; r2 = memw(r16+#0x224); if (cmp.eq(r17.new,#0x0)) jump:nt count_success }
{ r1 = #0x2411; r0 = ##0x64a0; memw(r29+#0x0) = r2 }
{ call 0x15ae0c }
{ call -0xbd764; r0 = ##-0x4fd490e8; r1 = memw(r16+#0x224) }
{ r19 = r17; jump restore_return }
allocation_failed:
{ call 0x15adf4; r0 = ##0x6498; r1 = #0x2410 }
{ call -0xbe0b8; r0 = ##0xb02b6ec0; r19 = #-0x1 }
{ jump restore_return }
count_success:
{ r2 = add(r2,#0x1); memw(r16+#0x224) = r2.new }
restore_return:
{ r0 = r19; jump ##0x1e545c }
assert_length:
{ r2 = #0x56; jump report_assertion }
assert_capacity:
{ r2 = #0x58 }
report_assertion:
{ r0 = ##-0x4fd49134; r1 = ##-0x4fd49116 }
{ call 0x15cbb8 }
