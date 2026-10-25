# SPDX-License-Identifier: GPL-3.0-only
"""Agent11 early modem packets: standalone emission and fail-closed checks."""
from pathlib import Path
import importlib.util
import unittest
import os

SCRIPT=Path(__file__).resolve().parents[1]/"tools/assemble-agent11-modem.py"
spec=importlib.util.spec_from_file_location("agent11_modem",str(SCRIPT))
module=importlib.util.module_from_spec(spec)
assert spec.loader is not None
spec.loader.exec_module(module)

class ModemAgent11Tests(unittest.TestCase):
    def test_mapped_entry_boundaries(self):
        self.assertEqual(module.FILE_START,0x4058)
        self.assertEqual(module.PHYSICAL_START,0x8e001058)
        self.assertEqual(module.ELF_VADDR,0xc0801058)
        self.assertEqual(module.PHYSICAL_START-module.FILE_START,0x8dffd000)

    def test_missing_or_wrong_reference_rejected(self):
        with self.assertRaisesRegex(ValueError,"incorrect revision"):
            module.verify_reference(bytes(0x5000),bytes(64))
        self.assertEqual(len(module.EXPECTED_SHA256),64)

    def test_packet_order_normalizer_rejects_unknown_encoding(self):
        size=0x42c8-module.FILE_START
        with self.assertRaisesRegex(ValueError,"unexpected Hexagon"):
            module.restore_original_legal_hexagon_packet_order(bytes(size))

    def test_assembler_with_llvm6_when_available(self):
        llvm=os.environ.get("AGENT11_LLVM_MC")
        if not llvm: self.skipTest("set AGENT11_LLVM_MC for external tool verification")
        source=SCRIPT.parent.parent/"crates/firmware-tools/src/devices/xiaomi_nabu/modem_entry_followup.s"
        data=module.assemble(Path(llvm),source)
        self.assertEqual(len(data),3588)
        # Structural assertion checks the two adjacent word loads are in
        # one duplex slot and the final instruction is a packet-ending return.
        words=[int.from_bytes(data[n:n+4],"little") for n in range(0,len(data),4)]
        self.assertEqual(words[0] & 0xc000,0x4000) # extended immediate packet
        self.assertEqual(words[16] & 0xc000,0xc000) # S27 = r0, end packet
        self.assertEqual(words[30] & 0xc000,0xc000) # :endloop0 at 0x40d0
        self.assertEqual(words[31]&0xc000,0xc000) # function return at 0x40d4
        self.assertEqual((words[16]>>16)&0x3f,27) # S27 read in first table-copy
        self.assertEqual((words[32]>>16)&0x3f,27) # S27 read in descriptor update
        self.assertEqual(words[36]&0xc000,0x4000) # predicate in 4-word packet
        self.assertEqual(words[39]&0xc000,0xc000) # predicated update closes packet
        self.assertEqual(words[46]&0xc000,0xc000) # protected S-register setup
        self.assertEqual(words[81]&0xc000,0xc000) # fault brkpt at 0x419c
        self.assertEqual(words[(0x4210-module.FILE_START)//4-1]&0xc000,0xc000) # status shim jumpr r13
        self.assertEqual(words[95]&0xc000,0xc000) # controller register barrier
        i=(0x42bc-module.FILE_START)//4
        self.assertEqual(words[i]&0xc000,0x4000) # extender of compare immediate
        self.assertEqual(words[i+1]&0xc000,0x4000) # comparison before .new branch
        self.assertEqual(words[i+2]&0xc000,0xc000) # branch ends packet
        for a in (0x4318,0x4340,0x4368):
            self.assertEqual(words[(a-module.FILE_START)//4]&0xc000,0x4000,
                "r7 copy and r5 halfword initialization share one packet")
        self.assertEqual(words[(0x44e8-module.FILE_START)//4]&0xc000,0xc000) # diagnostic brkpt
        self.assertEqual(words[(0x4528-module.FILE_START)//4]&0xc000,0xc000)
        self.assertEqual(words[(0x45fc-module.FILE_START)//4]&0xc000,0x4000)
        self.assertEqual(words[(0x4600-module.FILE_START)//4]&0xc000,0xc000)
        self.assertEqual(words[(0x48a4-module.FILE_START)//4-1]&0xc000,0xc000) # physical copy return
        i=(0x46f4-module.FILE_START)//4
        self.assertEqual([words[i+j]&0xc000 for j in range(4)],
                         [0x4000,0x4000,0x4000,0xc000])
        self.assertEqual(words[(0x444c-module.FILE_START)//4]&0xc000,0x4000)
        self.assertEqual(words[(0x4454-module.FILE_START)//4]&0xc000,0xc000)
        # The next pagewalk has one packet joining loop0 and its R2/R3 setup.
        self.assertEqual(words[(0x4744-module.FILE_START)//4]&0xc000,0x4000)
        self.assertEqual(words[(0x474c-module.FILE_START)//4]&0xc000,0xc000)
        # Independently specified Hexagon V60 TLBR / TLBLOCK / TLBUNLOCK.
        self.assertEqual(words[(0x4750-module.FILE_START)//4],0x6c43c004)
        self.assertEqual(words[(0x4778-module.FILE_START)//4],0x6c20c020)
        self.assertEqual(words[(0x477c-module.FILE_START)//4],0x6c06c300)
        self.assertEqual(words[(0x4780-module.FILE_START)//4],0x6c20c040)
        self.assertEqual([words[(0x47c4-module.FILE_START)//4+j]&0xc000 for j in range(2)],
                         [0x4000,0xc000])
        # The protected cache transfer and physical-mask helper retain
        # packet-local predicate, backedge branch and pointer load order.
        i=(0x4814-module.FILE_START)//4
        self.assertEqual([words[i+j]&0xc000 for j in range(4)],
                         [0x4000,0xc000,0x4000,0xc000])
        i=(0x4870-module.FILE_START)//4
        self.assertEqual([words[i+j]&0xc000 for j in range(4)],
                         [0x4000,0x4000,0x4000,0xc000])
        self.assertEqual(words[(0x4894-module.FILE_START)//4],0x9200e405)
        # Protected scan's cache-pointer packet extends the controller
        # register shift and 17 intentionally installed exception traps.
        self.assertEqual(words[(0x4938-module.FILE_START)//4]&0xc000,0x4000)
        for a in range(0x49bc,0x4a00,4):
            self.assertEqual(words[(a-module.FILE_START)//4],0x6c20c000)
        self.assertEqual([words[(0x4a30-module.FILE_START)//4+i]&0xc000 for i in range(3)],
                         [0x4000,0x4000,0xc000])
        # Invalid controller mode transfers to the existing earlier GP base
        # helper; the actual Hexagon J2_call is assembled from a named label.
        self.assertEqual(words[(0x4acc-module.FILE_START)//4]&0xc000,0xc000)
        # The exception state transition requires the original R24/P3:0 and
        # control-register operands, followed by exactly 14 NOP packets.
        assert len(data) == 0x4e5c-module.FILE_START
        for offset,sys,reg in [(0x4ae8,52,24),(0x4af8,47,29)]:
            code=words[(offset-module.FILE_START)//4]
            self.assertEqual(code & ~(0x7f<<16 | 0x1f),0x6e80c000)
            self.assertEqual((code>>16)&0x7f,sys)
            self.assertEqual(code&31,reg)
        self.assertEqual(words[(0x4b08-module.FILE_START)//4],
                         0x64600020 | 23<<16 | 3<<14)  # Y4_start(r23)
        self.assertEqual(words[(0x4b0c-module.FILE_START)//4]&0xc000,0x4000)
        for packet in range(0x4b20,0x4c00,16):
            idx=(packet-module.FILE_START)//4
            self.assertEqual([x&0xc000 for x in words[idx:idx+4]],
                             [0x4000,0x4000,0x4000,0xc000])
            self.assertEqual([x&~0xc000 for x in words[idx:idx+4]],
                             [0x7f000000]*4)
        for trap in range(0x4c04,0x4c1c,4):
            w=words[(trap-module.FILE_START)//4]
            self.assertEqual(w,0x5800c000)  # intentionally self-looping jumps
        # Saving the hardware context is a distinct explicit code span, not
        # original copied firmware bytes or filler. A32-sized Hexagon words.
        def w(addr):return words[(addr-module.FILE_START)//4]
        self.assertEqual(w(0x4c1c),0x6e88c017)  # thread identifier S8 -> r23
        self.assertEqual(w(0x4c78),0x6eb6c016)  # S54 -> r22
        self.assertEqual(w(0x4ccc),0x68004002)  # C1:0 -> r3:2 (CONTINUE)
        self.assertEqual(w(0x4cd4),0x68024002)  # C3:2 -> r3:2 (CONTINUE)
        self.assertEqual(w(0x4ce8),0x680cc002)  # CS1:0 -> r3:2 (END)
        self.assertEqual(w(0x4cf4),0x6e864003)  # SSR -> r3
        self.assertEqual(w(0x4d08),0x6e82c003)  # STID -> r3
        self.assertEqual(w(0x4d0c)&0xc000,0)  # paired duplex
        # Preserve both new-predicate/branch packet orderings exactly.
        for packet in [0x4d88,0x4db8]:
            self.assertEqual([w(packet+i)&0xc000 for i in (0,4)],
                             [0x4000,0xc000])
        self.assertEqual(w(0x4db8)&0xff000000,0xbf000000) # R4 decrement
        self.assertEqual(w(0x4d70)&0xc000,0x4000) # S54-relative table extender
        # A numerical LLVM target would encode the wrong PC-relative branch.
        # The monitor address is reconstructed from an explicit packet base.
        self.assertEqual(w(0x4e50)&0xc000,0x4000) # call extender continues


if __name__=='__main__':unittest.main()
