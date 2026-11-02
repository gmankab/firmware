"""Conservative, source-only cross-references from upper ILM/EX9 MOVI constants."""
import importlib.util
import json
import os
from pathlib import Path
import unittest

ROOT=Path(__file__).resolve().parents[1]
spec=importlib.util.spec_from_file_location('novatek_dlm_literals',ROOT/'tools/novatek/audit-novatek-dlm-literal-anchors.py')
audit=importlib.util.module_from_spec(spec)
spec.loader.exec_module(audit)


def ledger():
    return {'source_dlm_data_bytes':9353,'unknown_dlm_bytes':40199,'regions':[
        {'file_start':0x22000,'file_end':0x22488,'kind':'unknown_dlm'},
        {'file_start':0x22488,'file_end':0x22710,'kind':'source_dlm_data'},
        {'file_start':0x22710,'file_end':0x2e190,'kind':'unknown_dlm'}]}


def upper():
    return '''
 (0x11000, Movi(R1, 0x203c0)),
 (0x11004, Movi(R2, 0x20488)),
 (0x11008, Movi(R1, 0x2b5d8)),
 (0x1100c, Movi(R1, 0x38d40)),
 (0x11010, Jal(0x13000)),
 '''


def ex9():
    return '''
 Movi(R0, 0x20000), // slot 159, file 0x1803c: movi
 Movi(R1, 0x38d40), // slot 160, file 0x18040: movi
 '''


class TestNovatekDlmLiteralAnchors(unittest.TestCase):
    def test_literal_addresses_are_not_memory_accesses_or_new_source(self):
        obj=audit.audit(upper(),ex9(),ledger())
        self.assertEqual(obj['absolute_dlm_literals'],4)
        self.assertEqual(obj['unresolved_target_literals'],3)
        self.assertEqual(obj['source_owned_target_literals'],1)
        self.assertEqual(obj['new_source_data_bytes'],0)
        self.assertFalse(obj['independently_recovered_semantics'])
        self.assertEqual(sorted(x['dlm_file_target'] for x in obj['cross_references']),[0x22000,0x223c0,0x22488,0x2d5d8])

    def test_invalid_ex9_slot_pc_and_duplicate_are_rejected(self):
        with self.assertRaisesRegex(ValueError,'invalid source file address'):
            audit.parse_ex9_file('Movi(R0, 0x20000), // slot 159, file 0x18040: invalid')
        with self.assertRaisesRegex(ValueError,'duplicate EX9'):
            audit.parse_ex9_file(ex9()+ex9())
        with self.assertRaisesRegex(ValueError,'duplicate typed'):
            audit.parse_upper_file(upper()+upper())
        with self.assertRaisesRegex(ValueError,'no typed'):
            audit.parse_ex9_file('LwiGp(R0, -1), // slot 159, file 0x1803c')

    def test_wrong_ledger_classification_and_missing_target_rejected(self):
        l=ledger();l['regions']=[{'file_start':0x22000,'file_end':0x2e190,'kind':'source_layout'}]
        with self.assertRaisesRegex(ValueError,'invalid provenance'):
            audit.audit(upper(),ex9(),l)
        l=ledger();l['regions']=[{'file_start':0x22000,'file_end':0x22100,'kind':'unknown_dlm'}]
        with self.assertRaisesRegex(ValueError,'not mapped'):
            audit.audit(upper(),ex9(),l)
        l=ledger();l['unknown_dlm_bytes']-=2
        with self.assertRaisesRegex(ValueError,'unexpectedly changed'):
            audit.audit(upper(),ex9(),l)

    def test_pinned_snapshot_rejects_missing_external_inputs(self):
        with self.assertRaisesRegex(ValueError,'snapshot changed'):
            audit.audit(upper(),ex9(),ledger(),strict_snapshot=True)
        with self.assertRaises(ValueError):
            audit.audit('empty','empty',ledger())

    @unittest.skipUnless(os.environ.get('AGENT18_UPPER_ILM_RS') and os.environ.get('AGENT18_UPPER_EX9_RS') and os.environ.get('NOVATEK_DLM_LEDGER'),
                         'Agent18 read-only typed source and Agent19 ledger not supplied')
    def test_readonly_agent18_checkpoint02_source(self):
        result=audit.audit(Path(os.environ['AGENT18_UPPER_ILM_RS']).read_text(),Path(os.environ['AGENT18_UPPER_EX9_RS']).read_text(),json.loads(Path(os.environ['NOVATEK_DLM_LEDGER']).read_text()),strict_snapshot=True)
        self.assertEqual(result['absolute_dlm_literals'],34)
        self.assertEqual(result['upper_ilm_literals'],28)
        self.assertEqual(result['ex9_literals'],6)
        self.assertEqual(result['new_source_data_bytes'],0)

if __name__=='__main__':unittest.main()
