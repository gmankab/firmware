// ADSP Hexagon V66 [file 0x3bfcc,0x3c010): atomically enable the
// 64-bit event bit corresponding to the active controller channel.
// Uses Hexagon locked 64-bit memory load/store and predicate retry loop.
.text
.globl adsp_late_atomic_event_bit
adsp_late_atomic_event_bit:
{ memd(r29+#-16) = r17:16; allocframe(#8) }
{ r16 = r0; if (cmp.eq(r16.new,#0)) jump:nt finish_empty }
{ call platform_prepare; r1:0 = combine(r16,#1); r2 = #0; r17 = #0 }
{ immext(#0xf02ca000); r2 = ##4029460336; r1:0 = combine(#0,#1) }
{ r1:0 = asl(r1:0,r16); immext(#0xf02cb000); memw(r16<<#2+##4029477384) = r17 }
retry_locked:
{ r5:4 = memd_locked(r2) }
{ r5:4 = or(r5:4,r1:0) }
{ memd_locked(r2,p0) = r5:4 }
{ if (!p0) jump:nt retry_locked }
{ r17:16 = memd(r29+#0); dealloc_return }
finish_empty:
{ r17:16 = memd(r29+#0); dealloc_return }
.set platform_prepare, adsp_late_atomic_event_bit+0x36b8
