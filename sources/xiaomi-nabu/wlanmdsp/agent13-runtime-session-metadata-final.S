.text
.globl wlan_metadata_reconcile_tail
wlan_metadata_reconcile_tail:
// Original Hexagon packet location: file 0x46e00
L_003f8:
{
    call -0x5e4
    r1:0 = combine(r19,r16)
    r2 = add(r29,#40)
    r3 = add(r29,#0)
}
// Original Hexagon packet location: file 0x46e0c
L_00404:
{
    r17 = #6
    p0 = cmp.gtu(r19,#5)
    if (p0.new) jump:t L_00458
}
// Original Hexagon packet location: file 0x46e14
L_0040c:
{
    jump L_00458
    r17 = memw(r19<<#2+##2955571216)
}
// Original Hexagon packet location: file 0x46e20
L_00418:
{
    r0 = r16
    r2 = memub(r16+#1)
}
// Original Hexagon packet location: file 0x46e24
L_0041c:
{
    if (p0.new) r1 = #1
    if (p0.new) r3:2 = combine(r18,r17)
    p0 = cmp.gtu(r2,#2)
    if (p0.new) jump:t L_00444
}
// Original Hexagon packet location: file 0x46e30
L_00428:
{
    call -0x4f0
    r3:2 = combine(r18,r17)
    r1 = #0
    r17 = #0
}
// Original Hexagon packet location: file 0x46e3c
L_00434:
{
    call -0x620
    r3 = add(r29,#0)
    r2 = add(r29,#40)
    r1:0 = combine(#0,r16)
}
// Original Hexagon packet location: file 0x46e48
L_00440:
{
    jump L_00458
}
// Original Hexagon packet location: file 0x46e4c
L_00444:
{
    call -0x50c
}
// Original Hexagon packet location: file 0x46e50
L_00448:
{
    call -0x634
    r1:0 = combine(#1,r16)
    r2 = add(r29,#40)
    r3 = add(r29,#0)
}
// Original Hexagon packet location: file 0x46e5c
L_00454:
{
    r17 = #2
}
// Original Hexagon packet location: file 0x46e60
L_00458:
{
    call 0x1fda88
}
// Original Hexagon packet location: file 0x46e64
L_0045c:
{
    call 0x1fe404
    r1 = add(r29,#40)
    r0 = r17
}
// Original Hexagon packet location: file 0x46e6c
L_00464:
{
    call 0x1fda44
}
// Original Hexagon packet location: file 0x46e70
L_00468:
{
    r1 = #1825
    p0 = cmp.eq(r20,#0)
    if (!p0.new) jump:nt L_00508
}
// Original Hexagon packet location: file 0x46e78
L_00470:
{
    r1 = #1826
    r17 = memw(r16+#72)
    if (cmp.eq(r17.new,#0)) jump:nt L_00508
}
// Original Hexagon packet location: file 0x46e84
L_0047c:
{
    r2 = memb(r16+#1)
}
// Original Hexagon packet location: file 0x46e88
L_00480:
{
    p0 = cmp.eq(r2,#0)
    if (p0.new) jump:nt L_004c8
    r2 = memw(r17+#0)
    memw(r16+#76) = r2.new
}
// Original Hexagon packet location: file 0x46e94
L_0048c:
{
    r18 = add(r16,#808)
    r19 = #0
    r20 = #0
}
// Original Hexagon packet location: file 0x46e9c
L_00494:
{
    r2 = memw(r18+r20<<#2)
    r3 = memw(r17+r20<<#2)
}
// Original Hexagon packet location: file 0x46ea4
L_0049c:
{
    memw(r2+#36) = r3
    r2 = memw(r18+r20<<#2)
}:mem_noshuf
// Original Hexagon packet location: file 0x46eac
L_004a4:
{
    call 0x1f8960
    r0 = memw(r2+#36)
}
// Original Hexagon packet location: file 0x46eb4
L_004ac:
{
    r19 = add(r19,#1)
    r2 = memw(r18+r20<<#2)
}
// Original Hexagon packet location: file 0x46ebc
L_004b4:
{
    r20 = and(r19,#255)
    memw(r2+#180) = r0
}
// Original Hexagon packet location: file 0x46ec4
L_004bc:
{
    r2 = memub(r16+#1)
    if (cmp.gtu(r2.new,r20)) jump:t L_00494
}
// Original Hexagon packet location: file 0x46ecc
L_004c4:
{
    r2 = memw(r16+#76)
}
// Original Hexagon packet location: file 0x46ed0
L_004c8:
{
    r0 = r16
    p0 = cmp.eq(r2,#0)
    if (p0.new) jump:nt L_00504
    if (!p0.new) r2 = memw(r29+#188)
}
// Original Hexagon packet location: file 0x46edc
L_004d4:
{
    r1 = memub(r16+#320)
    memb(r16+#20) = r2
}
// Original Hexagon packet location: file 0x46ee4
L_004dc:
{
    call 0x30a3c
}
// Original Hexagon packet location: file 0x46ee8
L_004e0:
{
    call 0x133ac0
    r0 = ##-1342115456
    memw(r16+#80) = r0
}
// Original Hexagon packet location: file 0x46ef8
L_004f0:
{
    call 0x14c82c
    r0 = ##-1342149776
}
// Original Hexagon packet location: file 0x46f04
L_004fc:
{
    r0 = #0
    jump ##0x29e92c
}
// Original Hexagon packet location: file 0x46f0c
L_00504:
{
    r1 = #1868
}
// Original Hexagon packet location: file 0x46f10
L_00508:
{
    call -0x13c8
    r0 = ##-1339397253
}
