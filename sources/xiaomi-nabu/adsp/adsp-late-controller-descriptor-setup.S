// SPDX-License-Identifier: GPL-3.0-only
// Controller descriptor setup and platform-backed per-channel allocation.
// Original ADSP file [0x3edd0,0x3ee58), 136 executable Hexagon bytes.
.text
.globl adsp_late_controller_descriptor_setup
adsp_late_controller_descriptor_setup:
{ r17 = r2; p0 = cmp.eq(r4,#0); memd(r29+#-16) = r17:16; allocframe(#16) }
{ r2 = #0; r16 = r0; memd(r29+#0) = r19:18 }
{ memw(r0++#24) = r2; memw(r16+#16) = r17 }
{ immext(#0xf004ef00); r2 = mux(p0,##-268189968,#0); r18 = r3; memw(r16+#4) = r1 }
{ call setup_shared_descriptor; memw(r16+#8) = r2 }
{ immext(#2048); p0 = cmp.gtu(r17,##2048); r0 = #1; if (p0.new) jump:t allocation_ready }
{ call initialize_descriptor_capacity; immext(#4096); r1:0 = combine(r17,##4096) }
allocation_ready:
{ p0 = cmp.gt(r18,#-1); memw(r16+#40) = r0; memw(r16+#44) = #0 }
{ r19 = mux(p0,r18,r0) }
{ p0 = cmp.gt(r19,#0); if (!p0.new) jump:nt completed }
{ r0 = mpyi(r19,r17); call allocate_queue_bytes }
{ r18 = r0; if (cmp.eq(r18.new,#0)) jump:nt completed }
next_queue:
{ call publish_descriptor_slot; r1:0 = combine(r18,r16); r18 = add(r18,r17) }
{ r19 = add(r19,#-1); if (!cmp.eq(r19.new,#0)) jump:t next_queue }
completed:
{ r17:16 = memd(r29+#8); r19:18 = memd(r29+#0) }
{ dealloc_return }
.set setup_shared_descriptor, adsp_late_controller_descriptor_setup+0xe9cdc
.set initialize_descriptor_capacity, adsp_late_controller_descriptor_setup+0x125110
.set allocate_queue_bytes, adsp_late_controller_descriptor_setup-0x4264
.set publish_descriptor_slot, adsp_late_controller_descriptor_setup+0x8e8
