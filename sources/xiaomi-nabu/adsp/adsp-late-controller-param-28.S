// SPDX-License-Identifier: GPL-3.0-only
// Independent Hexagon controller-parameter accessor: index 28.
// The actual function returns a platform register value without retaining
// external firmware bytes; original target calculated from the relocatable
// call displacement to the shared parameter-fetch entrypoint.
.text
.globl adsp_late_controller_param_28
adsp_late_controller_param_28:
{ r1 = #0; r2 = #28; memd(r29+#-16) = r17:16; allocframe(#8) }
{ call platform_fetch_parameter; r16 = r0 }
{ r0 = r16; r17:16 = memd(r29+#0); dealloc_return }
.set platform_fetch_parameter, adsp_late_controller_param_28+0x102b78
