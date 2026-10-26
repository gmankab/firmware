// SPDX-License-Identifier: GPL-3.0-only
// Independent Hexagon duplex in an endloop0 packet. Dummy loop setup is
// discarded by the emitter; only the instruction at `advanced:` is used.
.text
.globl advance_loop
advance_loop:
{ loop0(advanced,#3) }
advanced:
{ r0=add(r0,#4); r2=add(r2,#1) }:endloop0
