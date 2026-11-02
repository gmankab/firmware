// SPDX-License-Identifier: GPL-3.0-only
// Bounded Hexagon V60 codec class scan with hardware loop0.
// MBN file [0x447664,0x4476b8), virtual [0x1e401664,0x1e4016b8).
.text
.globl modem_late_aux_find_codec_class
modem_late_aux_find_codec_class:
{ r0 = #1
  r5 = r0 }
{ r7 = memuh(r5+#44) }
{ p0 = !cmp.gt(r7,r1)
  if (!p0.new) r7 = sub(r7,r1)
  if (p0.new) jump:t .Lnot_found }
{ if (!p0) r6 = memw(r5+#28) }
{ loop0(.Lscan,r7)
  r6 += add(r5,#20) }
.Lscan:
{ r5 = memw(r6+#-20)
  if (cmp.eq(r5.new,#4)) jump:nt .Lselected }
{ r6 = add(r6,#32)
  nop }:endloop0
.Lnot_found:
{ jumpr r31 }
.Lselected:
{ r0 = #0
  r5 = memw(r6+#-16)
  memw(r2+#0) = r5.new }
{ r2 = memw(r6+#-4)
  memw(r3+#0) = r2.new }
{ jumpr r31
  r2 = memw(r6+#0)
  memw(r4+#0) = r2.new }
