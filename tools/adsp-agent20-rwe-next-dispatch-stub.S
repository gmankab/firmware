// SPDX-License-Identifier: GPL-3.0-only
// ADSP V66 8-byte predicated return/tailcall stub, file [0x113f0,0x113f8).
.text
.globl adsp_rwe_dispatch_stub
adsp_rwe_dispatch_stub:
{ p0=cmp.eq(r1,#1); if(!p0.new) jumpr:nt lr }
{ jump controller_delegate }
.set controller_delegate,adsp_rwe_dispatch_stub+(0x8be14724-0x8be0e3f0)
