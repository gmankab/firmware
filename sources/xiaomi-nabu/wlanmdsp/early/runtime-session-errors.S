// SPDX-License-Identifier: GPL-3.0-only
// Error- and success-report paths for bounded record serialization. This
// completes the preceding session record-processing routine. Packets use
// semantic Hexagon V66 instructions and native immediate extension.
.text
.globl agent13_wlan_session_error_paths
agent13_wlan_session_error_paths:
L_3e8:
{
    jump 0x29e460
}
L_3ec:
{
    r1 = #594
    r0 = ##544
    r2 = and(r2,#255)
}
{
    call 0x213e08
    r16 = add(r23,r17)
    memw(r29+#4) = r2
    memw(r29+#0) = r19
}
{
    call -0x5098
    r1:0 = combine(r19,##-1339396716)
    r2 = memub(r16+#0)
}
{
    r1 = #2231
    jump L_448
}
L_41c:
{
    r1 = #594
    r0 = ##560
    memw(r29+#4) = r17
}
{
    call 0x213dd8
}
{
    call -0x50c0
    r1:0 = combine(r19,##-1339396704)
    r2 = r17
}
{
    r1 = #2313
    jump L_448
}
L_444:
{
    r1 = #2280
}
L_448:
{
    call -0x18b4
    r0 = ##-1339397163
}
{
    r1 = #594
    r0 = ##520
    memw(r29+#0) = r17
}
{
    call 0x213da0
    memw(r29+#4) = #80
}
{
    call -0x50fc
    r1:0 = combine(r17,##-1339396752)
    r2 = #80
}
{
    r1 = #2189
    jump L_448
}
L_480:
{
    r1 = #594
    r0 = ##528
    memw(r29+#0) = r2
}
{
    call 0x213d74
    memw(r29+#4) = #10
}
{
    r0 = ##2955570556
    r2 = #10
    r1 = memw(r22+#12)
}
{
    call -0x5134
}
{
    r1 = #2200
    jump L_448
}
L_4ac:
{
    r1 = #594
    r0 = ##536
    memw(r29+#4) = r4
}
{
    call 0x213d48
    memw(r29+#0) = r3
}
{
    r0 = ##-1339396728
    r2 = memw(r22+#12)
    r1 = memub(r16+r18<<#0)
}
{
    call -0x5164
    r2 = add(r2,r24)
}
{
    r1 = #2219
    jump L_448
}
{
    nop
}
{
    nop
}
{
    nop
}
