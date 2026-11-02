// Independently assembled mini-packets; a source packet joiner changes
// parse bits of the first two full instructions into continuation markers.
.text
.globl agent14_late_status_frame_explicit_duplex
agent14_late_status_frame_explicit_duplex:
{ r17:16 = combine(r2,r1) }
{ r0 = add(r29,#0x8) }
{ r1 = #0x0; r18 = r0 }
