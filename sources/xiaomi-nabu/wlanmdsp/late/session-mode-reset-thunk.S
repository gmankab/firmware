// SPDX-License-Identifier: GPL-3.0-only
// Agent 20-14: file [0x101688, 0x101694),
// runtime [0xb00c1688, 0xb00c1694).
// Tail-call the external mode operation with r1=0x11 and r3:r2=0.
// The negative external displacement is deliberately forced extended.
// Explicit V66 mnemonic/packet source; no reference bytes are used at build time.
.text
.globl wlan_late_session_mode_reset_thunk
wlan_late_session_mode_reset_thunk:
{ r1 = #0x11 ; jump ##-0x7bfd4; r3:2 = combine(#0x0,#0x0) }
