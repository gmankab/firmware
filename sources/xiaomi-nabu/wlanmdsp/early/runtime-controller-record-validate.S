// SPDX-License-Identifier: GPL-3.0-only
// Check session record dimensions, report invalid counts, normalize extent
// bookkeeping, and pass diagnostic arguments to the controller callback.
// Hexagon V66 instruction packets are explicitly grouped; no opaque bytes.
.text
.globl agent13_wlan_controller_record_validate
agent13_wlan_controller_record_validate:
L_a8:
{
    call 0x29e70c
    allocframe(#32)
}
{
    r16 = r0
}
{
    r20 = add(r16,#12)
    r2 = memw(r16+#12)
    if (cmp.gtu(r2.new,#5)) jump:nt L_110
}
{
    r20 = add(r16,#16)
    r4 = memw(r16+#16)
    if (cmp.gtu(r4.new,#20)) jump:nt L_128
}
{
    r20 = add(r16,#20)
    r3 = memw(r16+#20)
    if (cmp.gtu(r3.new,#6)) jump:nt L_144
}
{
    r19 = memw(r16+#24)
}
{
    r5 = #5
    if (!cmp.gtu(r5.new,r19)) jump:nt L_160
}
{
    r5 = add(r4,#7)
    r2 = add(r2,r4)
    r0 = add(r3,#7)
}
{
    r1 = lsr(r5,#3)
}
{
    r3 += add(r2,r1)
    r2 = memw(r16+#4)
}
{
    r3 += lsr(r0,#3)
}
{
    r19 += add(r3,#1)
    if (cmp.gtu(r19.new,r2)) jump:nt L_198
}
L_108:
{
    r0 = #1
    jump ##0x29e788
}
L_110:
{
    r16 = #2102
    r17 = ##-1339396812
}
{
    r18 = #5
    jump L_17c
    r0 = ##480
}
L_128:
{
    r16 = #2112
    r17 = ##-1339396800
}
{
    r18 = #20
    r0 = ##488
    r2 = r4
}
{
    jump L_17c
}
L_144:
{
    r16 = #2122
    r17 = ##-1339396788
}
{
    r18 = #6
    r0 = ##496
    r2 = r3
}
{
    jump L_17c
}
L_160:
{
    r20 = add(r16,#24)
    r16 = #2133
    r17 = ##-1339396776
}
{
    r18 = #4
    r0 = ##504
    r2 = r19
}
L_17c:
{
    call 0x214084
    r1 = #594
    memw(r29+#4) = r18
    memw(r29+#0) = r2
}
{
    call -0x4e1c
    r0 = r17
    r1 = memw(r20+#0)
    r2 = r18
}
{
    jump L_1c0
}
L_198:
{
    r1 = #594
    r0 = ##512
    memw(r29+#4) = r2
}
{
    call 0x21405c
    memw(r29+#0) = r19
}
{
    call -0x4e40
    r1:0 = combine(r19,##-1339396764)
    r2 = memw(r16+#4)
}
{
    r16 = #2153
}
L_1c0:
{
    call -0x162c
    r1:0 = combine(r16,##-1339397192)
}
{
    call 0x29e5a0
    allocframe(#128)
}
