// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V60 callback trampolines, file [0x3aa8c,0x3aab4).
// Both paths select callback parameters and jump to the previously reconstructed
// callback frame at 0x3aa34. The immediate values are the mapped absolute
// controller addresses; they are operands, not original instruction words.
.text
.globl agent11_modem_controller_callback_trampoline_primary
.set agent11_controller_callback_frame, agent11_modem_controller_callback_trampoline_primary - 0x58
agent11_modem_controller_callback_trampoline_primary:
{ r4 = ##-1065080276; r0 = r4 }
{ r5 = ##-1065080596; jump agent11_controller_callback_frame }
.globl agent11_modem_controller_callback_trampoline_secondary
agent11_modem_controller_callback_trampoline_secondary:
{ r4 = ##-1065080968; r0 = r4 }
{ r5 = ##-1065080284; jump agent11_controller_callback_frame }
