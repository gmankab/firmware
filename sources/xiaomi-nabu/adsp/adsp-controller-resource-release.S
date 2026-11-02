// SPDX-License-Identifier: GPL-3.0-only
// Hexagon physical controller cleanup and stop dispatch [0x5184,0x51dc).
// LLVM6 reassembles resource release and two state dispatch branches;
// S54 and STOP(R0) are checked typed privileged instructions.
.text
.globl adsp_controller_resource_release
adsp_controller_resource_release:
{ nop } // V66 R23 = S54
{
 immext(#0xf0015400)
 r22 = add(r23,##-0xffeabf8)
}
{ r22 = memw(r22+#0) }
{
 immext(#0xf000d300)
 r23 = add(r22,##-0xfff2ce0)
}
{ r22 = #4; memw(r23+#0) = r22.new }
{ dccleana(r23) }
{ dcinva(r23) }
{ jumpr lr }

.globl return_dispatch
return_dispatch:
{ call physical_tlb_copy }
{ call resource_release }
{ r0 = #0 }
{ jump stop_thread }
secondary:
{ call physical_tlb_copy }
{ call l2_global }
{ call resource_release }
{ r0 = #0 }
{ jump stop_thread }
stop_thread:
{ nop } // V66 stop(r0)
self_loop:
{ jump stop_thread }
.set physical_tlb_copy, return_dispatch-(0x51b0-0x5138)
.set resource_release, return_dispatch-(0x51b0-0x5184)
.set l2_global, return_dispatch-(0x51b0-0x5108)
