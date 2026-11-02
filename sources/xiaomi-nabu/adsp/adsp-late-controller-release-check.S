// SPDX-License-Identifier: GPL-3.0-only
// ADSP V66 controller context: validate the registered command tag, dispatch
// three resource-release variants, then hand off with the original callback
// frame restored. [ADSP ELF file 0x3e4d0,0x3e5a8), executable text only.
.text
.globl adsp_late_controller_release_check
adsp_late_controller_release_check:
{ r17:16 = combine(r3,r2); memd(r29+#-16) = r17:16; allocframe(#32) }
{ memd(r29+#16) = r19:18; r18 = r0; immext(#0xf02cb000); r0 = ##-265493296 }
{ call read_command_registry }
{ r3:2 = combine(#0,#0); r19 = #13; r1:0 = memd(r18+#40) }
{ p0 = cmp.eq(r1:0,r3:2); if (p0.new) jump:nt tagged_command; if (p0.new) r2 = memw(r18+#16) }
again:
{ call callback_update; immext(#0xf02cb000); r0 = ##-265493296 }
{ r0 = r19; r17:16 = memd(r29+#24); r19:18 = memd(r29+#16) }
{ dealloc_return }
tagged_command:
{ immext(#0xada12800); r3 = ##-1381126739; if (cmp.eq(r3.new,r2)) jump:nt special_release }
{ r1:0 = combine(#-1,#-100); immext(#0xede85e80); p0 = cmp.eq(r2,##-303174163); if (!p0.new) jump:nt again }
{ r2 = memw(r18+#20); if (!cmp.eq(r2.new,#1)) jump:t bounded_release }
{ r1:0 = combine(#0,#0); r5:4 = combine(r17,r16); r2 = memw(r18+#68); memw(r29+#8) = r2.new }
{ call release_callback_a; memd(r29+#0) = r1:0; r0 = r18; r3:2 = combine(#0,#0) }
{ r19 = r0; jump again }
special_release:
{ call release_callback_b; r0 = r18; r3:2 = combine(r17,r16) }
{ r19 = r0; jump again }
bounded_release:
{ immext(#0x914cb000); r2 = ##-1857819748 }
{ r1:0 = add(r17:16,r1:0); immext(#0x000d5500); r3 = ##873812; r19 = #4 }
{ p0 = cmp.gtu(r1:0,r3:2); if (p0.new) jump:t again }
{ r1:0 = combine(#0,#0); r5:4 = combine(#0,#0); r2 = memw(r18+#68); memw(r29+#8) = r2.new }
{ call release_callback_a; r3:2 = combine(r17,r16); r0 = r18; memd(r29+#0) = r1:0 }
{ r19 = r0; jump again }
.set read_command_registry, adsp_late_controller_release_check+0xe83d0
.set callback_update, adsp_late_controller_release_check+0xe8510
.set release_callback_a, adsp_late_controller_release_check-0xa00
.set release_callback_b, adsp_late_controller_release_check-0x7d0
