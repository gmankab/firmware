// SPDX-License-Identifier: GPL-3.0-only
// Write controller-ready event and thin forwarding trampoline [0x64b4,0x64d8).
.text
.globl adsp_agent20_rwe_state_ready
adsp_agent20_rwe_state_ready:
{ r0=#1; immext(#0xf000d300); memw(r1=##0xf000d320)=r0.new }
{ dccleana(r1) }
{ dcinva(r1) }
{ r0=memw(r1+#0) }
{ jumpr lr }
{ r1=add(r1,#16) }
{ jump manager_handoff }
.set manager_handoff,adsp_agent20_rwe_state_ready+(0x8be21904-0x8be034b4)
