// SPDX-License-Identifier: GPL-3.0-only
// WLAN V66 source reconstructed from instruction semantics and control-flow.
// File [0x106598,0x1065a8); runtime [0xb00c6598,0xb00c65a8).
// Forward an event with a structured descriptor.
// This is instruction-level reconstruction, not evidence of on-target behavioral equivalence.
.text
.globl agent14_late_descriptor_event_thunk
agent14_late_descriptor_event_thunk:
pkt_0000:
// offset +0x0000, 1 encoded words
{ r3:2 = combine(r0,r1) }
pkt_0004:
// offset +0x0004, 3 encoded words
{ r1:0 = combine(r3,##-0x4fd48918); jump 1403432 }
