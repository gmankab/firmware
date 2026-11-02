// SPDX-License-Identifier: GPL-3.0-only
// Source reconstruction of the 8-byte Hexagon V60 controller mode-3 handler
// [file 0x3a40c,0x3a414). The extracted unsigned field is r15[8+:7].
// The original parse bits put EXTRACTU and RTE in ONE packet; LLVM6 lacks
// a mnemonic for privileged RTE and emits EXTRACTU as a standalone packet.
// The external assembler/verifier validates EXTRACTU's architecture fields,
// then changes only its packet marker from end to continuation.
.text
.macro AGENT11_MODE_RTE marker=3
  .if (\marker != 3)
    .error "Hexagon exception-return must end this packet"
  .endif
  .word (0x57e00000 | (\marker << 14))
.endm
.globl agent11_mode3_extractu_and_exception_return
agent11_mode3_extractu_and_exception_return:
{ r0 = extractu(r15,#7,#8) }
AGENT11_MODE_RTE 3
