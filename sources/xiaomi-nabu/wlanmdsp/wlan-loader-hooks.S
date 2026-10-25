// SPDX-License-Identifier: GPL-3.0-only
// After the Hexagon PLT, the loader publishes saved return/frame registers
// into two runtime relocation slots and dispatches control to the lazy
// binding startup. The seven literal NOPs are actual Hexagon instructions,
// distinct from the page-alignment fill before the PLT.
//
// Relative calls use the segment-local location (the source assembles as
// its own section beginning at reference runtime offset 0x1b80). The two
// negative extended branch offsets target existing import stubs within the
// earlier PLT: they are NOT the raw OEM instruction encoding.
.text
.globl wlan_loader_hooks
wlan_loader_hooks:
{ r0 = add(pc,##3527840) }
{ memw(r0+#0) = r31 }
{ r0 = add(pc,##3527832) }
{ memw(r0+#0) = r29 }
{ r0 = #-1 }
{ jump ##-2924 }
.rept 7
{ nop }
.endr
// The tiny two-way dispatch resolves a static GOT entry via the PLT.
{ r0 = pc }
{ jump wlan_resolve_import }
wlan_resolve_import:
{ jump ##-2936 }
// Preserved registers are stored to their independently identified
// global-pointer slots, then control returns to the caller.
{ memw(##2959748528) = r0; memw(##2956331872) = r1 }
{ jumpr r31 }
