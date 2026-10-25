// SPDX-License-Identifier: GPL-3.0-only
// Configure/copy two physical memory banks and clean cache, [0x67a8,0x67e8).
.text
.globl adsp_rwe_physical_sync
adsp_rwe_physical_sync:
{ allocframe(#0) }
{ r2=#128; immext(#0xf0003dc0); r1=##0xf0003de0 }
{ immext(#0xf000ea80); r0=memw(gp+##0xf000eab8) }
{ immext(#0xffffff40); call cache_copy }
{ r2=#128; immext(#0xf0003e40); r1=##0xf0003e60 }
{ immext(#0xf000eac0); r0=memw(gp+##0xf000eac4) }
{ immext(#0xffffff40); call cache_copy }
{ dealloc_return }
.set cache_copy,adsp_rwe_physical_sync+(0x8be03734-0x8be037a8)
