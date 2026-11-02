// SPDX-License-Identifier: GPL-3.0-only
// Initialize controller format parameters and choose indexed device-control
// handlers. This is a partial session routine and does not invent its return.
// Semantic Hexagon source with checked packet layout, no OEM opcode words.
.text
.globl agent13_wlan_session_parameter_dispatch
agent13_wlan_session_parameter_dispatch:
L_4ec:
{
    call 0x29e280
    allocframe(#80)
}
{
    r17:16 = combine(r0,r1)
    r18 = r3
    r24 = r2
}
{
    p0 = cmp.eq(r16,#0)
    r26 = add(r17,#808)
}
{
    r0 = p0
    if (p0) jump:nt L_544
    memw(r29+#12) = r0.new
}
{
    r2 = addasl(r26,r16,#2)
    p0 = cmp.gtu(r16,#2)
    if (p0.new) jump:nt ##0x5d4
}
{
    r5:4 = combine(#1,##2388)
}
{
    r3:2 = combine(##-1339397133,#4)
    r6 = memw(r2+#-4)
}
{
    r19 = memw(r6+#36)
}
{
    call 0x205604
    r1:0 = combine(#2,r19)
}
{
    jump L_550
}
L_544:
{
    call 0x206854
    r0 = #2
    r19 = memw(r17+#76)
}
L_550:
{
    r2 = add(r24,#-1)
}
{
    p0 = cmp.gtu(r2,#206)
    if (!p0.new) r4 = ##-1340084804
    if (p0.new) jump:nt 0x580
}
{
    r20 = add(r17,#1)
    r21 = #3
    r5 = memw(r2<<#2+##2955568640)
}
{
    r27 = ##-1340088024
    r23:22 = combine(#9,#0)
}
{
    r1:0 = combine(#26,##568)
    r3:2 = combine(#7,#47)
}
{
    r25 = ##-1339396692
    jumpr r5
}
{
    call 0x21fe48
    r1 = r24
    r2 = r18
    r0 = memw(r17+#592)
}
{
    jump 0x52c
}
