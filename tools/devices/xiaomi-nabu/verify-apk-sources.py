#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Verify that local inputs match the *packaged* Nabu firmware inventory.

One upstream source checkout has a different Novatek firmware revision.
This script explicitly selects the postmarketOS source for that file.
No proprietary firmware bytes are copied to project outputs.
"""
import argparse
import hashlib
import json
import tarfile
from pathlib import Path

SPECIAL_SOURCES = {'novatek_nt36523_fw.bin': 'postmarketos-nabu-firmware'}
DEFAULT_SOURCE = 'map220v-nabu-firmware'


def check_sources(checksum_file: Path, source_root: Path) -> list[dict]:
    results = []
    for line in checksum_file.read_text().splitlines():
        if not line.strip() or line.lstrip().startswith('#'):
            continue
        sha, rel = line.strip().split(maxsplit=1)
        filename = Path(rel.removeprefix('*').removeprefix('./')).name
        repo = SPECIAL_SOURCES.get(filename, DEFAULT_SOURCE)
        candidate = source_root / repo / filename
        if not candidate.is_file():
            raise FileNotFoundError(f'{filename}: expected {candidate}')
        with candidate.open('rb') as stream:
            actual = hashlib.file_digest(stream, 'sha256').hexdigest()
        results.append({'file': filename, 'source': repo,
                        'expected_sha256': sha, 'sha256': actual,
                        'size': candidate.stat().st_size,
                        'matches_package': sha == actual})
    if len(results) != 9:
        raise ValueError(f'expected 9 firmware files, found {len(results)}')
    return results


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('source_root', type=Path,
                        help='directory containing map220v-nabu-firmware and postmarketos-nabu-firmware')
    parser.add_argument('--hashes', type=Path,
                        default=Path(__file__).resolve().parents[3]/'reference/xiaomi-nabu/firmware.sha256')
    parser.add_argument('--output', type=Path, help='optional report JSON path')
    parser.add_argument('--apk', type=Path, help='optional original Alpine APK (gzip tar), verified in memory')
    args = parser.parse_args()
    results = check_sources(args.hashes, args.source_root)
    if args.apk is not None:
        with tarfile.open(args.apk, 'r:gz') as archive:
            contents = {}
            for entry in archive:
                name = Path(entry.name).name
                if entry.isfile() and name in {r['file'] for r in results}:
                    fileobj = archive.extractfile(entry)
                    if fileobj is None:
                        raise ValueError(f'cannot read {entry.name}')
                    with fileobj:
                        contents[name] = (hashlib.file_digest(fileobj, 'sha256').hexdigest(), entry.size)
        for r in results:
            if r['file'] not in contents:
                raise ValueError(f"{r['file']} absent from APK")
            apk_sha, apk_size = contents[r['file']]
            r['matches_apk'] = apk_sha == r['sha256'] and apk_size == r['size']
    for r in results:
        print(('OK' if r['matches_package'] else 'FAIL'), r['file'], r['size'], r['source'])
    if args.output:
        args.output.write_text(json.dumps(results, indent=2) + '\n')
    if not all(r['matches_package'] and r.get('matches_apk', True) for r in results):
        raise SystemExit(1)


if __name__ == '__main__':
    main()
