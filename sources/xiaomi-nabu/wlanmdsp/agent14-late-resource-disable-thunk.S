// SPDX-License-Identifier: GPL-3.0-only
// Agent 20-14: file [0x101680, 0x101688),
// runtime [0xb00c1680, 0xb00c1688).
// Tail-call the external resource operation with r1=0; retain the forced
// extended branch encoding from the reference packet.
// Explicit V66 mnemonic/packet source; no reference bytes are used at build time.
.text
.globl wlan_late_resource_disable_thunk
wlan_late_resource_disable_thunk:
{ r1 = #0x0 ; jump ##0xbde20 }
