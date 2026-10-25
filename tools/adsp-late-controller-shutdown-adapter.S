// ADSP Hexagon V66 [file 0x3d098,0x3d0c8): load the shared runtime
// controller, invoke its cleanup handler and tail-jump to the common close.
.text
.globl adsp_late_controller_shutdown_adapter
adsp_late_controller_shutdown_adapter:
{ memd(r29+#-16) = r17:16; allocframe(#8) }
{ r16 = r0; if (cmp.eq(r16.new,#0)) jump:nt return_empty }
{ immext(#0xf02ca000); r2 = memw(##4029457660) }
{ r2 = add(r16,r2) }
{ call platform_shutdown; r0 = memw(r2+#72) }
{ r0 = r16; r17:16 = memd(r29+#0); deallocframe }
{ jump platform_tail }
return_empty:
{ r17:16 = memd(r29+#0); dealloc_return }
.set platform_shutdown, adsp_late_controller_shutdown_adapter+0xec088
.set platform_tail, adsp_late_controller_shutdown_adapter-0x20f8
