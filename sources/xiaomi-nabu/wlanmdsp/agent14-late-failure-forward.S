// SPDX-License-Identifier: GPL-3.0-only
// Agent 20-14: file [0x1012f4, 0x101304),
// runtime [0xb00c12f4, 0xb00c1304).
// Forward an input diagnostic pointer and integer to the external helper at
// 0xb021cfc4 with a fixed format/context address. This thunk has no local return;
// calling sites use it on failure paths. No stronger callee contract is assumed.
// Explicit V66 mnemonic/packet source; no reference bytes are used at build time.
.text
.globl wlan_late_failure_forward
wlan_late_failure_forward:
{ r3:2 = combine(r0,r1) }
{ r1:0 = combine(r3,##-0x4fd49070); jump 0x15bccc }
