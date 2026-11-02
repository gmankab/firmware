# SPDX-License-Identifier: GPL-3.0-only
"""Bounded file inspection must not load giant sources wholesale."""
import importlib.util
from pathlib import Path
import tempfile
import unittest

P = Path(__file__).resolve().parents[1] / 'tools/analysis/bounded-inspect.py'
spec = importlib.util.spec_from_file_location('bounded_inspect', P)
assert spec and spec.loader
m = importlib.util.module_from_spec(spec)
spec.loader.exec_module(m)


class BoundedSourceInspectionTests(unittest.TestCase):
    def test_small_line_range_and_large_source(self):
        with tempfile.TemporaryDirectory() as dirname:
            file = Path(dirname) / 'big.rs'
            with file.open('wb') as out:
                for _ in range(200_000):
                    out.write(b'let x = 0;\n')
            text = m.inspect_lines(file, 42, 3)
            self.assertEqual(text.count('let x = 0;'), 3)
            self.assertIn('      42', text)
            with self.assertRaises(ValueError):
                m.inspect_lines(file, 42, 201)
            with self.assertRaises(ValueError):
                m.inspect_lines(file, 0, 1)

    def test_large_read_and_out_of_bounds_are_rejected(self):
        with tempfile.TemporaryDirectory() as dirname:
            file = Path(dirname) / 'small.bin'
            file.write_bytes(b'0123456789')
            self.assertEqual(m.inspect_bytes(file, 2, 4), b'2345')
            for args in [(-1, 1), (0, 65537), (8, 5)]:
                with self.subTest(args=args), self.assertRaises(ValueError):
                    m.inspect_bytes(file, *args)


if __name__ == '__main__':
    unittest.main()
