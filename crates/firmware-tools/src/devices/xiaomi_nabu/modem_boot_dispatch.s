// SPDX-License-Identifier: GPL-3.0-only
// Packetized modem early dispatch and low-level cache handling.
.text
.macro HEX_SYSTEM_READ general, system, marker=3
    .if (\general < 0) || (\general > 31) || (\system < 0) || (\system > 127)
        .error "invalid system register read"
    .endif
    .word (0x6e800000 | (\system << 16) | (\marker << 14) | \general)
.endm
.macro HEX_SYSTEM_WRITE system, general, marker=3
    .if (\general < 0) || (\general > 31) || (\system < 0) || (\system > 127)
        .error "invalid Hexagon system register write"
    .endif
    .word (0x67000000 | (\general << 16) | (\marker << 14) | \system)
.endm
.macro HEX_PHYSICAL_WORD_LOAD dst, address, offset_reg, marker=3
    // Y2_memw_phys: 10010010000sssssPP1ttttt---ddddd.
    .if (\dst < 0) || (\dst > 31) || (\address < 0) || (\address > 31) || (\offset_reg < 0) || (\offset_reg > 31)
        .error "Hexagon physical-register operands out of range"
    .endif
    .word (0x92002000 | (\address << 16) | (\marker << 14) | (\offset_reg << 8) | \dst)
.endm
// Hexagon GP-relative absolute access with a constant extender. The V5
// `memw(gp+#u16:2)` encoding treats GP as zero when an extender is present.
// LLVM6's assembler shortens this to a different PC/GP-relative addressing
// form. Emit the documented ISA bitfields with checked operands instead;
// no OEM opcode words are embedded here.
.macro HEX_ABS_MEMW_STORE src, absolute
    .if (\src < 0) || (\src > 31)
        .error "invalid Hexagon absolute store source register"
    .endif
    .if ((\absolute & 3) != 0) || (\absolute < 0) || (\absolute > 0xffffffff)
        .error "invalid Hexagon absolute word-store address"
    .endif
    // Constant extender: u32[31:20] at word[27:16], u32[19:6] at [13:0].
    .word ((((\absolute >> 20) & 0xfff) << 16) | (1 << 14) | ((\absolute >> 6) & 0x3fff))
    // `memw(gp + #u16:2) = Rt`, with six low address bits and R[t].
    .word (0x48800000 | (3 << 14) | (\src << 8) | (\absolute & 63))
.endm
.macro HEX_ABS_MEMW_LOAD dst, absolute
    .if (\dst < 0) || (\dst > 31)
        .error "invalid Hexagon absolute load destination register"
    .endif
    .if ((\absolute & 3) != 0) || (\absolute < 0) || (\absolute > 0xffffffff)
        .error "invalid Hexagon absolute word-load address"
    .endif
    .word ((((\absolute >> 20) & 0xfff) << 16) | (1 << 14) | ((\absolute >> 6) & 0x3fff))
    // `Rd = memw(gp + #u16:2)`, low address bits stored from bit 5.
    .word (0x49800000 | (3 << 14) | ((\absolute & 63) << 5) | \dst)
.endm
// Checked privileged Hexagon cache-maintenance/control instruction fields.
.macro HEX_CACHE_OP operation, src, marker=3
    .if (\src < 0) || (\src > 31)
        .error "invalid Hexagon cache operation source register"
    .endif
    .if (\operation != 0x20) && (\operation != 0x60)
        .error "invalid Hexagon CSWI/CIAD cache operation"
    .endif
    .if (\marker != 1) && (\marker != 3)
        .error "invalid Hexagon packet parse marker"
    .endif
    // ISA: 01100100000sssssPP------0oo----- with CSWI oo=01, CIAD oo=11.
    .word (0x64000000 | (\src << 16) | (\marker << 14) | \operation)
.endm
.macro HEX_CONTROL_WRITE control, src, marker=3
    .if (\control < 0) || (\control > 31) || (\src < 0) || (\src > 31)
        .error "invalid Hexagon control register assignment"
    .endif
    // ISA: 01100010001sssssPP---------ddddd.
    .word (0x62200000 | (\src << 16) | (\marker << 14) | \control)
