// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V60 post-TLB system setup; original operations with explicit packets.
// ISA-only expressions encode privileged register transfers. Firmware machine
// words from the OEM MBN are never imported into this assembly source.
.text
.macro HEX_SYSTEM_READ general, system, marker=3
    .if (\general < 0) || (\general > 31) || (\system < 0) || (\system > 127)
        .error "out-of-range system-register read"
    .endif
    .word (0x6e800000 | (\system << 16) | (\marker << 14) | \general)
.endm
.macro HEX_SYSTEM_WRITE system, general, marker=3
    .if (\general < 0) || (\general > 31) || (\system < 0) || (\system > 127)
        .error "out-of-range system-register write"
    .endif
    .word (0x67000000 | (\general << 16) | (\marker << 14) | \system)
.endm
.macro HEX_SYSTEM_PAIR_WRITE system, pairlow, marker=3
    .if (\pairlow < 0) || (\pairlow > 30) || ((\pairlow % 2) != 0) || (\system < 0) || (\system > 31)
        .error "invalid privileged system-register pair"
    .endif
    .word (0x6d000000 | (\pairlow << 16) | (\marker << 14) | \system)
.endm
.globl modem_post_tlb
modem_post_tlb:
{ isync }
HEX_SYSTEM_READ 0,18
{
  r3:2 = combine(#0,#0)
  r0 = or(r0,#64)
}
HEX_SYSTEM_WRITE 18,0
HEX_SYSTEM_PAIR_WRITE 30,2
{ isync }
{ c1:0 = r3:2 }
{ c3:2 = r3:2 }
HEX_SYSTEM_PAIR_WRITE 4,2
HEX_SYSTEM_WRITE 28,2
HEX_SYSTEM_WRITE 7,2
{ usr = r2 }
{
  r0 = ##-4096
  r26 = ##-1065353216
}
{
  r24 = pc
  r27 = ##-1032593408
  r26 = and(r26,r0)
}
{ r24 = and(r24,r0) }
{ r25 = sub(r24,r26) }
{ r17 = ##256 }
{ r17 = or(r17,##10) }
HEX_SYSTEM_WRITE 25,17
{ r17 = ##511 }
HEX_SYSTEM_WRITE 23,17
.set modem_followup, modem_post_tlb + 0xad4
{ call modem_followup }
