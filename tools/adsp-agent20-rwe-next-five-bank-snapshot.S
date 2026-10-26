// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V66 snapshot of five pairs of physical-memory controller banks.
// ADSP firmware file [0x105c4,0x10674), physical [0x8be0d5c4,0x8be0d674).
// Ten independent privileged memw_phys reads are generated from typed
// checked operands in the Python emitter, not copied as reference opcodes.
.text
.globl adsp_rwe_five_bank_snapshot
adsp_rwe_five_bank_snapshot:
{ immext(#0xf0003000); r0=memw(##0xf0003004) }
{ r2=#256; r4=#260 }
{ r1=lsr(r0,#11) }
{ nop } // checked R3=memw_phys(R2,R1)
{ nop } // checked R5=memw_phys(R4,R1)
{ immext(#0xf000d6c0); memw(##0xf000d6c0)=r3; immext(#0xf000d6c0); memw(##0xf000d6c4)=r5 }
{ r2=#264; r4=#268 }
{ nop } // checked R3=memw_phys(R2,R1)
{ nop } // checked R5=memw_phys(R4,R1)
{ immext(#0xf000d6c0); memw(##0xf000d6c8)=r3; immext(#0xf000d6c0); memw(##0xf000d6cc)=r5 }
{ r2=#272; r4=#276 }
{ nop } // checked R3=memw_phys(R2,R1)
{ nop } // checked R5=memw_phys(R4,R1)
{ immext(#0xf000d6c0); memw(##0xf000d6d0)=r3; immext(#0xf000d6c0); memw(##0xf000d6d4)=r5 }
{ r2=#4096; r4=#8192 }
{ nop } // checked R3=memw_phys(R2,R1)
{ nop } // checked R5=memw_phys(R4,R1)
{ immext(#0xf000d6c0); memw(##0xf000d6d8)=r3; immext(#0xf000d6c0); memw(##0xf000d6dc)=r5 }
{ r2=#12288; r4=#16384 }
{ nop } // checked R3=memw_phys(R2,R1)
{ nop } // checked R5=memw_phys(R4,R1)
{ immext(#0xf000d6c0); memw(##0xf000d6e0)=r3; immext(#0xf000d6c0); memw(##0xf000d6e4)=r5 }
{ jumpr lr }
