# SPDX-License-Identifier: GPL-3.0-only
"""Tests for the typed Hexagon modem revision and dispatch build steps."""
from pathlib import Path
import importlib.util
import unittest

ROOT=Path(__file__).resolve().parents[1]
SPEC=importlib.util.spec_from_file_location('modem_dispatch', ROOT/'tools/assemble-modem-dispatch.py')
modem=importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(modem)

class DispatchChecks(unittest.TestCase):
    def test_contiguous_size(self):
        self.assertEqual(modem.FILE_START,0x3644)
        self.assertEqual(modem.FILE_END,0x4058)
        self.assertEqual(modem.LENGTH,2580)
        self.assertEqual(modem.PREFIX_LENGTH,1604)
        self.assertEqual(modem.LENGTH+modem.PREFIX_LENGTH,4184)

    def test_rejects_wrong_revision(self):
        with self.assertRaisesRegex(ValueError,'wrong modem reference'):
            modem.verify(b'wrong',b'abc',0x3644)

    def test_all_packet_sites_are_inside_reconstructed_source(self):
        self.assertEqual(modem.PREDICATE_BRANCH_PACKETS,(0x3814,))
        self.assertEqual(modem.IMMEDIATE_BEFORE_BRANCH_PACKETS,(0x39a0,))
        self.assertEqual(modem.TLB_LOOP_PACKETS,(0x3aa8,))
        self.assertEqual(modem.TLB_FILL_PACKETS,(0x3e84,))
        self.assertEqual(modem.LOGIC_AND_EXTENDED_GP_PACKETS,(0x3f64,0x3f70))
        self.assertEqual(modem.PREDICATE_LOOKUP_PACKETS,
                         (0x3710, 0x3758, 0x37a0, 0x37e8))
        self.assertTrue(all(modem.FILE_START <= pc < modem.FILE_END for pc in modem.PREDICATE_LOOKUP_PACKETS))
        with self.assertRaisesRegex(ValueError,'unexpected code size'):
            modem.restore_predicate_order(b'\x00' * 16)

    def test_source_has_symbolic_branches_and_packet_groupings(self):
        text=(ROOT/'crates/firmware-tools/src/devices/xiaomi_nabu/modem_boot_dispatch.s').read_text()
        self.assertIn('HEX_SYSTEM_READ 2,29,1',text)
        self.assertIn('r4 = memw(r25+##-1064821884)',text)
        self.assertIn('jump:nt modem_ready_handler',text)
        self.assertIn('if (p0.new) memw(r23+#0) = r0',text)
        self.assertIn('HEX_PHYSICAL_WORD_LOAD 2,0,1', text)
        self.assertIn('r2 = asl(r2,#16)', text)
        self.assertIn('memd(r0++#8) = r5:4', text)
        self.assertIn('}:endloop0', text)
        self.assertIn('tlb_stage_loop:', text)
        self.assertIn('tlbw', text)
        self.assertIn('transfer_record_next', text)
        self.assertIn('HEX_ABS_MEMW_STORE 26,0xc08722b0', text)
        self.assertIn('HEX_ABS_MEMW_LOAD 1,0xc191c134', text)
        self.assertIn('second_page_map_iter:', text)
        self.assertIn('HEX_CACHE_OP 0x20,0', text)
        self.assertIn('HEX_CONTROL_WRITE 11,1', text)
        self.assertIn('HEX_EXTENDED_CALL 0x74f20,0x3bfc', text)
        self.assertIn('adjust_ptrs_one:', text)
        self.assertIn('adjust_ptrs_two:', text)
        self.assertIn('HEX_COPY_MAP_PAIR 256,260', text)
        self.assertIn('skip_s59_after_snapshot:',text)
        self.assertIn('commit_page_cache:',text)
        self.assertIn('relocated_fault_done:',text)
        self.assertIn('relocated_start_ready:',text)
        self.assertIn('initialized_tlb_table_loop:',text)
        self.assertIn('HEX_TLB_WRITE 2,0',text)
        self.assertIn('HEX_ICKILL',text)
        self.assertIn('HEX_ABS_MEMUB_LOAD 1,0xc0b36524',text)
        self.assertIn('dccleaninva(r1)',text)
        self.assertIn('loop0(copy_buffer_words_first,#37)',text)
        self.assertIn('loop0(copy_buffer_words_second,#6)',text)
        self.assertIn('memw(r6++#4) = r7.new',text)
        self.assertIn('jumpr r31',text)
        self.assertIn('bootstrap_relative_complete:',text)
        self.assertIn('HEX_COPY_MAP_PAIR 12288,16384', text)
        self.assertIn('physical_snapshot_retry:', text)
        self.assertIn('memd(r4++#8) = r1:0', text)
        self.assertNotIn('.incbin',text)

if __name__=='__main__':unittest.main()
