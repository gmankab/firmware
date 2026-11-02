// SPDX-License-Identifier: GPL-3.0-only
// Flush cached controller state to a fixed MMIO firmware flag byte. The
// fixed address is an immediate operand, not a borrowed memory payload.
.text
.globl adsp_late_controller_state_flush
adsp_late_controller_state_flush:
{ r16 = r0; memd(r29+#-16) = r17:16; allocframe(#8) }
{ call platform_get_flush_state; immext(#4029460480); r0 = ##-265506816 }
{ memb(r0+#25) = r16; r17:16 = memd(r29+#0) }:mem_noshuf
{ dealloc_return }
.set platform_get_flush_state, adsp_late_controller_state_flush+0xa88
