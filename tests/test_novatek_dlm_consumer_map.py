# SPDX-License-Identifier: GPL-3.0-only
"""Memory-access provenance tests; no OEM firmware or ILM source required."""
import copy
import importlib.util
from pathlib import Path
import unittest

SOURCE = Path(__file__).resolve().parents[1] / 'tools/novatek' / 'audit-novatek-dlm-consumers.py'
spec = importlib.util.spec_from_file_location('agent19_consumer_map', SOURCE)
mod = importlib.util.module_from_spec(spec)
# Dataclass class lookup relies on a registered import module.
import sys
sys.modules[spec.name] = mod
spec.loader.exec_module(mod)


def ledger():
    rows = [
        {'file_start': 0, 'file_end': mod.DLM_START, 'kind': 'external_metadata'},
        {'file_start': mod.DLM_START, 'file_end': 0x2e17c, 'kind': 'unknown_dlm'},
        {'file_start': 0x2e17c, 'file_end': 0x2e180, 'kind': 'source_dlm_data'},
        {'file_start': 0x2e180, 'file_end': mod.DLM_END, 'kind': 'unknown_dlm'},
        {'file_start': mod.DLM_END, 'file_end': 196608, 'kind': 'source_layout'},
    ]
    return {'schema': 'agent20-19-novatek-dlm-v1',
            'profile': 'postmarketos-nabu',
            'reference_sha256': mod.PINNED_SHA256,
            'total_bytes': 196608, 'new_executable_bytes': 0,
            'source_dlm_data_bytes': 4,
            'unknown_dlm_bytes': mod.DLM_END-mod.DLM_START-4,
            'regions': rows}


class ConsumerEvidenceTests(unittest.TestCase):
    def setUp(self):
        self.ilmtxt = '''
            (0x10000, LbiGp(R1, -55774)),
            (0x10002, Ex9It(7)),
            (0x10004, SwiGp(R0, -6244)),
            (0x10008, Movi55(R0, 4)),
            (0x1000a, SbiGp(R0, -54678)),
            (0x1000e, Movi(R1, 0x20300)),
        '''
        self.ex9txt = '''
            SbiGp(R0, -6328), // slot 7, example memory operation
            Movi(R1, 3), // slot 9, another example
        '''

    def test_maps_exact_widths_and_expands_ex9_source_once(self):
        entries = mod.collect_accesses(self.ilmtxt, self.ex9txt, ledger())
        self.assertEqual(len(entries), 4)
        self.assertEqual([hex(x.dlm_file_address) for x in entries],
                         ['0x22002', '0x2e128', '0x2e17c', '0x2244a'])
        self.assertEqual([x.width_bytes for x in entries], [1, 1, 4, 1])
        self.assertEqual([x.direction for x in entries], ['read','write','write','write'])
        self.assertEqual(entries[2].credited_region_kind, 'source_dlm_data')
        self.assertTrue(all(x.credited_region_kind=='unknown_dlm' for x in
                            (entries[0],entries[1],entries[3])))
        self.assertEqual([x.ex9_slot for x in entries], [None, 7, None, None])
        j = mod.summarize(entries, ledger())
        self.assertEqual(j['new_source_dlm_bytes'], 0)
        self.assertEqual(j['new_executable_bytes'], 0)
        self.assertEqual(j['unresolved_read_offsets'], [0x22002])
        self.assertEqual(j['source_dlm_data_bytes'], 4)

    def test_only_addresses_that_intersect_dlm_are_returned(self):
        in_range = mod.collect_accesses(self.ilmtxt, self.ex9txt)
        extra = '''(0x10010, SwiGp(R0, 0)),
                   (0x10014, SbiGp(R0, -82880)),'''
        out = mod.collect_accesses(self.ilmtxt + extra, self.ex9txt)
        self.assertEqual(in_range, out)
        with self.assertRaisesRegex(ValueError, 'straddling DLM'):
            mod.collect_accesses('(0x10000, SwiGp(R0, -6226)),','Movi(R0, 0), // slot 0, note',ledger())

    def test_fail_closed_on_missing_duplicate_and_bad_gp_instruction(self):
        with self.assertRaisesRegex(ValueError, 'missing EX9 operation 7'):
            mod.collect_accesses(self.ilmtxt, 'Movi(R0, 3), // slot 2, note')
        with self.assertRaisesRegex(ValueError, 'duplicate EX9 slot'):
            mod.parse_ex9_table(self.ex9txt+'\nSbiGp(R2, -6328), // slot 7, duplicate')
        with self.assertRaisesRegex(ValueError, 'duplicate ILM instruction'):
            mod.parse_upper_ilm(self.ilmtxt+'\n(0x1000e, Movi(R2, 1)),')
        with self.assertRaisesRegex(ValueError, 'unknown GP-width'):
            mod.collect_accesses('(0x10000, UnknownGp(R0, -6244)),', self.ex9txt)
        with self.assertRaisesRegex(ValueError, 'invalid GP operands'):
            mod.collect_accesses('(0x10000, LwiGp(R1)),', self.ex9txt)

    def test_pinned_snapshot_requires_complete_pinned_map_and_unchanged_ledger(self):
        fake = mod.summarize(mod.collect_accesses(self.ilmtxt, self.ex9txt, ledger()), ledger())
        with self.assertRaisesRegex(ValueError,'provenance was altered'):
            mod.verify_agent18_checkpoint02_snapshot(fake)
        fake['source_dlm_data_bytes'] = 9353
        fake['unresolved_dlm_bytes'] = 40199
        with self.assertRaisesRegex(ValueError,'snapshot changed'):
            mod.verify_agent18_checkpoint02_snapshot(fake)

    def test_bad_profile_or_ledger_tamper_fails_before_coverage_claims(self):
        for mutate,errmsg in [
            (lambda x: x.update(profile='map220v-nabu'), 'profile'),
            (lambda x: x.update(reference_sha256='bad'), 'profile'),
            (lambda x: x.update(new_executable_bytes=1), 'executable provenance'),
            (lambda x: x.update(source_dlm_data_bytes=5), 'accounting'),
            (lambda x: x['regions'][2].update(file_start=0x2e17b), 'nonpartitioning'),
            (lambda x: x['regions'][3].update(kind='source_layout'), 'unrecognized DLM'),
        ]:
            bad=copy.deepcopy(ledger());mutate(bad)
            with self.subTest(reason=errmsg),self.assertRaises(ValueError):
                mod.validate_ledger(bad)

    def test_unmodified_source_coverage_is_not_new_authorship(self):
        a=mod.collect_accesses(self.ilmtxt,self.ex9txt,ledger())
        first=mod.summarize(a,ledger())
        self.assertEqual(first['new_source_dlm_bytes'],0)
        self.assertEqual(first['provenance_class_sites'],{'unknown_dlm':3,'source_dlm_data':1})
        self.assertEqual(first['unique_dlm_byte_offsets'],4)


if __name__ == '__main__':
    unittest.main()
