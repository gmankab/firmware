// SPDX-License-Identifier: GPL-3.0-only
// One-word Hexagon V60 slot accessor, file [0x4476b8,0x4476bc).
.text
.globl modem_late_aux_controller_slot
modem_late_aux_controller_slot:
{ r0 = memw(r0+#48)
  jumpr r31 }
