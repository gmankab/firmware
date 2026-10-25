// SPDX-License-Identifier: GPL-3.0-only
// Secondary boot stage, source-level reconstruction of control descriptor
// initialization and platform callback sequence in the modem auxiliary text.
// ELF file [0x446278,0x4462e0), VMA [0x1e400278,0x1e4002e0).
.text
.globl modem_late_aux_stage
modem_late_aux_stage:
.set aux_control_regs, modem_late_aux_stage + (0x2e0 - 0x278)
.set aux_clear_bytes, modem_late_aux_stage + (0x95b0 - 0x278)
.set aux_create_resource, modem_late_aux_stage + (0xc00 - 0x278)
.set aux_platform_init, modem_late_aux_stage + (0x6770 - 0x278)
.set aux_publish_resource, modem_late_aux_stage + (0xc18 - 0x278)
.set aux_start_worker, modem_late_aux_stage + (0x646c - 0x278)
.set aux_run_worker, modem_late_aux_stage + (0x8020 - 0x278)
{ memd(r29+#-16) = r17:16; allocframe(#96) }
{ call aux_control_regs; memw(r29+#84) = #0 }
{ r2 = #80; r1 = #0; r16 = add(r29,#0) }
{ call aux_clear_bytes; r0 = add(r16,#4) }
{ r1 = add(r29,#84) }
{ r0 = #0; r2 = add(r29,#0) }
{ memw(r16+#40) = ##507511544 }
{ memw(r16+#44) = ##507511728 }
{ call aux_create_resource; memw(r29+#0) = #40 }
{ call aux_platform_init; r0 = #1 }
{ call aux_publish_resource }
{ r1:0 = combine(#4,##507576320) }
{ call aux_start_worker; r1:0 = combine(#4,#0); r3:2 = combine(#0,#39) }
{ call aux_run_worker; r0 = #0 }
{ r17:16 = memd(r29+#88); dealloc_return }
