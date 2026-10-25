// SPDX-License-Identifier: GPL-3.0-only
// Qualcomm WLAN Hexagon startup, independently re-authored from the observed
// QuRT initialization and loop behavior. Numeric PC-relative displacements
// reflect this exact firmware revision. Packet braces and the forced extender
// below are significant: changing them changes the emitted binary.
.text
.globl wlan_entry
wlan_entry:
// Establish TLS, stack, frame limit and the global pointer before the
// first QuRT loader callback.
{
  r29 = add(pc,##8039416)
  r30 = #0
  r0 = #0
}
{ framelimit = r0 }
{ r0 = add(pc,##3538924) }
{ r1 = add(pc,##4132) }
{ gp = r0; callr r1 }
{ r0 = add(pc,##3547092) }
{ r1 = add(pc,##8039368) }
{ call clear_runtime_data }
{ r0 = add(pc,##3544768) }
{ r1 = add(pc,##3544760) }
{ call clear_runtime_data }
{ r0 = add(pc,##3541724) }
{ memw(r0+#0) = r16 }
{ r2 = add(pc,##4304) }
{ callr r2 }
{ r0 = add(pc,##3534764) }
{ r1 = add(pc,##3534760) }
{ r0 = memw(r0+#0) }
{ r1 = memw(r1+#0) }
{ call heap_reservation_stub }
{ r2 = add(pc,##4296) }
{ r0 = add(pc,##8035176) }
{ r1 = ##4096; callr r2 }
idle_forever:
{ jump idle_forever }
// Return address is r31. Round the start up to an eight-byte boundary,
// store any leading/trailing unaligned bytes, and clear aligned words.
// A backward jump and store share a packet: this VLIW grouping is required.
clear_runtime_data:
{ r3 = add(r0,#7); r4 = #0 }
{ r3 = insert(r4,#3,#0); r5 = #0 }
{ p0 = cmp.gt(r3,r1) }
{ if (p0) r3 = add(r1,#0) }
first_byte_loop:
{ p0 = cmp.eq(r0,r3); if (p0.new) jump:nt first_bytes_done }
{ jump first_byte_loop; memb(r0++#1) = r4 }
first_bytes_done:
{ p0 = cmp.eq(r0,r1); if (p0.new) jump:nt clear_done; r3 = r1 }
{ r3 = insert(r4,#3,#0) }
clear_words_loop:
{ p0 = cmp.eq(r0,r3); if (p0.new) jump:nt clear_words_done }
{ jump clear_words_loop; memd(r0++#8) = r5:4 }
clear_words_done:
{ r3 = r1 }
last_bytes_loop:
{ p0 = cmp.eq(r0,r3); if (p0.new) jump:nt clear_done }
{ jump last_bytes_loop; memb(r0++#1) = r4 }
clear_done:
{ jumpr r31 }
// Original runtime heap initialization jumps to vaddr 0xb0001140.
// The extended *relative* displacement is 0x1048 from instruction 0xf8.
heap_reservation_stub:
{ jump ##0x1048 }
// Initialize registered constructors from the first function pointer table.
wlan_init:
{ allocframe(#8) }
{ memw(r29+#0) = r27 }
{ r0.h = #45105 }
{ r0.l = #5760 }
{ r1 = memw(r0+#0) }
{ p0 = cmp.eq(r1,#0); if (p0.new) jump:nt ctor_invocation_done }
{ r3 = #0; r2 = #0; r1 = #0 }
{ r28.h = #45098 }
{ r28.l = #22108 }
{ callr r28 }
ctor_invocation_done:
{ r27.h = #45108 }
{ r27.l = #4 }
invoke_finalizer:
{ r27 = add(r27,#-4) }
{ r0 = memw(r27+#0) }
{ p0 = cmp.eq(r0,#0); if (p0.new) jump:nt ctor_done }
{ callr r0 }
{ jump invoke_finalizer }
ctor_done:
{ r27 = memw(r29+#0); deallocframe }
{ jumpr r31 }
