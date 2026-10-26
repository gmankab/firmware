// SPDX-License-Identifier: GPL-3.0-only
// Hexagon V60 controller flags and short-circuit branch, [0x447d28,0x447d38).
// The reference uses one *four-word* V60 packet including two independent
// branches and a new-value predicate. Both LLVM6 and LLVM22 reject that
// combination for the assembler slot model, so we source-assemble two legal
// packets plus the last old-value predicate, then apply checked ISA bitfield
// transformations to preserve the original single packet and its .new use.
// In the original packet the last instruction is `if (p0.new) r23 = #0`;
// the temporary `if (p0)` syntax below permits standalone LLVM MC assembly.
// The emitter restores this typed `.new` predicate only after checking that
// the immediately preceding source packet creates P0 via bitsclr(r2,r3).
.text
.globl modem_late_aux_large_controller_phase13_gap
modem_late_aux_large_controller_phase13_gap:
.set process_remaining_resources, modem_late_aux_large_controller_phase13_gap + 0x24
.set finalize_scan_result, modem_late_aux_large_controller_phase13_gap + 0x5c
{
  if (!p0.new) jump:t process_remaining_resources
  p0 = bitsclr(r2,r3)
}
{jump finalize_scan_result}
{if (p0) r23 = #0}