.endm
// Extended PC-relative call. The original call's relative offset is based on
// the PC of the extender word, not the call word. LLVM6 collapses a far call
// into a single instruction when it fits 22 bits; retain the explicit form.
.macro HEX_EXTENDED_CALL target_file, extender_file
    .set hex_rel\@, (\target_file - \extender_file)
    .if ((hex_rel\@ & 3) != 0) || (hex_rel\@ >= 0x80000000) || (hex_rel\@ < -0x80000000)
        .error "invalid extended Hexagon PC-relative call target"
    .endif
    .word ((((hex_rel\@ >> 20) & 0xfff) << 16) | (1 << 14) | ((hex_rel\@ >> 6) & 0x3fff))
    // J2_call: 0101101iiiiiiiiiPPiiiiiiiiiiiii0, low six bits unscaled.
    .word (0x5a000000 | (3 << 14) | ((hex_rel\@ & 63) << 1))
.endm
// Copy one Hexagon MMU/controller pair into the destination metadata table.
// Consecutive extended memory operations share a single four-word packet.
.macro HEX_COPY_MAP_PAIR dst_a, dst_b, source_a, source_b
    {
        r2 = add(r1,#\dst_a)
        r4 = add(r1,#\dst_b)
    }
    {
        r3 = memw(r25+##\source_a)
        r5 = memw(r25+##\source_b)
    }
    { memw(r2+#0) = r3; memw(r4+#0) = r5 }
.endm
.globl modem_boot_dispatch
modem_boot_dispatch:
.set modem_ready_handler, modem_boot_dispatch+0x5c4
{ r0 = memw(r25+##-1064935416) }
{
 p0 = cmp.eq(r0,#2)
 p1 = cmp.eq(r0,#0)
}
{ if (p0) jump:nt modem_ready_handler }
// Y2_l2kill, an external L2-cache-kill ISA instruction not accepted by LLVM6 MC.
// 1010100000100000PP00000000000000: opcode class plus packet parse bits.
.word (0xa8200000 | (3 << 14))
{ syncht }
{ if (!p1) jump:nt modem_ready_handler }
// Reset the current bootstrap handshake state only if the controller has not
// already installed a valid address. Keep condition-producing packets intact.
{
 r23 = add(r25,##-1064885440)
 r0 = memw(r25+##-1064935408)
}
.set current_handshake_unset, modem_boot_dispatch+0x44
.set handshake_done, modem_boot_dispatch+0x48
{
 p0 = cmp.eq(r0,#-1)
 if (p0.new) jump:nt current_handshake_unset
 r1 = memw(r23+#0)
}
{
 p0 = cmp.eq(r1,#0)
 if (p0.new) memw(r23+#0) = r0
}
{ jump handshake_done }
current_handshake_unset:
{ memw(r23+#0) = r27 }
// Platform-version dispatch: read S29 then choose one of two controller
// resource tables according to the extracted revision code.
HEX_SYSTEM_READ 2,29,1
{ r3 = ##135429 }
{
 r2 = extract(r2,#24,#0)
 r12 = r2
 r4 = memw(r25+##-1064821884)
}
.set use_alternate_resource, modem_boot_dispatch+0x80
.set descriptor_ready, modem_boot_dispatch+0x9c
{
 p0 = cmp.eq(r2,r3); if (!p0.new) jump:nt use_alternate_resource
 r3 = ##139525
}
{ p0 = cmp.eq(r4,#0); if (p0.new) jump:nt descriptor_ready }
{
 r1 = r4
 jump descriptor_ready
 r0 = memw(r25+##-1064821856)
}
use_alternate_resource:
{
 p0 = cmp.eq(r2,r3); if (!p0.new) jump:nt descriptor_ready
 r4 = memw(r25+##-1064821880)
}
{ p0 = cmp.eq(r4,#0); if (p0.new) jump:nt descriptor_ready }
{ r1 = r4; r0 = memw(r25+##-1064821852) }
// First staged modem image-map lookup: return the table entry whose key
// matches S29's extracted revision field, retaining the previous default.
{ r4 = memw(r25+##-1064821872) }
.set first_stage_done, modem_boot_dispatch+0xe4
{
 r3 = memw(r25+##-1061984940)
 if (cmp.eq(r3.new,#0)) jump:t first_stage_done
}
{
 r7 = add(r25,##-1045593560)
 r8 = add(r25,##-1045593496)
}
first_stage_iter:
{
 r3 = add(r3,#-1)
 r5 = memw(r7++#4)
 r6 = memw(r8++#4)
}
.set first_stage_match, modem_boot_dispatch+0xdc
{
 p0 = cmp.eq(r6,r12)
 if (p0.new) jump:nt first_stage_match
 if (p0.new) r4 = add(r5,#0)
}
{ p1 = cmp.eq(r3,#0); if (!p1.new) jump:t first_stage_iter }
first_stage_match:
{ memw(r25+##-1064821872) = r4 }
// Second staged resource table: the modem's secondary memory-map variant.
{ r4 = memw(r25+##-1064821860) }
.set second_stage_done, modem_boot_dispatch+0x12c
{
 r3 = memw(r25+##-1061985036)
 if (cmp.eq(r3.new,#0)) jump:t second_stage_done
}
{
 r7 = add(r25,##-1033824968)
 r8 = add(r25,##-1033824904)
}
second_stage_iter:
{
 r3 = add(r3,#-1)
 r5 = memw(r7++#4)
 r6 = memw(r8++#4)
}
.set second_stage_match, modem_boot_dispatch+0x124
{
 p0 = cmp.eq(r6,r12)
 if (p0.new) jump:nt second_stage_match
 if (p0.new) r4 = add(r5,#0)
}
{ p1 = cmp.eq(r3,#0); if (!p1.new) jump:t second_stage_iter }
second_stage_match:
{ memw(r25+##-1064821860) = r4 }
// Third image-map table, with a different resource stride but identical
// bounded search semantics. Registers r7/r8 are post-incremented in packet.
{ r4 = memw(r25+##-1064821876) }
.set third_stage_done, modem_boot_dispatch+0x174
{
 r3 = memw(r25+##-1061985032)
 if (cmp.eq(r3.new,#0)) jump:t third_stage_done
}
{
 r7 = add(r25,##-1033824840)
 r8 = add(r25,##-1033824776)
}
third_stage_iter:
{
 r3 = add(r3,#-1)
 r5 = memw(r7++#4)
 r6 = memw(r8++#4)
}
.set third_stage_match, modem_boot_dispatch+0x16c
{
 p0 = cmp.eq(r6,r12)
 if (p0.new) jump:nt third_stage_match
 if (p0.new) r4 = add(r5,#0)
}
{ p1 = cmp.eq(r3,#0); if (!p1.new) jump:t third_stage_iter }
third_stage_match:
{ memw(r25+##-1064821876) = r4 }
// Fourth image-map table, covering the final optional region lookup.
{ r4 = memw(r25+##-1064821888) }
.set fourth_stage_done, modem_boot_dispatch+0x1bc
{
 r3 = memw(r25+##-1061984944)
 if (cmp.eq(r3.new,#0)) jump:t fourth_stage_done
}
{
 r7 = add(r25,##-1045593688)
 r8 = add(r25,##-1045593624)
}
fourth_stage_iter:
{
 r3 = add(r3,#-1)
 r5 = memw(r7++#4)
 r6 = memw(r8++#4)
}
.set fourth_stage_match, modem_boot_dispatch+0x1b4
{
 p0 = cmp.eq(r6,r12)
 if (p0.new) jump:nt fourth_stage_match
 if (p0.new) r4 = add(r5,#0)
}
{ p1 = cmp.eq(r3,#0); if (!p1.new) jump:t fourth_stage_iter }
fourth_stage_match:
{ memw(r25+##-1064821888) = r4 }
// Controller status publication after the resource-map lookups: record the
// selected table format in one byte and install four MMU page metadata words.
{
 r6 = add(r25,##-1061984988)
 r4 = memub(r25+##-1061984988)
}
.set table_version_done, modem_boot_dispatch+0x1e4
.set table_version_store, modem_boot_dispatch+0x1e0
{
 r3 = extract(r2,#4,#12)
 p0 = cmp.eq(r4,#100)
 if (!p0.new) jump:nt table_version_done
}
{ p0 = cmp.gt(r3,#3); if (p0.new) jump:nt table_version_store }
{ r3 = add(r3,#1) }
table_version_store:
{ memb(r6+#0) = r3 }
table_version_done:
{ r0 = memw(r25+##-1064821872) }
HEX_SYSTEM_WRITE 60,0
{ r0 = memw(r25+##-1064821888) }
{ r0 = clrbit(r0,#21) }
HEX_SYSTEM_WRITE 63,0
{ r0 = memw(r25+##-1064821876) }
HEX_SYSTEM_WRITE 62,0
{ r0 = memw(r25+##-1064821860) }
HEX_SYSTEM_WRITE 61,0
.set skip_s59_write, modem_boot_dispatch+0x228
{
 r0 = memw(r25+##-1064821868)
 if (cmp.eq(r0.new,#0)) jump:t skip_s59_write
}
HEX_SYSTEM_WRITE 59,0
skip_s59_write:
{ r0 = memw(r25+##-1061984980) }
.set finish_cache_options, modem_boot_dispatch+0x254
{ p0 = cmp.eq(r0,#0); if (p0.new) jump:nt finish_cache_options }
{ r0 = or(r0,##-2147483648) }
{ r2 = memw(r25+##-1061985028) }
.set skip_large_cache, modem_boot_dispatch+0x250
{ p0 = cmp.eq(r2,#0); if (p0.new) jump:nt skip_large_cache }
{ r0 = or(r0,##1073741824) }
skip_large_cache:
HEX_SYSTEM_WRITE 35,0
finish_cache_options:
HEX_SYSTEM_READ 0,18
{ r0 = setbit(r0,#1) }
HEX_SYSTEM_WRITE 18,0
{ isync }
// Poll controller readiness and collect two physical architecture-control
// registers. Each result is shifted and stored into the early boot vectors.
.set modem_boot_is_ready, modem_boot_dispatch+0x274
.set modem_boot_spin, modem_boot_dispatch+0x270
{
 r0 = memw(r25+##-1065353148)
 if (cmp.eq(r0.new,#0)) jump:t modem_boot_is_ready
}
modem_boot_spin:
{ jump modem_boot_spin }
modem_boot_is_ready:
.set modem_boot_helper, modem_boot_dispatch+0x94c
{
 call modem_boot_helper
 r0 = #13
}
HEX_SYSTEM_READ 1,27
{
 r1 = asl(r1,#5)
 r0 = ##12
}
HEX_PHYSICAL_WORD_LOAD 2,0,1
{
 r2 = asl(r2,#16)
 memw(r25+##-1064935424) = r2.new
}
HEX_SYSTEM_READ 1,27
{
 r1 = asl(r1,#5)
 r0 = ##16
}
HEX_PHYSICAL_WORD_LOAD 2,0,1
{
 r2 = asl(r2,#16)
 memw(r25+##-1064935420) = r2.new
}
// Fault-status initialization. Respect S42 hardware gating and only
// enable the translated-address cache when controller probes have succeeded.
HEX_SYSTEM_READ 10,42
.set skip_fault_status_probe, modem_boot_dispatch+0x304
{
 p0 = tstbit(r10,#2)
 if (!p0.new) jump:t skip_fault_status_probe
}
HEX_SYSTEM_READ 1,27,1
{ r3 = #0 }
{ r1 = insert(r3,#12,#20) }
{
 r1 = asl(r1,#5)
 r0 = #8
}
HEX_PHYSICAL_WORD_LOAD 1,0,1
{ r1 = insert(r3,#12,#20) }
{
 r1 = asl(r1,#5)
 r0 = #24
}
{ r1 = add(r1,#-256) }
HEX_PHYSICAL_WORD_LOAD 1,0,1
{ r2 = memw(r25+##-1061985012) }
{ r0 = and(r1,r2) }
.set test_cache_probe_done, modem_boot_dispatch+0x314
{ p0 = cmp.eq(r0,#0); if (p0.new) jump:t test_cache_probe_done }
{ r1 = #1 }
HEX_SYSTEM_WRITE 42,1
{ isync }
{ brkpt }
test_cache_probe_done:
{
 r0 = #18
 memw(r25+##-1065353148) = r0.new
}
.set vector_remap_done, modem_boot_dispatch+0x33c
{
 r5 = memw(r25+##-1064935400)
 if (cmp.eq(r5.new,#0)) jump:t vector_remap_done
}
{ p0 = tstbit(r5,#0) }
{ if (p0) r5 = add(r5,#-1) }
{ if (p0) r5 = add(r5,r24) }
{ callr r5 }
vector_remap_done:
HEX_SYSTEM_READ 0,18
{ r0 = setbit(r0,#2) }
HEX_SYSTEM_WRITE 18,0
{ isync }
// Relocate the initial 64-bit word stream into staged memory. The copy loop
// processes one 8-byte pair per packet and branches only while the source
// pointer has not reached the end of the mapped segment.
{
 r0 = ##88080384
 r2 = ##-1065201664
}
.set memory_map_done, modem_boot_dispatch+0x384
{
 r1 = ##-1065287680
 p0 = cmp.eq(r0,#0); if (p0.new) jump:nt memory_map_done
}
{ p0 = cmp.eq(r1,r2); if (p0.new) jump:nt memory_map_done }
{
 r1 = add(r1,r25)
 r2 = add(r2,r25)
}
{ r5:4 = memd(r1++#8) }
initial_memory_copy:
{
 p0 = cmp.eq(r1,r2); if (!p0.new) jump:t initial_memory_copy
 r5:4 = memd(r1++#8)
 memd(r0++#8) = r5:4
}
// Firmware transfer-descriptor interpreter. Types 3/4 copy 64-bit or 32-bit
// chunks to the configured destination; type 6 invokes a range callback.
{ r2 = add(pc,##-2432) }
{ r0 = #0 }
transfer_record_next:
{ r2 = add(r2,r0) }
{ r0 = memuh(r2+#0) }
{ p0 = cmp.eq(r0,#0) }
.set transfer_records_complete, modem_boot_dispatch+0x41c
{ if (p0) jump:nt transfer_records_complete }
{ r1 = memuh(r2+#2) }
{ p0 = cmp.eq(r1,#3) }
.set transfer_decode, modem_boot_dispatch+0x3c0
{ if (p0) jump transfer_decode }
{ p0 = cmp.eq(r1,#4) }
{ if (p0) jump transfer_decode }
{ p0 = cmp.eq(r1,#6) }
.set transfer_type6_callback, modem_boot_dispatch+0xa94
{ if (p0) call transfer_type6_callback }
{ jump transfer_record_next }
transfer_decode:
{ r4 = memw(r2+#4) }
{ r5 = memw(r2+#8) }
{ r6 = memw(r2+#12) }
{ p0 = tstbit(r5,#0) }
{ if (p0) r5 = add(r5,#-1) }
{ if (p0) r5 = add(r5,r25) }
{ r4 = add(r4,r25) }
{ p0 = cmp.eq(r1,#3) }
.set transfer_double_words, modem_boot_dispatch+0x3ec
{ if (p0) jump transfer_double_words }
{ p0 = cmp.eq(r1,#4) }
.set transfer_single_words, modem_boot_dispatch+0x404
{ if (p0) jump transfer_single_words }
transfer_double_words:
{ r6 = lsr(r6,#3) }
{ loop0(transfer_double_iteration,r6) }
transfer_double_iteration:
{ r9:8 = memd(r4++#8) }
{
 nop
 memd(r5++#8) = r9:8
}:endloop0
{ jump transfer_record_next }
transfer_single_words:
{ r6 = lsr(r6,#2) }
{ loop0(transfer_single_iteration,r6) }
transfer_single_iteration:
{ r8 = memw(r4++#4) }
{
 nop
 memw(r5++#4) = r8
}:endloop0
{ jump transfer_record_next }
// Program initial modem memory protection translations. The loop reads
// doublewords of checked page-map entries and writes TLB slots with a
// monotonically increasing index before handing control to init callbacks.
.set install_modem_tlbs, modem_boot_dispatch+0xfa4
{ call install_modem_tlbs }
{ r5 = memw(r25+##-1064935396) }
{ p0 = cmp.eq(r5,#0) }
.set finished_modem_tlbs, modem_boot_dispatch+0x4dc
{ if (p0) jump finished_modem_tlbs }
{ p0 = tstbit(r5,#0) }
{ if (p0) r5 = add(r5,#-1) }
{ if (p0) r5 = add(r5,r24) }
{ r4 = memw(r25+##-1064935412) }
{ r3 = lsr(r24,#11) }
{ r2 = #0 }
tlb_stage_loop:
{
 p0 = !cmp.eq(r4,#0)
 r1:0 = memd(r5+#0)
}
.set tlb_stage_exit, modem_boot_dispatch+0x470
{
 p1 = cmp.eq(r1,#0); if (p1.new) jump:nt tlb_stage_exit
 if (p0) r0 = add(r0,r3)
}
{ memd(r5++#8) = r1:0 }
// Y2_tlbw: 01101100000sssssPP0ttttt00000000; source pair R1:R0,
// index r2, one instruction at the end of a packet.
.word (0x6c000000 | (0 << 16) | (3 << 14) | (2 << 8))
{
 if (p0) r4 = add(r4,#-1)
 r2 = add(r2,#1)
 jump tlb_stage_loop
}
tlb_stage_exit:
{ memw(r25+##-1047412276) = r2 }
{ memw(r25+##-1064860592) = r2 }
HEX_SYSTEM_READ 0,18
.set modem_tlb_init, modem_boot_dispatch-0x144
{
 call modem_tlb_init
 r0 = or(r0,#1)
}
.set post_tlb_callback, modem_boot_dispatch+0xeac
{ call post_tlb_callback }
// Publish the selected bootstrap memory window, rounded up to an aligned
// word count. Global-pointer-relative controls are system configuration,
// not executable copies of OEM words.
HEX_ABS_MEMW_STORE 26,0xc08722b0
HEX_ABS_MEMW_STORE 24,0xc08722ac
HEX_ABS_MEMW_LOAD 0,0xc0872340
HEX_ABS_MEMW_LOAD 1,0xc191c134
{ r1 = add(r1,#-1) }
{ r0 = add(r0,r1) }
{ r0 = and(r0,~r1) }
HEX_ABS_MEMW_STORE 0,0xc0866010
{ r0 = sub(r0,r26) }
{ r0 = add(r0,r24) }
HEX_ABS_MEMW_STORE 0,0xc087233c
{ r25 = #0 }
.set modem_boot_advance, modem_boot_dispatch+0x524
{ jump modem_boot_advance }
// This secondary page-map loader is reachable via a distinct bootstrap
// branch and installs long memory mappings before rejoining TLB handling.
{ r2 = #0 }
{
 r5 = add(r25,##-1046887928)
 r3 = memw(r25+##-1047412276)
}
{
 r3 = add(r3,r2)
 r4 = add(r25,##-1064894464)
}
.set second_page_map_iter, modem_boot_dispatch+0x4fc
second_page_map_iter:
{ r1:0 = memd(r5+#0) }
.set next_page_map_word, modem_boot_dispatch+0x518
{ p1 = cmp.eq(r1,#0); if (p1.new) jump:nt next_page_map_word }
.set page_map_write_done, modem_boot_dispatch+0x50c
{ p1 = cmp.gt(r3,r2); if (!p1.new) jump:t page_map_write_done }
{ memd(r4++#8) = r1:0 }
page_map_write_done:
// Hexagon Y2_tlbw: checked source pair r1:r0 and index r2.
.word (0x6c000000 | (0 << 16) | (3 << 14) | (2 << 8))
{
 jump second_page_map_iter
 r5 = add(r5,#8)
 r2 = add(r2,#1)
}
next_page_map_word:
{ memw(r25+##-1064860592) = r3 }
{ jump modem_boot_dispatch+0x480 }
// Enter the final early-boot hook chain and relocate live load addresses.
.set vendor_init_callback, modem_boot_dispatch+(0x3f14-0x3644)
.set secondary_image_handler, modem_boot_dispatch+(0x7b400-0x3644)
.set boot_resource_callback, modem_boot_dispatch+(0x44fc-0x3644)
.set debug_reporting_handler, modem_boot_dispatch+(0x1d3a70-0x3644)
{ call vendor_init_callback }
{ call secondary_image_handler }
{ call boot_resource_callback }
{ r0 = #191 }
{ call debug_reporting_handler }
HEX_ABS_MEMW_LOAD 1,0xc08722ac
{ r1 = lsr(r1,#12) }
{ r3 = ##-1061984952 }
.set adjust_ptrs_one, modem_boot_dispatch+(0x3b90-0x3644)
.set adjust_ptrs_one_done, modem_boot_dispatch+(0x3ba4-0x3644)
adjust_ptrs_one:
{ r0 = memw(r3++#4) }
{ p0 = cmp.eq(r0,#0) }
{ if (p0) jump:nt adjust_ptrs_one_done }
{ memw(r0+#0) += r1 }
{ jump adjust_ptrs_one }
adjust_ptrs_one_done:
{ r3 = ##-1061985008 }
.set adjust_ptrs_two, modem_boot_dispatch+(0x3bac-0x3644)
.set adjust_ptrs_two_done, modem_boot_dispatch+(0x3bc0-0x3644)
adjust_ptrs_two:
{ r1:0 = memd(r3++#8) }
{ p0 = cmp.eq(r0,#0) }
{ if (p0) jump:nt adjust_ptrs_two_done }
{ memw(r0+#0) += r1 }
{ jump adjust_ptrs_two }
adjust_ptrs_two_done:
.set modem_maintain_system, modem_boot_dispatch+(0x3fa8-0x3644)
{
 call modem_maintain_system
 r0 = #20
}
{ r0 = #-1 }
HEX_CACHE_OP 0x20,0
HEX_CACHE_OP 0x60,0
{
 r0 = ##0xc0840000
 r1 = ##0xc2710000
}
HEX_SYSTEM_WRITE 16,0
HEX_CONTROL_WRITE 11,1
HEX_SYSTEM_WRITE 0,29
{
 call modem_maintain_system
 r0 = #21
}
.set restart_dsp_handler, modem_boot_dispatch+(0x78980-0x3644)
{ call restart_dsp_handler }
.set cpu_forward_handler, modem_boot_dispatch+(0x74f20-0x3644)
HEX_EXTENDED_CALL 0x74f20,0x3bfc
.set modem_hold_forever, modem_boot_dispatch+(0x3c04-0x3644)
{ jump modem_hold_forever }
// Hardware memory map synchronization: capture physical MPU state and copy
// validated controller page entries into their assigned hardware slots.
HEX_SYSTEM_READ 1,27,1
{ r2 = #0 }
{ r1 = insert(r2,#12,#20) }
{
 r1 = asl(r1,#5)
 r0 = ##8
}
HEX_PHYSICAL_WORD_LOAD 2,0,1
{ r0 = asl(r2,#5) }
{
 r7 = add(r0,#66)
 r6 = #4
 r4 = #0
}
.set physical_snapshot_retry, modem_boot_dispatch+(0x3c30-0x3644)
physical_snapshot_retry:
HEX_PHYSICAL_WORD_LOAD 1,6,7
HEX_PHYSICAL_WORD_LOAD 0,4,7
HEX_PHYSICAL_WORD_LOAD 2,6,7
{ p0 = cmp.eq(r2,r1); if (!p0.new) jump:nt physical_snapshot_retry }
{ memd(r25+##-1064891952) = r1:0 }
{ r1 = memw(r25+##-1064935420) }
HEX_COPY_MAP_PAIR 256,260,-1064891904,-1064891900
HEX_COPY_MAP_PAIR 264,268,-1064891896,-1064891892
HEX_COPY_MAP_PAIR 272,276,-1064891888,-1064891884
HEX_COPY_MAP_PAIR 4096,8192,-1064891880,-1064891876
HEX_COPY_MAP_PAIR 12288,16384,-1064891872,-1064891868
