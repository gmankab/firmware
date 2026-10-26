// SPDX-License-Identifier: GPL-3.0-only
// Retrieve an optional command callback from a descriptor, checking its
// sentinel tag and forwarding unusual layouts to the platform dispatcher.
.text
.globl adsp_late_call_descriptor_dispatch
adsp_late_call_descriptor_dispatch:
{ allocframe(#0) }
{ r2 = memw(r0+#16) }
{ immext(#3755990976); p0 = cmp.eq(r2,##-538976289); if (!p0.new) jump:t callback_ready; if (!p0.new) r2 = memw(r0+#24) }
{ jump platform_descriptor_dispatch; r0 = memw(r0+#12); deallocframe }
return_direct:
callback_ready:
{ callr r2 }
{ dealloc_return }
.set platform_descriptor_dispatch, adsp_late_call_descriptor_dispatch+0xecd3c
