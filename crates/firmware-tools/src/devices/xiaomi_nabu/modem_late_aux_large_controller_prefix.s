// SPDX-License-Identifier: GPL-3.0-only
// Source-encoded Hexagon V60 entry/check section of a larger controller
// command handler. File [0x4476bc,0x447724), VMA [0x1e4016bc,0x1e401724).
// Later branches and function body are NOT yet independently encoded.
.text
.globl modem_late_aux_large_controller_prefix
modem_late_aux_large_controller_prefix:
.set aux_logged_error, modem_late_aux_large_controller_prefix - 0xf1c
.set aux_controller_continuation, modem_late_aux_large_controller_prefix + 0x13a0
.set aux_normal_mode, modem_late_aux_large_controller_prefix + 0x68
.set aux_missing_registry, modem_late_aux_large_controller_prefix + 0x58
{ r16 = r0
  memd(r29+#-16) = r17:16; allocframe(#152) }
{ r20 = r2
  memd(r29+#128) = r21:20; memd(r29+#136) = r19:18 }
{ r19 = ##507617104
  p0 = cmp.eq(r20,#-1)
  memd(r29+#104) = r27:26 }
{ r27:26 = combine(r3,r1)
  memd(r29+#120) = r23:22
  memd(r29+#112) = r25:24 }
{ if (!p0) jump:nt aux_normal_mode
  memw(r29+#92) = #0
  memw(r29+#88) = #0 }
{ r4 = memw(r19+#0)
  if (cmp.eq(r4.new,#0)) jump:nt aux_missing_registry }
{ r1:0 = combine(##507576841,#3)
  r2 = #255 }
{ r3 = ##507576854
  callr r4 }
{ call aux_logged_error
  r0 = ##507576854; r17 = #0 }
{ jump aux_controller_continuation }
