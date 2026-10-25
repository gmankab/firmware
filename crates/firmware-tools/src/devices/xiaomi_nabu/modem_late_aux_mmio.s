// SPDX-License-Identifier: GPL-3.0-only
// Explicit MMIO controller descriptor initialization; Hexagon v60 source.
// ELF file [0x4462e0,0x4462f8). The return shares its instruction packet
// with two data-word stores as permitted by the target architecture.
.text
.globl modem_late_aux_mmio
modem_late_aux_mmio:
{ r2 = ##507593544; r3 = #2048 }
{
  jumpr r31
  memw(r2+#2048) = r2
  memw(r2+#2052) = r3
}
