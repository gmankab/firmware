# SPDX-License-Identifier: GPL-3.0-only
"""Optional, independent ILM source checks for Agent19 DLM reset defaults."""
from pathlib import Path
import importlib.util
import os
import unittest

ROOT=Path(__file__).resolve().parents[1]
SPEC=importlib.util.spec_from_file_location("upper_consumer",ROOT/"tools/novatek/verify-upper-ilm-evidence.py")
evidence=importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(evidence)


class UpperIlmConsumerEvidence(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.ilm=os.environ.get("AGENT18_UPPER_ILM_RS")
        cls.ex9=os.environ.get("AGENT18_UPPER_EX9_RS")
        if not (cls.ilm and cls.ex9):
            raise unittest.SkipTest("Agent 18 source paths not supplied")
        cls.ilm_text=Path(cls.ilm).read_text()
        cls.ex9_text=Path(cls.ex9).read_text()

    def test_proven_four_sram_reset_fields_and_unclaimed_counter(self):
        proof=evidence.verify_ilm(self.ilm_text,self.ex9_text)
        self.assertEqual(proof["independently_source_reconstructed_initial_bytes"],13)
        self.assertEqual([x["file_offset"] for x in proof["source_checks"]],
                         ["0x2e17c","0x2e180","0x2e188","0x2e18c"])
        self.assertEqual(proof["uncredited_counter"],"0x2e184..0x2e188")

    def test_modified_reset_value_or_offset_is_rejected(self):
        modified=self.ilm_text.replace("(0x155e2, Movi55(R0, -1))", "(0x155e2, Movi55(R0, 0))")
        with self.assertRaisesRegex(ValueError,"0x155e2"):
            evidence.verify_ilm(modified,self.ex9_text)
        modified=self.ilm_text.replace("(0x15738, SwiGp(R1, -6232))", "(0x15738, SwiGp(R1, -6236))")
        with self.assertRaisesRegex(ValueError,"0x15738"):
            evidence.verify_ilm(modified,self.ex9_text)

    def test_unproven_ex9_clobber_is_rejected(self):
        modified=self.ex9_text.replace("SbiGp(R0, -1691), // slot 436,", "Movi55(R1, 0), // slot 436,")
        with self.assertRaisesRegex(ValueError,"slot 436"):
            evidence.verify_ilm(self.ilm_text,modified)


if __name__=="__main__":
    unittest.main()
