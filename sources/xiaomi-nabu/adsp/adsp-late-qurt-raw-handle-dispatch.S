// SPDX-License-Identifier: GPL-3.0-only
// V66 shared Hexagon source for private ADSP file 0x3f71c
.text
.globl adsp_late_qurt_raw_handle_dispatch
adsp_late_qurt_raw_handle_dispatch:
// Conditionally decode a controller handle and tail-dispatch to its
// tagged function pointer; preserve the Hexagon packet parse markers.
{ immext(#4029460672); r6 = ##-265506564; p0 = cmp.gtu(r2,#255) }
{ r6 = #-1; immext(#1024); if (p0) r7 = memw(r6+##1024); if (!p0) r7 = memw(r6+r2<<#2) }
{ p0 = tstbit(r7,#0); if (!p0.new) jumpr:t r7; if (p0.new) r6 = memw(r7+r6<<#0) }
{ r1 = add(r7,#-1); jumpr r6 }
