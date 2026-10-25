// SPDX-License-Identifier: GPL-3.0-only
// Agent20-14: file [0x101bd0,0x101be4), runtime [0xb00c1bd0,0xb00c1be4).
// Emit fixed status warning arguments to the terminal diagnostic helper.
// Typed Hexagon V66 packet operations, not opaque OEM instructions.
.text
.globl agent14_late_status_warning_thunk
agent14_late_status_warning_thunk:
// +0x0: 4 encoded words
{ r0 = ##-1339330352; r1 = ##-1339330332 }
// +0x10: 1 encoded words
{ call 0x15b3e4 }
