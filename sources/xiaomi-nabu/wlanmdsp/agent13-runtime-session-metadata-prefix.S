.text
.globl wlan_session_metadata_setup
wlan_session_metadata_setup:
// Original Hexagon packet location: file 0x46b9c
L_00194:
{
    call 0x29ebb8
    allocframe(#384)
}
// Original Hexagon packet location: file 0x46ba4
L_0019c:
{
    r17:16 = combine(r2,r0)
    r19:18 = combine(r1,r3)
    r1 = #0
    r0 = add(r29,#80)
}
// Original Hexagon packet location: file 0x46bb0
L_001a8:
{
    call -0x4c08
    r2 = #48
    r21 = #0
}
// Original Hexagon packet location: file 0x46bb8
L_001b0:
{
    call -0x4c10
    r2 = #40
    r1 = #0
    r0 = add(r29,#0)
}
// Original Hexagon packet location: file 0x46bc4
L_001bc:
{
    r0 = add(r29,#140)
    r1 = #0
    r2 = #212
    r22 = add(r29,#140)
}
// Original Hexagon packet location: file 0x46bd0
L_001c8:
{
    call -0x4c28
}
// Original Hexagon packet location: file 0x46bd4
L_001cc:
{
    r2 = memw(r16+#324)
}
// Original Hexagon packet location: file 0x46bd8
L_001d0:
{
    r0 = +mpyi(r2,#152)
    call -0x774
}
// Original Hexagon packet location: file 0x46be0
L_001d8:
{
    call -0x77c
    r0 = #8
    memw(r16+#32) = r0
}
// Original Hexagon packet location: file 0x46be8
L_001e0:
{
    call -0x784
    r20 = r0
    r0 = #4096
    memw(r16+#36) = r20.new
}
// Original Hexagon packet location: file 0x46bf8
L_001f0:
{
    r0 = r16
    r3 = ##8388608
    memw(r20+#0) = r0
}
// Original Hexagon packet location: file 0x46c04
L_001fc:
{
    r2 = memw(##2956214496)
    memw(r20+#4) = r3
}
// Original Hexagon packet location: file 0x46c10
L_00208:
{
    callr r2
}
// Original Hexagon packet location: file 0x46c14
L_0020c:
{
    r0 = add(r16,#816)
    r1 = ##-1342135400
}
// Original Hexagon packet location: file 0x46c20
L_00218:
{
    call 0x214adc
    r2 = r0
}
// Original Hexagon packet location: file 0x46c28
L_00220:
{
    r2 = #2
    r0 = r16
    r4 = memw(r16+#320)
    r3 = memuh(r16+#336)
}
// Original Hexagon packet location: file 0x46c38
L_00230:
{
    r5 = asl(r4,#1)
    r6 = memw(r16+#324)
    memw(r29+#144) = r6.new
}
// Original Hexagon packet location: file 0x46c44
L_0023c:
{
    r8 = add(r6,r4)
    r1 = memw(r16+#348)
    r7 = memw(r16+#340)
}
// Original Hexagon packet location: file 0x46c50
L_00248:
{
    r1 = addasl(r8,r4,#1)
    memb(r29+#181) = r1
    memw(r29+#264) = r1
}
// Original Hexagon packet location: file 0x46c5c
L_00254:
{
    r1 = memub(r16+#532)
    memw(r29+#156) = r1
}
// Original Hexagon packet location: file 0x46c64
L_0025c:
{
    r1 = memw(r16+#368)
    memb(r29+#348) = r1
}
// Original Hexagon packet location: file 0x46c6c
L_00264:
{
    r1 = memw(r16+#372)
    memb(r29+#186) = r1
}
// Original Hexagon packet location: file 0x46c74
L_0026c:
{
    r1 = memw(r16+#364)
    memb(r29+#187) = r1
}
// Original Hexagon packet location: file 0x46c7c
L_00274:
{
    r9 = memw(r16+#508)
    memw(r29+#172) = r1
}
// Original Hexagon packet location: file 0x46c84
L_0027c:
{
    p0 = cmp.eq(r9,#0)
    r1 = memw(r16+#360)
    memw(r29+#168) = r1.new
}
// Original Hexagon packet location: file 0x46c90
L_00288:
{
    r7 = mux(p0,r4,r9)
    r10 = memw(r16+#344)
    memw(r29+#148) = r7
}
// Original Hexagon packet location: file 0x46c9c
L_00294:
{
    r6 = add(r10,r6)
    r1 = memw(r16+#392)
    memw(r29+#188) = r1.new
}
// Original Hexagon packet location: file 0x46ca8
L_002a0:
{
    r6 = addasl(r6,r4,#1)
    r1 = memw(r16+#408)
    r11 = memw(r16+#416)
}
// Original Hexagon packet location: file 0x46cb4
L_002ac:
{
    r6 += add(r5,#8)
    r2 = add(r29,#80)
    r23 = memw(r16+#808)
    memb(r16+#851) = r2
}
// Original Hexagon packet location: file 0x46cc4
L_002bc:
{
    r1 = memw(r16+#412)
    memw(r29+#192) = r1
}
// Original Hexagon packet location: file 0x46ccc
L_002c4:
{
    memb(r16+#850) = r21
    memh(r29+#176) = r3
}
// Original Hexagon packet location: file 0x46cd4
L_002cc:
{
    r3 = add(r29,#0)
    memh(r29+#260) = r3
    memb(r22+#43) = #1
}
// Original Hexagon packet location: file 0x46ce0
L_002d8:
{
    memb(r22+#44) = #1
    memw(r22+#148) = #1
}
// Original Hexagon packet location: file 0x46ce8
L_002e0:
{
    r22 = memw(r16+#420)
    memw(r22+#152) = #1
}
// Original Hexagon packet location: file 0x46cf0
L_002e8:
{
    memb(r29+#349) = r7
    memb(r29+#200) = r11
}
// Original Hexagon packet location: file 0x46cf8
L_002f0:
{
    memw(r29+#204) = r22
    memw(r29+#196) = r1
}
// Original Hexagon packet location: file 0x46d00
L_002f8:
{
    r7 = memw(r16+#352)
    memw(r29+#252) = r10
}
// Original Hexagon packet location: file 0x46d08
L_00300:
{
    r1 = memw(r23+#28)
    memw(r29+#208) = r1.new
}
// Original Hexagon packet location: file 0x46d10
L_00308:
{
    r6 = memw(r23+#32)
    memw(r29+#160) = r6
}
// Original Hexagon packet location: file 0x46d18
L_00310:
{
    r1 = memw(r16+#440)
    memw(r29+#212) = r6
}
// Original Hexagon packet location: file 0x46d20
L_00318:
{
    r4 += add(r10,#8)
    r6 = memw(r16+#452)
    memw(r29+#140) = r4
}
// Original Hexagon packet location: file 0x46d2c
L_00324:
{
    memw(r29+#216) = r1
    memw(r29+#224) = r6
}
// Original Hexagon packet location: file 0x46d34
L_0032c:
{
    r1 = memw(r16+#460)
    memw(r29+#228) = r1.new
}
// Original Hexagon packet location: file 0x46d3c
L_00334:
{
    r6 = memw(r16+#484)
    memw(r29+#152) = r4
}
// Original Hexagon packet location: file 0x46d44
L_0033c:
{
    r5 = memw(gp+#1932)
    memw(r29+#164) = r7
}
// Original Hexagon packet location: file 0x46d4c
L_00344:
{
    r4 = memub(r16+#1)
    memw(r29+#296) = r6
}
// Original Hexagon packet location: file 0x46d54
L_0034c:
{
    p0 = cmp.eq(r4,#3)
    p1 = cmp.eq(r4,#2)
    r1 = memw(r16+#488)
    memw(r29+#300) = r1.new
}
// Original Hexagon packet location: file 0x46d64
L_0035c:
{
    r6 = mux(p0,#4,#0)
    r7 = memw(r16+#444)
    r1 = memw(r5+#376)
}
// Original Hexagon packet location: file 0x46d70
L_00368:
{
    r7 = memw(r16+#464)
    memw(r29+#220) = r7
}
// Original Hexagon packet location: file 0x46d78
L_00370:
{
    r1 = mux(p0,#4,#1)
    memw(r29+#316) = r1
    memw(r29+#232) = r7
}
// Original Hexagon packet location: file 0x46d84
L_0037c:
{
    if (!p1) r1 = add(r6,#0)
    r7 = memw(r5+#380)
    memw(r29+#320) = r7.new
}
// Original Hexagon packet location: file 0x46d90
L_00388:
{
    r7 = memw(r5+#384)
    memw(r29+#324) = r7.new
}
// Original Hexagon packet location: file 0x46d98
L_00390:
{
    call -0x678
    r6 = memw(r5+#388)
    memw(r29+#328) = r6.new
}
// Original Hexagon packet location: file 0x46da4
L_0039c:
{
    r0 = add(r29,#132)
    r3 = memw(r16+#520)
    memw(r29+#336) = r3.new
}
// Original Hexagon packet location: file 0x46db0
L_003a8:
{
    r3 = add(r29,#140)
    r2 = memw(r16+#512)
    r1 = memw(r16+#524)
}
// Original Hexagon packet location: file 0x46dbc
L_003b4:
{
    r5 = lsr(r2,#2)
    r1 = add(r16,#72)
    r4 = memw(r29+#332)
    memw(r29+#340) = r1
}
// Original Hexagon packet location: file 0x46dcc
L_003c4:
{
    r4 = insert(r5,#5,#0)
    r2 = add(r29,#80)
    r6 = memw(r16+#528)
    memw(r29+#344) = r6.new
}
// Original Hexagon packet location: file 0x46ddc
L_003d4:
{
    call 0x1fdb44
    memw(r29+#332) = r4
}
// Original Hexagon packet location: file 0x46de4
L_003dc:
{
    r20 = r0
    if (!p0.new) r3:2 = combine(r18,r17)
    p0 = cmp.eq(r19,#0)
    if (p0.new) jump:nt 0x3c
}
// Original Hexagon packet location: file 0x46df0
L_003e8:
{
    r19 = memw(r19+#8)
    memw(r16+#884) = r19.new
}
// Original Hexagon packet location: file 0x46df8
L_003f0:
{
    call -0x4b8
    r1:0 = combine(r19,r16)
}
