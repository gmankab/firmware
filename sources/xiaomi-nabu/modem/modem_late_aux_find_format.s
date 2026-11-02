// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V60 source-generated indexed controller format record scan.
// modem.mbn file [0x447608,0x447664), virtual [0x1e401608,0x1e401664).
.text
.globl modem_late_aux_find_format
modem_late_aux_find_format:
{ r0 = #1
  r4 = r0 }
{ r6 = memuh(r4+#44) }
{ p0 = cmp.eq(r6,#0); if (p0.new) jump:nt .Lnot_found
  if (!p0.new) r5 = memw(r4+#28) }
{ loop0(.Lscan,r6) }
{ r5 += add(r4,#24) }
.Lscan:
{ r4 = memw(r5+#0) }
{ r4 = and(r4,##117440512) }
{ p0 = cmp.eq(r4,##33554432)
  if (p0.new) r0 = #0
  if (p0.new) jump:nt .Lselected }
{ r5 = add(r5,#32)
  nop }:endloop0
.Lnot_found:
{ jumpr r31 }
.Lselected:
{ r4 = memw(r5+#-20)
  memw(r1+#0) = r4.new }
{ r4 = memw(r5+#-8)
  memw(r2+#0) = r4.new }
{ jumpr r31
  r2 = memw(r5+#-4)
  memw(r3+#0) = r2.new }
