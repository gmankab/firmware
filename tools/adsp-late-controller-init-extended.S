// SPDX-License-Identifier: GPL-3.0-only
// ADSP firmware file [0x3f4d0,0x3f64c) -- controller startup, QuRT handle,
// four-entry descriptor initializer and hardware-loop clearing procedure.
// Control-flow and literal constants are semantic source operands, not raw
// OEM opcode words or a disassembler dump.
.text
.globl adsp_late_controller_init_extended
adsp_late_controller_init_extended:
{ immext(#4030007744); r0 = ##-264959552; memd(r29+#-16) = r17:16; allocframe(#16) }
{ r16 = #0; memd(r29+#0) = r19:18 }
{ call platform_controller_init; immext(#4032983552); memw(##4032983596) = r16 }
{ loop0(loop_clear,#4); immext(#4030801088); r2 = ##4030801088; r1:0 = combine(#0,#0) }
loop_clear:
{ memb(r2+#-16) = r16; memw(r2+#-12) = r16 }
{ memw(r2+#-8) = r16; memd(r2+#8) = r1:0 }
{ nop; memd(r2++#32) = r1:0 }:endloop0
{ immext(#4028651776); r3 = memw(##4028651808); if (!cmp.eq(r3.new,#-1)) jump:t initialized }
{ immext(#4026683520); r3 = memw(##4026683536); immext(#4028651776); memw(##4028651808) = r3.new }
{ immext(#4026690880); r2 = memw(##4026690932); immext(#4028651776); memw(##4028651804) = r2.new }
initialized:
{ call platform_fetch; r0 = #1024; r17 = #1024 }
{ r16 = r0; if (cmp.eq(r16.new,#0)) jump:nt out_of_memory }
{ immext(#4030007616); r18 = ##-264959648; immext(#16711680); r2 = ##16711680 }
{ r19 = #0; immext(#4028152320); r1 = ##-266814952; memw(r18+#40) = r2 }
{ r0 = add(r18,#24); immext(#4294901504); r3 = ##4294901504; r2 = #16 }
{ memb(r18+#24) = r19; memw(r18+#44) = r3 }
{ memw(r18+#48) = r19; memw(r18+#52) = r19 }
{ call platform_register_buffer; memh(r18+#56) = r19 }
{ immext(#65472); r2 = ##65533; immext(#4029458112); r3 = memw(##4029458124) }
{ immext(#4029486208); r0 = memw(##4029486224); memb(r18+#39) = r19 }
{ memw(r18+#48) = r17; memw(r18+#52) = r16 }
{ memh(r18+#42) = r3; memh(r18+#46) = r2 }
{ memb(r18+#40) = r0 }
{ r2 = ugp }
{ r0 = r18; immext(#4027746752); r2 = ##-267220544; memb(r18+#16) = r19 }
{ call platform_dispatch; memw(r18+#4) = r2 }
{ r1:0 = combine(r18,#3); immext(#4027746752); r2 = ##-267220544 }
{ immext(#4030007744); r3 = ##-264959536; immext(#4030007872); r4 = ##-264959424 }
{ call platform_notify; memw(r18+#72) = r4 }
{ call platform_shutdown; immext(#4028152320); r0 = ##-266814925 }
{ r1:0 = combine(#0,#0); r3:2 = combine(#0,#-1) }
{ immext(#4030007680); memd(##4030007736) = r1:0; immext(#4030007680); memd(##4030007728) = r3:2 }
{ r17:16 = memd(r29+#8); r19:18 = memd(r29+#0) }
{ dealloc_return }
out_of_memory:
{ call platform_shutdown; immext(#4028151808); r0 = ##-266815480 }
{ call platform_failure; immext(#4028152320); r0 = ##-266814939; r1 = #298 }
{ immext(#4028152384); r1:0 = combine(#1,##4028152408); jump final_fallback }
.set platform_controller_init, adsp_late_controller_init_extended+0xeb0e4
.set platform_fetch, adsp_late_controller_init_extended-0x4964
.set platform_register_buffer, adsp_late_controller_init_extended+0x3aca8
.set platform_dispatch, adsp_late_controller_init_extended+0xea310
.set platform_notify, adsp_late_controller_init_extended+0x3890
.set platform_shutdown, adsp_late_controller_init_extended+0xe795c
.set platform_failure, adsp_late_controller_init_extended-0xbdc
.set final_fallback, adsp_late_controller_init_extended+0x29c
