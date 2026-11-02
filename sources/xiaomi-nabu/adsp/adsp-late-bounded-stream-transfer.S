// Hexagon V66 ADSP bounded stream copy/transfer at original ELF file
// [0x3f000,0x3f074): process up to 4096 bytes at a time, call the platform
// queue and copy routines, and return the transfer status. Source-authored
// instruction packets: no raw OEM executable opcode directives.
.text
.globl adsp_late_bounded_stream_transfer
adsp_late_bounded_stream_transfer:
{ r17:16 = combine(r2,r3); memd(r29+#-16) = r17:16; allocframe(#24) }
{ p0 = cmp.eq(r16,#0); r19:18 = combine(r0,r1); memd(r29+#8) = r19:18; memd(r29+#0) = r21:20 }
{ if (p0) r21 = #-1; if (p0) jump:nt finish_at_source_width; nop }
next_chunk:
{ r2 = extractu(r17,#12,#0); r1:0 = combine(r19,#0) }
{ immext(#4096); r2 = sub(##4096,r2) }
{ r20 = minu(r2,r16); r3:2 = combine(r18,#82) }
{ call platform_queue; r5:4 = combine(r20,r17) }
{ r21 = r0; if (!cmp.gt(r21.new,#-1)) jump:nt finish_at_source_width }
{ call platform_transfer; r21 = #0; r2 = r20; r1:0 = combine(#0,r18) }
{ p0 = cmp.eq(r0,#0); if (!p0.new) jump:nt finish_at_source_width }
{ r16 = sub(r16,r20); r18 = add(r10,r20); r17 = add(r9,r20) }
{ if (p0.new) r21 = #-1; p0 = cmp.eq(r16,#0); if (!p0.new) jump:t next_chunk_at_source_width }
finish:
{ r0 = r21; r17:16 = memd(r29+#16); r19:18 = memd(r29+#8) }
{ r21:20 = memd(r29+#0); dealloc_return }
.set platform_queue, adsp_late_bounded_stream_transfer+0xebe2c
.set platform_transfer, adsp_late_bounded_stream_transfer+0x1005a0

// The LLVM6 V65 toolchain emits the paired V66 register additions as two
// full words. The typed V66 encoder combines them into one duplex word.
// Keep branch displacements expressed in final-length addresses.
.set finish_at_source_width, finish-4
.set next_chunk_at_source_width, next_chunk+4
