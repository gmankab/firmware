# SPDX-License-Identifier: GPL-3.0-only
"""Test verification entrypoint path behavior without compiling OEM images."""
from pathlib import Path
import os
import shutil
import subprocess
import tempfile
import unittest

ROOT=Path(__file__).resolve().parents[1]


def executable(path: Path, body: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text('#!/bin/sh\nset -eu\n'+body)
    path.chmod(0o755)


class VerificationEntrypointsTests(unittest.TestCase):
    def test_verify_delegates_to_low_memory_gate(self):
        with tempfile.TemporaryDirectory() as d:
            root=Path(d)
            (root/'tools').mkdir()
            shutil.copy2(ROOT/'tools/verify.sh',root/'tools/verify.sh')
            executable(root/'tools/check-local.sh',f'printf "%s\\n" "$@" > "{root}/args"\n')
            subprocess.run(['bash',str(root/'tools/verify.sh')],cwd='/',check=True)
            self.assertEqual((root/'args').read_text(),'--full\n')

    def test_target_dir_resolves_relative_absolute_and_space_paths(self):
        with tempfile.TemporaryDirectory() as d:
            root=Path(d)
            (root/'tools').mkdir()
            shutil.copy2(ROOT/'tools/verify-xiaomi-nabu.sh',root/'tools/verify-xiaomi-nabu.sh')
            executable(root/'tools/verify.sh',':\n')
            (root/'reference/xiaomi-nabu').mkdir(parents=True)
            (root/'reference/xiaomi-nabu/firmware.sha256').write_text('')
            tree=root/'fw tree'
            (tree/'qcom/sm8150/Xiaomi/nabu').mkdir(parents=True)
            fake=root/'fake tools'
            for command in ('python3','sha256sum'):
                executable(fake/command,':\n')
            env=os.environ.copy()
            env['PATH']=f'{fake}:{env["PATH"]}'
            for value in ('relative target',str(root/'absolute target')):
                with self.subTest(value=value):
                    dest=root/value
                    log=root/'called'
                    executable(dest/'release/firmware-tools',
                        f'printf "%s\\n" "$1" >> "{log}"\n')
                    env['CARGO_TARGET_DIR']=value
                    subprocess.run(['bash',str(root/'tools/verify-xiaomi-nabu.sh'),str(tree)],
                                   cwd='/',env=env,check=True,stdout=subprocess.DEVNULL)
                    calls=log.read_text().splitlines()
                    self.assertIn('verify-modemuw',calls)
                    self.assertIn('verify-venus-contiguous-boot',calls)
                    self.assertEqual(calls.count('verify-mbnv6-hashes'),6)
                    log.unlink()


if __name__=='__main__': unittest.main()
