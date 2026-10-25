// SPDX-License-Identifier: GPL-3.0-only
// Packetized modem early dispatch and low-level cache handling.
.text
.macro HEX_SYSTEM_READ general, system, marker=3
    .if (\general < 0) || (\general > 31) || (\system < 0) || (\system > 127)
        .error "invalid system register read"
    .endif
    .word (0x6e800000 | (\system << 16) | (\marker << 14) | \general)
.endm
.globl modem_boot_dispatch
modem_boot_dispatch:
.set modem_ready_handler, modem_boot_dispatch+0x5c4
{ r0 = memw(r25+##-1064935416) }
{
 p0 = cmp.eq(r0,#2)
 p1 = cmp.eq(r0,#0)
}
{ if (p0) jump:nt modem_ready_handler }
// Y2_l2kill, an external L2-cache-kill ISA instruction not accepted by LLVM6 MC.
// 1010100000100000PP00000000000000: opcode class plus packet parse bits.
.word (0xa8200000 | (3 << 14))
{ syncht }
{ if (!p1) jump:nt modem_ready_handler }
// Reset the current bootstrap handshake state only if the controller has not
// already installed a valid address. Keep condition-producing packets intact.
{
 r23 = add(r25,##-1064885440)
 r0 = memw(r25+##-1064935408)
}
.set current_handshake_unset, modem_boot_dispatch+0x44
.set handshake_done, modem_boot_dispatch+0x48
{
 p0 = cmp.eq(r0,#-1)
 if (p0.new) jump:nt current_handshake_unset
 r1 = memw(r23+#0)
}
{
 p0 = cmp.eq(r1,#0)
 if (p0.new) memw(r23+#0) = r0
}
{ jump handshake_done }
current_handshake_unset:
{ memw(r23+#0) = r27 }
// Platform-version dispatch: read S29 then choose one of two controller
// resource tables according to the extracted revision code.
HEX_SYSTEM_READ 2,29,1
{ r3 = ##135429 }
{
 r2 = extract(r2,#24,#0)
 r12 = r2
 r4 = memw(r25+##-1064821884)
}
.set use_alternate_resource, modem_boot_dispatch+0x80
.set descriptor_ready, modem_boot_dispatch+0x9c
{
 p0 = cmp.eq(r2,r3); if (!p0.new) jump:nt use_alternate_resource
 r3 = ##139525
}
{ p0 = cmp.eq(r4,#0); if (p0.new) jump:nt descriptor_ready }
{
 r1 = r4
 jump descriptor_ready
 r0 = memw(r25+##-1064821856)
}
use_alternate_resource:
{
 p0 = cmp.eq(r2,r3); if (!p0.new) jump:nt descriptor_ready
 r4 = memw(r25+##-1064821880)
}
{ p0 = cmp.eq(r4,#0); if (p0.new) jump:nt descriptor_ready }
{ r1 = r4; r0 = memw(r25+##-1064821852) }
