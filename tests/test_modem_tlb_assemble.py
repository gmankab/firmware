# SPDX-License-Identifier: GPL-3.0-only
"""Pure, no-reference tests of the synthetic Hexagon packet-order restorer."""
import importlib.util
from pathlib import Path
import struct
import unittest

FILE=Path(__file__).resolve().parents[1]/"tools/modem/assemble-modem-tlb.py"
spec=importlib.util.spec_from_file_location("modem_tlb_assembler",FILE)
mod=importlib.util.module_from_spec(spec)
spec.loader.exec_module(mod)


class ModemTlbPacketTests(unittest.TestCase):
    def fake(self,words):
        entries=[0]* (mod.PART_LEN//4)
        pos=(mod.JUMP_GROUP_VMA-mod.START)//4
        entries[pos:pos+4]=words
        return struct.pack('<'+'I'*len(entries),*entries)

    def test_marker_preservation_and_predicated_order(self):
        first=0x8f000000|mod.CONTINUE
        jump=0x5c200000|mod.CONTINUE
        add_not=0x74800000|mod.CONTINUE
        add_yes=0x74000000|mod.END_PACKET
        x=mod.restore_predicated_packet(self.fake([first,jump,add_not,add_yes]))
        ws=struct.unpack_from('<4I',x,mod.JUMP_GROUP_VMA-mod.START)
        self.assertEqual(list(ws),[first,add_not,add_yes & ~mod.MARKER_MASK | mod.CONTINUE,
                             jump & ~mod.MARKER_MASK | mod.END_PACKET])

    def test_wrong_length_rejected(self):
        with self.assertRaises(ValueError):mod.restore_predicated_packet(b'')

    def test_bad_packet_class_rejected(self):
        with self.assertRaises(ValueError):mod.restore_predicated_packet(self.fake([0,0,0,0]))

    def test_malformed_elf_rejected(self):
        with self.assertRaises(ValueError):mod.text_from_object(b'raw binary')


if __name__ == '__main__':unittest.main()
