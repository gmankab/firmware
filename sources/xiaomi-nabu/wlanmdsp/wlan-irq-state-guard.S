// SPDX-License-Identifier: GPL-3.0-only
// WLAN interrupt-state guard at [0xb0002184, 0xb00021a0), Hexagon VLIW.
// Read global runtime state; return when the relevant bit is clear and
// dispatch to the existing registered IRQ callback when it is set.
.text
.globl wlan_irq_state_guard
wlan_irq_state_guard:
{ r2 = memw(gp+#1932) }
{ r2 = memub(r2+#200) }
{ p0 = !tstbit(r2,#3); if (!p0.new) jump:t dispatch }
{ jumpr r31 }
dispatch:
{ r0 = #1; jump ##0x1d3cd8 }
