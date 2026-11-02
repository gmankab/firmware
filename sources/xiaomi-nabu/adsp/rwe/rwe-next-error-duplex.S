// SPDX-License-Identifier: GPL-3.0-only
// Source-defined one-word Hexagon duplex in original error packet.
.text
.globl controller_error_duplex
controller_error_duplex:
{ r0=#36; r17:16=memd(r29+#0) }
