.text
.globl wlan_record_tail
wlan_record_tail:
// Original Hexagon packet location: file 0x46a08
L_00000:
{
    r7 = lsr(r7,#3)
    if (cmp.gtu(r4,r7.new)) jump:t L_00054
}
// Original Hexagon packet location: file 0x46a10
L_00008:
{
    jump L_00150
}
// Original Hexagon packet location: file 0x46a14
L_0000c:
{
    r12 = memuh(r12+#14)
    if (cmp.gtu(r5,r12.new)) jump:nt L_00154
}
// Original Hexagon packet location: file 0x46a1c
L_00014:
{
    memb(r16+#4) = r7
    r9 = memw(r9+#20)
}:mem_noshuf
// Original Hexagon packet location: file 0x46a24
L_0001c:
{
    p0 = tstbit(r9,#0)
    if (p0.new) jump:t L_00038
    if (!p0.new) r7 = memw(r8+#0)
}
// Original Hexagon packet location: file 0x46a30
L_00028:
{
    r7 = memuh(r7+#12)
}
// Original Hexagon packet location: file 0x46a34
L_0002c:
{
    r7 = lsr(r7,#3)
    if (!cmp.gtu(r4,r7.new)) jump:t L_00054
}
// Original Hexagon packet location: file 0x46a3c
L_00034:
{
    jump L_0015c
}
// Original Hexagon packet location: file 0x46a40
L_00038:
{
    memb(r16+#3) = r7
    r7 = memw(r8+#0)
}:mem_noshuf
// Original Hexagon packet location: file 0x46a48
L_00040:
{
    r7 = memuh(r7+#12)
}
// Original Hexagon packet location: file 0x46a4c
L_00044:
{
    r7 = lsr(r7,#3)
}
// Original Hexagon packet location: file 0x46a50
L_00048:
{
    p0 = cmp.gtu(r7,#374)
    if (p0.new) r1 = #1496
    if (p0.new) jump:nt L_00164
}
// Original Hexagon packet location: file 0x46a5c
L_00054:
{
    r2 = add(r2,#16)
    nop
}:endloop0
// Original Hexagon packet location: file 0x46a64
L_0005c:
{
    jump L_00140
}
// Original Hexagon packet location: file 0x46a68
L_00060:
{
    r3 = +mpyi(r0,#240)
    r2 = asl(r0,#2)
    p0 = cmp.gt(r0,#0)
    if (!p0.new) jump:nt L_00140
}
// Original Hexagon packet location: file 0x46a74
L_0006c:
{
    r7 = #4915
    r5 = #2484
    r6 = add(r0,#-1)
    r4 = #2412
}
// Original Hexagon packet location: file 0x46a84
L_0007c:
{
    r3 += mpyi(r17,##488)
    r2 += add(r16,##804)
}
// Original Hexagon packet location: file 0x46a94
L_0008c:
{
    r3 += add(r16,##756)
    p0 = cmp.eq(r17,#4)
    p1 = cmp.eq(r17,#2)
}
// Original Hexagon packet location: file 0x46aa4
L_0009c:
{
    r8 = #5500
}
// Original Hexagon packet location: file 0x46aa8
L_000a0:
{
    r9 = memw(r3+#0)
}
// Original Hexagon packet location: file 0x46aac
L_000a4:
{
    p2 = !tstbit(r9,#1)
    if (!p2.new) jump:t L_000d0
    if (!p2.new) memb(r16+#4) = r6
}
// Original Hexagon packet location: file 0x46ab8
L_000b0:
{
    p2 = tstbit(r9,#0)
    if (!p2.new) jump:t L_0012c
}
// Original Hexagon packet location: file 0x46ac0
L_000b8:
{
    memb(r16+#3) = r6
    r9 = memw(r2+#0)
}:mem_noshuf
// Original Hexagon packet location: file 0x46ac8
L_000c0:
{
    memh(r9+#12) = r4
    r9 = memw(r2+#0)
}:mem_noshuf
// Original Hexagon packet location: file 0x46ad0
L_000c8:
{
    jump L_0012c
    memh(r9+#14) = r5
}
// Original Hexagon packet location: file 0x46ad8
L_000d0:
{
    if (p0) jump:nt L_000e4
}
// Original Hexagon packet location: file 0x46adc
L_000d4:
{
    if (!p1) r9 = ##5925
    if (!p1) jump:nt L_00108
    if (!p1) r12 = memw(r2+#0)
}
// Original Hexagon packet location: file 0x46aec
L_000e4:
{
    p2 = cmp.eq(r6,#0)
    r9 = memw(r2+#0)
}
// Original Hexagon packet location: file 0x46af4
L_000ec:
{
    if (!p2) r9 = ##5925
    if (!p2) memh(r9+#12) = r8
    if (p2) memh(r9+#12) = r7
}
// Original Hexagon packet location: file 0x46b04
L_000fc:
{
    if (p2) r9 = ##5320
    jump L_0010c
}
// Original Hexagon packet location: file 0x46b10
L_00108:
{
    memh(r12+#12) = r7
}
// Original Hexagon packet location: file 0x46b14
L_0010c:
{
    r12 = memw(r2+#0)
}
// Original Hexagon packet location: file 0x46b18
L_00110:
{
    memh(r12+#14) = r9
    r9 = memw(r3+#0)
}:mem_noshuf
// Original Hexagon packet location: file 0x46b20
L_00118:
{
    p2 = tstbit(r9,#0)
    if (!p2.new) jump:t L_0012c
}
// Original Hexagon packet location: file 0x46b28
L_00120:
{
    memb(r16+#3) = r6
    r9 = memw(r2+#0)
}:mem_noshuf
// Original Hexagon packet location: file 0x46b30
L_00128:
{
    memh(r9+#12) = r4
}
// Original Hexagon packet location: file 0x46b34
L_0012c:
{
    r9 = add(r6,#1)
    r3 = add(r3,#-240)
    r2 = add(r2,#-4)
    r6 = add(r6,#-1)
}
// Original Hexagon packet location: file 0x46b40
L_00138:
{
    p2 = cmp.gt(r9,#1)
    if (p2.new) jump:t L_000a0
}
// Original Hexagon packet location: file 0x46b48
L_00140:
{
    jump 0x29ed0c
}
// Original Hexagon packet location: file 0x46b4c
L_00144:
{
    r1 = #546
    r0 = ##-1339396993
}
// Original Hexagon packet location: file 0x46b58
L_00150:
{
    call -0x1020
}
// Original Hexagon packet location: file 0x46b5c
L_00154:
{
    call -0x1024
    r1 = #1491
}
// Original Hexagon packet location: file 0x46b64
L_0015c:
{
    call -0x102c
    r1 = #1498
}
// Original Hexagon packet location: file 0x46b6c
L_00164:
{
    call -0x1034
}
// Original Hexagon packet location: file 0x46b70
L_00168:
{
    p0 = cmp.eq(r2,#1)
    r4 = memw(##2960400244)
    allocframe(#0)
}
// Original Hexagon packet location: file 0x46b80
L_00178:
{
    r1 = #1
    r3 = #1
    r0 = memw(##2959867728)
}
// Original Hexagon packet location: file 0x46b90
L_00188:
{
    r2 = mux(p0,#1,#0)
    callr r4
}
// Original Hexagon packet location: file 0x46b98
L_00190:
{
    dealloc_return
}
