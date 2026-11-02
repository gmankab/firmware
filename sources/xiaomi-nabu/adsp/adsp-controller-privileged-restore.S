// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V66 privileged state restore, file [0x4ce8,0x4d1c).
// LLVM 6 does not encode PMUEVTCFG, P3:0, several system-register pairs or
// start(r23); narrow verified architectural encoders replace these sentinel
// NOP slots. Assembler owns all packet parse bits and relative branches.
.text
.globl adsp_controller_privileged_restore
adsp_controller_privileged_restore:
{ nop } // R24 = PMUEVTCFG
{ nop } // P3:0 = R24
{ nop } // R1:0 = S49:48
{ nop } // R25:24 = S45:44
{ nop } // SP = S47
{ nop } // LR:FP = S51:50
{ r23 = ##-1 }
{ nop } // start(r23)
{ jump controller_cpu_resume; nop; nop; nop }
.set controller_cpu_resume, adsp_controller_privileged_restore + 0x134
