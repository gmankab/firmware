// SPDX-License-Identifier: GPL-3.0-only
// Modem 2nd RWE PT_LOAD exception dispatch and return vector table.
// File [0x38000,0x38100), VMA [0xc0840000,0xc0840100),
// physical [0x8e040000,0x8e040100). Only 252 bytes are instructions;
// a separate zero word after the branch table is architectural padding.
// All jump destinations are symbolic *control flow*, not reference opcodes.
.text
.globl agent11_modem_rwe_irq_vectors
agent11_modem_rwe_irq_vectors:
// Jump targets use the natural Hexagon code addresses and the original
// vector-table base; LLVM6 fixes the displacement from each packet start.
.macro MODEM_RWE_JUMP_TO target
  { jump agent11_modem_rwe_irq_vectors + (\target - 0x38000) }
.endm
MODEM_RWE_JUMP_TO 0x102b0 // table +0x00
MODEM_RWE_JUMP_TO 0x3810c // table +0x04
MODEM_RWE_JUMP_TO 0x38168 // table +0x08
MODEM_RWE_JUMP_TO 0x381c0 // table +0x0c
MODEM_RWE_JUMP_TO 0x390cc // table +0x10
MODEM_RWE_JUMP_TO 0x381c0 // table +0x14
MODEM_RWE_JUMP_TO 0x390d4 // table +0x18
MODEM_RWE_JUMP_TO 0x381c0 // table +0x1c
MODEM_RWE_JUMP_TO 0x38484 // table +0x20
MODEM_RWE_JUMP_TO 0x38200 // table +0x24
MODEM_RWE_JUMP_TO 0x381c0 // table +0x28
MODEM_RWE_JUMP_TO 0x38178 // table +0x2c
MODEM_RWE_JUMP_TO 0x38464 // table +0x30
MODEM_RWE_JUMP_TO 0x381c0 // table +0x34
MODEM_RWE_JUMP_TO 0x381c0 // table +0x38
MODEM_RWE_JUMP_TO 0x381c0 // table +0x3c
MODEM_RWE_JUMP_TO 0x38198 // table +0x40
MODEM_RWE_JUMP_TO 0x38198 // table +0x44
MODEM_RWE_JUMP_TO 0x38d04 // table +0x48
MODEM_RWE_JUMP_TO 0x38198 // table +0x4c
MODEM_RWE_JUMP_TO 0x38198 // table +0x50
MODEM_RWE_JUMP_TO 0x38198 // table +0x54
MODEM_RWE_JUMP_TO 0x388c0 // table +0x58
MODEM_RWE_JUMP_TO 0x388c0 // table +0x5c
MODEM_RWE_JUMP_TO 0x388c0 // table +0x60
MODEM_RWE_JUMP_TO 0x388c0 // table +0x64
MODEM_RWE_JUMP_TO 0x388c0 // table +0x68
MODEM_RWE_JUMP_TO 0x38198 // table +0x6c
MODEM_RWE_JUMP_TO 0x38198 // table +0x70
MODEM_RWE_JUMP_TO 0x38198 // table +0x74
MODEM_RWE_JUMP_TO 0x38198 // table +0x78
MODEM_RWE_JUMP_TO 0x38198 // table +0x7c
MODEM_RWE_JUMP_TO 0x38198 // table +0x80
MODEM_RWE_JUMP_TO 0x38198 // table +0x84
MODEM_RWE_JUMP_TO 0x38198 // table +0x88
MODEM_RWE_JUMP_TO 0x38198 // table +0x8c
MODEM_RWE_JUMP_TO 0x38198 // table +0x90
MODEM_RWE_JUMP_TO 0x38198 // table +0x94
MODEM_RWE_JUMP_TO 0x38198 // table +0x98
MODEM_RWE_JUMP_TO 0x38198 // table +0x9c
MODEM_RWE_JUMP_TO 0x38198 // table +0xa0
MODEM_RWE_JUMP_TO 0x38198 // table +0xa4
MODEM_RWE_JUMP_TO 0x38198 // table +0xa8
MODEM_RWE_JUMP_TO 0x38198 // table +0xac
MODEM_RWE_JUMP_TO 0x38198 // table +0xb0
MODEM_RWE_JUMP_TO 0x38198 // table +0xb4
MODEM_RWE_JUMP_TO 0x38198 // table +0xb8
MODEM_RWE_JUMP_TO 0x38198 // table +0xbc
MODEM_RWE_JUMP_TO 0x38850 // table +0xc0
// Reserved null separator at 0x380c4 (not source executable).
.space 4,0
// LLVM6's Hexagon MC omits RTE, so use the architectural control-flow
// template with strict packet-marker validation. No OEM code word included.
.macro MODEM_RWE_RETURN_FROM_EXCEPTION packet_mark=3
  .if (\packet_mark != 3)
    .error "standalone Hexagon RTE must end its packet"
  .endif
  .word (0x57e00000 | (\packet_mark << 14))
.endm
MODEM_RWE_RETURN_FROM_EXCEPTION 3 // return-from-exception vector +0xc8
MODEM_RWE_RETURN_FROM_EXCEPTION 3 // return-from-exception vector +0xcc
MODEM_RWE_RETURN_FROM_EXCEPTION 3 // return-from-exception vector +0xd0
MODEM_RWE_RETURN_FROM_EXCEPTION 3 // return-from-exception vector +0xd4
MODEM_RWE_RETURN_FROM_EXCEPTION 3 // return-from-exception vector +0xd8
MODEM_RWE_RETURN_FROM_EXCEPTION 3 // return-from-exception vector +0xdc
MODEM_RWE_RETURN_FROM_EXCEPTION 3 // return-from-exception vector +0xe0
MODEM_RWE_RETURN_FROM_EXCEPTION 3 // return-from-exception vector +0xe4
MODEM_RWE_RETURN_FROM_EXCEPTION 3 // return-from-exception vector +0xe8
MODEM_RWE_RETURN_FROM_EXCEPTION 3 // return-from-exception vector +0xec
MODEM_RWE_RETURN_FROM_EXCEPTION 3 // return-from-exception vector +0xf0
MODEM_RWE_RETURN_FROM_EXCEPTION 3 // return-from-exception vector +0xf4
MODEM_RWE_RETURN_FROM_EXCEPTION 3 // return-from-exception vector +0xf8
MODEM_RWE_RETURN_FROM_EXCEPTION 3 // return-from-exception vector +0xfc
// Next disjoint routine begins at 0x38108; [0x38100,0x38108) = fill.
