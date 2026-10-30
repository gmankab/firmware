"""No-unknown-byte and non-overlapping ownership tests for Agent 04 assembler."""
import importlib.util
from pathlib import Path
import unittest

p=Path(__file__).resolve().parents[1]/'tools/agent20_04_full_range.py'
spec=importlib.util.spec_from_file_location('agent20_04_full_range',p)
assembly=importlib.util.module_from_spec(spec)
spec.loader.exec_module(assembly)

class FullAgent04Layout(unittest.TestCase):
    @staticmethod
    def good_parts():
        initial=b'A'*(assembly.MIDDLE_START-assembly.START)
        mid=[(assembly.MIDDLE_START,b'B'*0x2d4),
             (assembly.MIDDLE_START+0x2d4,b'C'*(assembly.MIDDLE_END-assembly.MIDDLE_START-0x2d4))]
        tail=b'D'*(assembly.TAIL_END-assembly.MIDDLE_END)
        return initial,mid,tail
    def test_exact_163840_bytes_no_fill(self):
        p,m,t=self.good_parts()
        merged=assembly.assemble(p,m,t)
        self.assertEqual(len(merged),0x28000)
        self.assertEqual(merged[:len(p)],p)
        self.assertEqual(merged[-len(t):],t)
    def test_reject_gap_and_overlap(self):
        p,m,t=self.good_parts()
        with self.assertRaisesRegex(ValueError,'overlap or hole'):
            assembly.assemble(p,[(m[0][0]+4,m[0][1])]+m[1:],t)
        with self.assertRaisesRegex(ValueError,'overlap or hole'):
            assembly.assemble(p,[m[0],(m[1][0]-4,m[1][1])],t)
    def test_reject_missing_tail_and_prefix(self):
        p,m,t=self.good_parts()
        with self.assertRaisesRegex(ValueError,'prefix'):
            assembly.assemble(p[:-4],m,t)
        with self.assertRaisesRegex(ValueError,'continuation length'):
            assembly.assemble(p,m,t[:-4])
    def test_reject_missing_middle_and_excess(self):
        p,m,t=self.good_parts()
        with self.assertRaisesRegex(ValueError,'no middle'):
            assembly.assemble(p,[],t)
        with self.assertRaisesRegex(ValueError,'exceeded'):
            assembly.assemble(p,[(assembly.MIDDLE_START,b'B'*(assembly.MIDDLE_END-assembly.MIDDLE_START+4))],t)

if __name__=='__main__':unittest.main()
