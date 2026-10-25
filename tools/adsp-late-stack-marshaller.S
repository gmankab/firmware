// ADSP Hexagon V66 [file 0x3d05c,0x3d098): copy six caller stack slots
// into a 24-byte outgoing argument frame before forwarding platform work.
.text
.globl adsp_late_stack_marshaller
adsp_late_stack_marshaller:
{ allocframe(#24) }
{ r6 = memw(r29+#52); memw(r29+#20) = r6.new }
{ r8 = memw(r29+#44); memw(r29+#12) = r8.new }
{ r12 = memw(r29+#36); memw(r29+#4) = r12.new }
{ r7 = memw(r29+#48); memw(r29+#16) = r7.new }
{ r9 = memw(r29+#40); memw(r29+#8) = r9.new }
{ call platform_marshal; r13 = memw(r29+#32); memw(r29+#0) = r13.new }
{ dealloc_return }
.set platform_marshal, adsp_late_stack_marshaller+0xed86c
