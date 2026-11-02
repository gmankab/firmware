// SPDX-License-Identifier: GPL-3.0-only
// Agent 20-14: file [0x100eb0, 0x1012a0),
// runtime [0xb00c0eb0, 0xb00c12a0).
// Dispatch tags 0x1002..0x106a through the 105-entry table at 0xb02b6d1c.
// tools/build-wlanmdsp-late-tables.py derives the table as separate DATA from
// these case labels; this source emits only the executable handler region.
// Handlers enforce fixed lengths or retain pointer/length pairs. Two bounded
// arrays admit at most 10 entries. Unhandled tags return zero without writes.
// Explicit V66 mnemonic/packet source; no reference bytes are used at build time.
.text
.globl wlan_late_wide_type_dispatch
wlan_late_wide_type_dispatch:
{ r4 = lsr(r1,#0x8); r16 = r3; memd(r29+#-0x10) = r17:16; allocframe(#0x10) }
{ r3 = and(r1,#0xff); p0 = cmp.eq(r4,#0x10); if (!p0.new) jump:t return_success }
{ r3 = add(r3,#-0x2) }
{ p0 = cmp.gtu(r3,#0x68); if (p0.new) jump:nt return_success }
// The dispatch load uses OLD r3 (the normalized tag) while r3 becomes 0.
{ r3 = #0x0; r4 = memw(r3<<#0x2+##0xb02b6d1c) }
{ jumpr r4 }
type_1002:
{ r3 = #-0x1; p0 = cmp.eq(r16,#0x2); if (!p0.new) jump:t restore_status }
{ jump return_success; memw(r0+#0x38) = r2 }
type_1003:
{ r3 = #-0x1; p0 = cmp.eq(r16,#0x2); if (!p0.new) jump:t restore_status }
{ jump return_success; memw(r0+#0x78) = r2 }
type_1004:
{ r3 = #-0x1; p0 = cmp.eq(r16,#0x2); if (!p0.new) jump:t restore_status }
{ jump return_success; memw(r0+#0x1c) = r2 }
type_1005:
{ r3 = #-0x1; p0 = cmp.eq(r16,#0x8); if (!p0.new) jump:t restore_status }
{ jump return_success; memw(r0+#0x50) = r2 }
type_1008:
{ r3 = #-0x1; p0 = cmp.eq(r16,#0x2); if (!p0.new) jump:t restore_status }
{ jump return_success; memw(r0+#0x28) = r2 }
type_1009:
{ r3 = #-0x1; p0 = cmp.eq(r16,#0x2); if (!p0.new) jump:t restore_status }
{ jump return_success; memw(r0+#0x3c) = r2 }
type_100d:
{ r3 = #-0x1; p0 = cmp.eq(r16,#0x1); if (!p0.new) jump:t restore_status }
{ jump return_success; memw(r0+#0x24) = r2 }
type_100e:
{ r4 = add(r0,#0x160); r3 = memw(r0+#0x160); if (cmp.gtu(r3.new,#0x9)) jump:t ##return_success }
{ r3 = addasl(r0,r3,#0x2) }
// The pointer store uses OLD r3 (the indexed output address), while
// r3 becomes the success result. Preserve the store/load slot order.
{ r3 = #0x0; memw(r3+#0x110) = r2; r2 = memw(r0+#0x160) }:mem_noshuf
{ r2 = addasl(r0,r2,#0x2) }
{ memw(r2+#0x138) = r16 }
{ jump restore_status; memw(r4+#0x0) += #0x1 }
type_100f:
{ r3 = #-0x1; p0 = cmp.eq(r16,#0x2); if (!p0.new) jump:t restore_status }
{ jump return_success; memw(r0+#0x7c) = r2 }
type_1010:
{ r3 = #-0x1; p0 = cmp.eq(r16,#0x2); if (!p0.new) jump:t restore_status }
{ jump return_success; memw(r0+#0x20) = r2 }
type_1011:
{ jump return_success; memw(r0+#0xd0) = r2; memw(r0+#0xd4) = r16 }
type_1012:
{ r3 = #-0x1; p0 = cmp.eq(r16,#0x2); if (!p0.new) jump:t restore_status }
{ jump return_success; memw(r0+#0x40) = r2 }
type_1014:
{ p0 = cmp.eq(r16,#0x20); r3 = #-0x1; if (!p0.new) jump:t restore_status; if (p0.new) memw(r0+#0x5c) = r2 }
{ jump return_success }
type_1015:
{ p0 = cmp.eq(r16,#0x20); r3 = #-0x1; if (!p0.new) jump:t restore_status; if (p0.new) memw(r0+#0x60) = r2 }
{ jump return_success }
type_1016:
{ r3 = #-0x1; p0 = cmp.eq(r16,#0x10); if (!p0.new) jump:t restore_status }
{ jump return_success; memw(r0+#0x6c) = r2 }
type_1017:
{ r3 = #-0x1; p0 = cmp.eq(r16,#0x10); if (!p0.new) jump:t restore_status }
{ jump return_success; memw(r0+#0x70) = r2 }
type_1018:
{ jump return_success; memw(r0+#0xe0) = r2; memw(r0+#0xe4) = r16 }
type_101a:
{ r3 = #-0x1; p0 = cmp.eq(r16,#0x10); if (!p0.new) jump:t restore_status }
{ jump return_success; memw(r0+#0xc) = r2 }
type_101e:
{ r3 = #-0x1; p0 = cmp.eq(r16,#0x8); if (!p0.new) jump:t restore_status }
{ jump return_success; memw(r0+#0x74) = r2 }
type_1020:
{ r3 = #-0x1; p0 = cmp.eq(r16,#0x6); if (!p0.new) jump:t restore_status }
{ jump return_success; memw(r0+#0x88) = r2 }
type_1021:
{ jump return_success; memw(r0+#0xb0) = r2; memw(r0+#0xb4) = r16 }
type_1022:
{ r3 = #-0x1; p0 = cmp.eq(r16,#0x1); if (!p0.new) jump:t restore_status }
{ jump return_success; memw(r0+#0x8) = r2 }
type_1023:
{ jump return_success; memw(r0+#0xb8) = r2; memw(r0+#0xbc) = r16 }
type_1024:
{ jump return_success; memw(r0+#0xc0) = r2; memw(r0+#0xc4) = r16 }
type_1026:
{ r3 = #-0x1; p0 = cmp.eq(r16,#0x1); if (!p0.new) jump:t restore_status }
{ jump return_success; memw(r0+#0x80) = r2 }
type_1027:
{ jump return_success; memw(r0+#0xf0) = r2; memw(r0+#0xf4) = r16 }
type_1028:
{ r3 = #-0x1; p0 = cmp.eq(r16,#0x1); if (!p0.new) jump:t restore_status }
{ jump return_success; memw(r0+#0x84) = r2 }
type_102c:
{ p0 = cmp.eq(r16,#0x36); r3 = #-0x1; if (!p0.new) jump:t restore_status; if (p0.new) memw(r0+#0x44) = r2 }
{ jump return_success }
type_102d:
{ r3 = #-0x1; p0 = cmp.eq(r16,#0x4); if (!p0.new) jump:t restore_status }
{ jump return_success; memw(r0+#0x48) = r2 }
type_1032:
{ jump return_success; memw(r0+#0xd8) = r2; memw(r0+#0xdc) = r16 }
type_1039:
{ r3 = #-0x1; p0 = cmp.eq(r16,#0x10); if (!p0.new) jump:t restore_status }
{ jump return_success; memw(r0+#0x10) = r2 }
type_103a:
{ r3 = #-0x1; p0 = cmp.eq(r16,#0x1); if (!p0.new) jump:t restore_status }
{ jump return_success; memw(r0+#0x98) = r2 }
type_103c:
{ r3 = #-0x1; p0 = cmp.eq(r16,#0x1); if (!p0.new) jump:t restore_status }
{ jump return_success; memw(r0+#0x34) = r2 }
type_103d:
{ p0 = cmp.eq(r16,#0x20); r3 = #-0x1; if (!p0.new) jump:t restore_status; if (p0.new) memw(r0+#0x54) = r2 }
{ jump return_success }
type_103f:
{ r3 = #-0x1; p0 = cmp.eq(r16,#0x10); if (!p0.new) jump:t restore_status }
{ jump return_success; memw(r0+#0x64) = r2 }
type_1040:
{ r3 = #-0x1; p0 = cmp.eq(r16,#0x10); if (!p0.new) jump:t restore_status }
{ jump return_success; memw(r0+#0x68) = r2 }
type_1041:
{ r3 = #-0x1; p0 = cmp.eq(r16,#0x1); if (!p0.new) jump:t restore_status }
{ jump return_success; memw(r0+#0x94) = r2 }
type_1042:
{ jump return_success; memw(r0+#0xc8) = r2; memw(r0+#0xcc) = r16 }
type_1044:
{ r3 = #-0x1; p0 = cmp.eq(r16,#0x1); if (!p0.new) jump:t restore_status }
{ jump return_success; memw(r0+#0x4c) = r2 }
type_1045:
{ jump return_success; memw(r0+#0xe8) = r2; memw(r0+#0xec) = r16 }
type_1047:
{ r3 = #-0x1; p0 = cmp.eq(r16,#0x10); if (!p0.new) jump:t restore_status }
{ jump return_success; memw(r0+#0x18) = r2 }
type_104a:
{ r3 = #-0x1; p0 = cmp.eq(r16,#0x1); if (!p0.new) jump:t restore_status }
{ jump return_success; memw(r0+#0x0) = r2 }
type_104d:
{ jump return_success; memw(r0+#0x100) = r2; memw(r0+#0x104) = r16 }
type_1053:
{ r3 = #-0x1; p0 = cmp.eq(r16,#0x2); if (!p0.new) jump:t restore_status }
{ jump return_success; memw(r0+#0x2c) = r2 }
type_1057:
{ r3 = #-0x1; p0 = cmp.eq(r16,#0x1); if (!p0.new) jump:t restore_status }
{ jump return_success; memw(r0+#0xa0) = r2 }
type_1059:
{ jump return_success; memw(r0+#0xf8) = r2; memw(r0+#0xfc) = r16 }
type_1061:
{ r3 = #-0x1; p0 = cmp.eq(r16,#0x1); if (!p0.new) jump:t restore_status }
{ jump return_success; memw(r0+#0x8c) = r2 }
type_103b:
{ r3 = #-0x1; p0 = cmp.eq(r16,#0x1); if (!p0.new) jump:t restore_status }
{ jump return_success; memw(r0+#0x9c) = r2 }
type_103e:
{ p0 = cmp.eq(r16,#0x20); r3 = #-0x1; if (!p0.new) jump:t restore_status; if (p0.new) memw(r0+#0x58) = r2 }
{ jump return_success }
type_1048:
{ r3 = #-0x1; p0 = cmp.eq(r16,#0x10); if (!p0.new) jump:t restore_status }
{ jump return_success; memw(r0+#0x14) = r2 }
type_1054:
{ r3 = #-0x1; p0 = cmp.eq(r16,#0x8); if (!p0.new) jump:t restore_status }
{ jump return_success; memw(r0+#0x30) = r2 }
type_1062:
{ r3 = #-0x1; p0 = cmp.eq(r16,#0x1); if (!p0.new) jump:t restore_status }
{ jump return_success; memw(r0+#0x90) = r2 }
type_106a:
{ p0 = cmp.eq(r16,#0x8); if (!p0.new) jump:t report_wrong_length }
{ r3 = memw(r0+#0x18c) }
{ r4 = #0xa; if (cmp.gtu(r4.new,r3)) jump:t append_eight_byte_pointer }
{ r0 = ##0x64e8; r1 = #0x2451; memw(r29+#0x0) = #0xa }
{ call 0x159f6c }
{ call -0xbe604; r1:0 = combine(#0xa,##0xb02b6f84) }
return_success:
{ r3 = #0x0 }
restore_status:
{ r0 = r3; r17:16 = memd(r29+#0x8); dealloc_return }
report_wrong_length:
{ r1 = #0x2451; r0 = ##0x64e0; memw(r29+#0x0) = r16 }
{ call 0x159f44 }
{ call -0xbe62c; r1:0 = combine(r16,##-0x4fd49088) }
{ r3 = #-0x1; jump restore_status }
append_eight_byte_pointer:
// r4 uses the old count in r3 while r3 becomes the success result.
{ r4 = addasl(r0,r3,#0x2); r3 = #0x0; r5 = add(r0,#0x18c) }
{ memw(r4+#0x164) = r2 }
{ jump restore_status; memw(r5+#0x0) += #0x1 }
