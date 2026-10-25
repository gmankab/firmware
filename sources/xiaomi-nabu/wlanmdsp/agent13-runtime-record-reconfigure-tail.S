// SPDX-License-Identifier: GPL-3.0-only
// Delegate stream reconfiguration via two tail calls. File [0x44cb4,0x44cd4).
// All arithmetic/condition/predicate operations are explicit. No OEM opcode data.
.text
.globl agent13_wlan_record_reconfigure_tail
agent13_wlan_record_reconfigure_tail:
L_04cb4:
{
  r1 = #0x3
  r16 = r0
  memd(r29+#-0x10) = r17:16; 	allocframe(#0x8)
}
L_04cc0:
{
  call 0x17b0
}
L_04cc4:
{
  immext(#0x80)
  r1:0 = combine(##0xb4,r16)
  r17:16 = memd(r29+#0x0); 	deallocframe
}
L_04cd0:
{
  jump 0x17a0
}
