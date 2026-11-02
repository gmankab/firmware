// Hexagon semantic source, relative packet addresses.
.text
.globl session_state_d4_e4
session_state_d4_e4:

{
 r3:2 = bitsplit(r18,#24)
}

{
 call 0x169d78
 r1:0 = combine(r3,r19)
}

{
 jump 0x3f0
}
