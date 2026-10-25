// SPDX-License-Identifier: GPL-3.0-only
// Agent 11: Hexagon V6 modem snapshot and SRAM controller startup.
// Qualcomm S-register transfer encoding from the Hexagon v6 architecture.
// Source operands drive every emitted word; reference firmware bytes are
// never included in source.
.text
.macro MODEM_SYS_WRITE system, general, marker=3
    .if (\system < 0) || (\system > 63) || (\general < 0) || (\general > 31)
        .error "invalid Hexagon special/general register"
    .endif
    .word (0x67000000 | (\general << 16) | (\marker << 14) | \system)
.endm

.globl agent11_modem_state_snapshot
agent11_modem_state_snapshot:
{
    r2 = add(r25,##-1064879408)
}
{
    r0 = memw(r2++#4)
}
MODEM_SYS_WRITE 33,0
{
    r0 = memw(r2++#8)
    r1 = memw(r2+#4)
}
MODEM_SYS_WRITE 34,0
MODEM_SYS_WRITE 36,1
{
    r0 = memw(r2++#8)
    r1 = memw(r2+#4)
}
MODEM_SYS_WRITE 37,0
MODEM_SYS_WRITE 38,1
{
    r0 = memw(r2+#0)
    r1 = memw(r2+#4)
}
MODEM_SYS_WRITE 39,0
MODEM_SYS_WRITE 43,1
{
    jumpr lr
}
