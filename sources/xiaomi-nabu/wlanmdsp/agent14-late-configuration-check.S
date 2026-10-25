// SPDX-License-Identifier: GPL-3.0-only
// Agent 20-14: file [0x1007c4,0x100840),
// runtime [0xb00c07c4,0xb00c0840).
// Check scalar configuration bounds without writing the input structure.
// Require index<global_limit; fields+0x10/+0x14 in [1,65535] and ordered;
// field+0x18<=65535; field+0x1c in {8,16,...,80}; field+0x20 in [1,512].
// Return zero on acceptance and -1 otherwise. No null-pointer guard is added.
// Explicit V66 mnemonic/packet source; no reference bytes are used at build time.
.text
.globl wlan_late_configuration_check
wlan_late_configuration_check:
// The saved input pointer uses the incoming r0 while r0 becomes -1.
{ r2 = r0; r0 = #-0x1; r3 = memw(gp+#0x78c) }
{ r4 = memw(r2+#0x4) }
{ r3 = memw(r3+#0x140); if (!cmp.gtu(r3.new,r4)) jump:t return_status }
{ r3 = memw(r2+#0x10) }
{ r4 = add(r3,#-0x1) }
{ p0 = cmp.gtu(r4,##0xfffe); if (p0.new) jump:t return_status; if (!p0.new) r4 = memw(r2+#0x14) }
{ r5 = add(r4,#-0x1) }
{ p0 = cmp.gtu(r5,##0xfffe); if (p0.new) jumpr:nt r31 }
{ p0 = cmp.gtu(r3,r4); if (p0.new) jump:nt return_status; if (!p0.new) r3 = memw(r2+#0x18) }
{ r3 = lsr(r3,#0x10); if (!cmp.eq(r3.new,#0x0)) jump:nt return_status }
{ r3 = memw(r2+#0x1c) }
{ r4 = add(r3,#-0x8) }
{ p0 = cmp.gtu(r4,#0x48); if (p0.new) jumpr:nt r31 }
{ r3 = and(r3,#0x7); if (!cmp.eq(r3.new,#0x0)) jump:nt return_status }
{ r2 = memw(r2+#0x20) }
{ r2 = add(r2,#-0x1) }
{ p0 = cmp.gtu(r2,#0x1ff); if (!p0.new) r0 = #0x0 }
return_status:
{ jumpr r31 }
