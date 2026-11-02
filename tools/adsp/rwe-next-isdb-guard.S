// SPDX-License-Identifier: GPL-3.0-only
// ADSP file [0x103a4,0x103d0), physical [0x8be0d3a4,0x8be0d3d0).
// Read SGP0, guard debugger enable status, clear SYSCFG bit 5 and wait for
// the ISDB enable bit to be deasserted. Unsupported register transfers use
// typed V66 encoders after exact LLVM6 NOP-location validation.
.text
.globl adsp_rwe_isdb_guard
adsp_rwe_isdb_guard:
{ nop; memw(r0+#0x90)=#0 } // R29=SGP0 is the first instruction
{ nop } // R0=ISDBEN
{ p0=tstbit(r0,#0); if(!p0.new) jump:t done }
{ nop } // R1=SYSCFG
{ r1=clrbit(r1,#5) }
{ nop } // SYSCFG=R1
{ isync }
watch_debug:
{ nop } // R0=ISDBEN
{ p0=tstbit(r0,#4); if(p0.new) jump:t watch_debug }
done:
