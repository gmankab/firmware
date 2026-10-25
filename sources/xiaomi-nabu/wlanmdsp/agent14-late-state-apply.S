// SPDX-License-Identifier: GPL-3.0-only
// Agent 20-14: file [0x1001c4,0x100240),
// runtime [0xb00c01c4,0xb00c0240).
// Apply a low-byte status command only when the record context ID matches.
// The high bits select a qualifier; flag-bit clear and state writes are
// observable separately. Matching tokens apply these status transitions:
// 0x01 or 0x80 => state=2; 0x02 or 0x100 => clear flag bit0, state=1.
// Other status words leave those context fields unchanged. The return value
// is not interpreted; the diagnostic helpers remain external.
// Explicit V66 mnemonic/packet source; no reference bytes are used at build time.
.text
.globl wlan_late_state_apply
wlan_late_state_apply:
{ r17:16 = combine(r1,r2); memd(r29+#-0x10) = r17:16; allocframe(#0x10) }
{ r2 = memw(r16+#0x280) }
{ r3 = memw(r17+#0x14); if (!cmp.eq(r3.new,r2)) jump:t restore_return }
{ r1 = #0x2411; r0 = ##0x6468; r2 = memw(r17+#0x0) }
{ call 0x15afd0; memw(r29+#0x0) = r2 }
{ call -0xbd5a4; r0 = ##0xb02b6c88; r1 = memw(r17+#0x0) }
{ r2 = memw(r17+#0x0) }
{ r1:0 = bitsplit(r2,#0x8) }
{ if (p0.new) r3 = #0xff; p0 = cmp.eq(r1,#0x1); if (p0.new) jump:nt qualified_command }
{ p0 = cmp.eq(r1,#0x0); if (!p0.new) jump:nt restore_return }
{ p0 = cmp.eq(r0,#0x1); if (p0.new) jump:nt set_state_two }
{ p0 = cmp.eq(r0,#0x2); if (p0.new) jump:nt clear_flag_and_activate }
{ p0 = cmp.eq(r0,#0x80); jump restore_return; if (p0.new) memw(r16+#0x8) = #0x2 }
qualified_command:
{ p0 = bitsclr(r2,r3); if (!p0.new) jump:t restore_return }
clear_flag_and_activate:
{ r2 = add(r16,#0x284) }
{ memw(r2+#0x0) = clrbit(#0x0) }
{ jump restore_return; memw(r16+#0x8) = #0x1 }
set_state_two:
{ memw(r16+#0x8) = #0x2 }
restore_return:
{ r17:16 = memd(r29+#0x8); dealloc_return }
