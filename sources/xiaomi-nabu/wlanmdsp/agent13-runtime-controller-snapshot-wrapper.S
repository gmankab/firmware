// SPDX-License-Identifier: GPL-3.0-only
// Parsed mnemonic operands from reference disassembler; typed Hexagon packet source.
.text
.globl agent13_wlan_controller_snapshot_wrapper
agent13_wlan_controller_snapshot_wrapper:
L_00000:
{
  call 0x216af0
  r1 = #0; 	allocframe(#0) }
L_00008:
{
  call 0x4844
  r2 = r0; 	r1:0 = combine(#2,#2) }
L_00010:
{
  r0 = zxth(r0); 	dealloc_return }
