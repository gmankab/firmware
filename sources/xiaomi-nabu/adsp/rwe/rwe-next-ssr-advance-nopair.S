// SPDX-License-Identifier: GPL-3.0-only
// Loop body scalar form with all four source operations; LLVM6 `-mno-pairing`
// preserves register order prior to inserting the separately assembled duplex.
.text
.globl advance_in_source_order
advance_in_source_order:
{ loop0(advance_body,#3) }
advance_body:
{ r4=add(r4,#4); r3=add(r3,#320); r0=add(r0,#4); r2=add(r2,#1) }:endloop0
