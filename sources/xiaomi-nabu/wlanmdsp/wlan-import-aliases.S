// SPDX-License-Identifier: GPL-3.0-only
// Three Hexagon import-alias tail branches after the packed-record consumer.
// These branches dispatch to the earlier lazy-binding PLT wrappers for
// memset, unsigned division and memmove respectively. Each uses an explicit
// 32-bit relative displacement (the assembler emits immediate extension).
//
// IMPORTANT: disjoint executable interval [0xb0001fa8,0xb0001fc0); the
// prior record-consumer function is NOT independently reconstructed through
// this range, so these 24 code bytes must not be counted as part of the
// contiguous startup prefix. No vendor opcode words are embedded here.
.text
.globl wlan_import_aliases
wlan_import_aliases:
{ jump ##-3896 } // to PLT memset at relative executable address 0x1070
{ jump ##-3888 } // to PLT unsigned division at relative 0x1080
{ jump ##-3816 } // to PLT memmove at relative 0x10d0
