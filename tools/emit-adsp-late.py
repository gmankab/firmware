#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Build source-authored ADSP later executable fragments, never OEM payloads.

An exact byte comparison of a generated fragment against a private, pinned
reference is a verification input, never a source of output bytes.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import importlib.util
from pathlib import Path
import struct
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[1]
TOOLS = ROOT / 'tools'
REFERENCE_SHA256 = '4b579e5dc1333d634e5bdc93f3314e357e0e30dab7140833f6cc7aef086b1c1e'
ELF_MACHINE_HEXAGON = 164
SEGMENTS = (
    (0x39000, 0xF0036000, 0x8BE36000, 0x1B97E8, 0x1BA000, 5),
    (0x364000, 0xB0000000, 0x8C500000, 0x94B790, 0x94C000, 5),
)
# File offset and exact executable-code length (not inferred BSS/alignment)
FRAGMENTS = {
    'queue-drain': (TOOLS / 'adsp-late-callback-drain.S', 0x39000, 0x54),
    'controller-dispatch': (TOOLS / 'adsp-late-controller-dispatch.S', 0x39060, 0x78),
    'zero-memory': (TOOLS / 'adsp-late-zero-memory.S', 0x3640A8, 0x50),
    'rx-startup': (TOOLS / 'adsp-late-rx-startup.S', 0x364000, 0xA8),
    'save-control': (TOOLS / 'adsp-late-save-control.S', 0x390D8, 0x24),
    'clear-ready': (TOOLS / 'adsp-late-clear-ready.S', 0x39100, 0x14),
    'callback-registry': (TOOLS / 'adsp-late-callback-registry.S', 0x39120, 0x80),
    'unregister-registry': (TOOLS / 'adsp-late-unregister-registry.S', 0x391A0, 0x40),
    'initialize-registry': (TOOLS / 'adsp-late-initialize-registry.S', 0x391E0, 0x50),
    'registry-allocator': (TOOLS / 'adsp-late-registry-allocator.S', 0x39230, 0x98),
    'registry-probe': (TOOLS / 'adsp-late-registry-probe.S', 0x392C8, 0x50),
    'copy-registry-slot': (TOOLS / 'adsp-late-copy-registry-slot.S', 0x39490, 0x10),
    'atomic-add': (TOOLS / 'adsp-late-atomic-add.S', 0x394A0, 0x14),
    'buffer-handoff': (TOOLS / 'adsp-late-buffer-handoff.S', 0x394B8, 0x88),
    'buffer-publish': (TOOLS / 'adsp-late-buffer-publish.S', 0x39540, 0x28),
    'conditional-query': (TOOLS / 'adsp-late-conditional-query.S', 0x39568, 0x20),
    'byte-prefix-compare': (TOOLS / 'adsp-late-byte-prefix-compare.S', 0x39590, 0x30),
    'bounded-copy': (TOOLS / 'adsp-late-bounded-copy.S', 0x395C0, 0x3C),
    # Several disjoint small helpers after the large unreconstructed codec
    # buffer manager; no bytes from that manager are included in the output.
    'controller-handoff-tail': (TOOLS / 'adsp-late-controller-handoff-tail.S', 0x3A7B8, 0x28),
    'global-pointer': (TOOLS / 'adsp-late-global-pointer.S', 0x3A7E0, 0x08),
    'format-buffer-context': (TOOLS / 'adsp-late-format-buffer-context.S', 0x3A7F0, 0x7C),
    'status-nibbles': (TOOLS / 'adsp-late-hex-prefix.S', 0x3A86C, 0x60),
    'create-context': (TOOLS / 'adsp-late-create-context.S', 0x3A090, 0xFC),
    'release-session': (TOOLS / 'adsp-late-release-session.S', 0x3A18C, 0x88),
    'configure-session': (TOOLS / 'adsp-late-configure-session.S', 0x3AA80, 0x80),
    'activate-context': (TOOLS / 'adsp-late-activate-context.S', 0x3AB00, 0x28),
    'return-mode-two': (TOOLS / 'adsp-late-return-mode-two.S', 0x3AB28, 0x04),
    'guarded-counter': (TOOLS / 'adsp-late-guarded-counter.S', 0x3AB2C, 0x40),
    'probe-callbacks': (TOOLS / 'adsp-late-probe-callbacks.S', 0x3AB6C, 0x48),
    'controller-shutdown': (TOOLS / 'adsp-late-controller-shutdown.S', 0x3AFA0, 0x44),
    'guarded-multiply': (TOOLS / 'adsp-late-guarded-multiply.S', 0x3BBE4, 0x14),
    'platform-calculate': (TOOLS / 'adsp-late-platform-calculate.S', 0x3BBF8, 0x10),
    'parameter-forward': (TOOLS / 'adsp-late-parameter-forward.S', 0x3BC08, 0x14),
    'parameter-combine': (TOOLS / 'adsp-late-parameter-combine.S', 0x3BE70, 0x10),
    'halfword-order': (TOOLS / 'adsp-late-halfword-order.S', 0x3BE80, 0x1C),
    'format-constraint': (TOOLS / 'adsp-late-format-constraint.S', 0x3BDD4, 0x9C),
    'buffer-bitfield': (TOOLS / 'adsp-late-buffer-bitfield.S', 0x3BA68, 0x58),
    'descriptor-tag-compare': (TOOLS / 'adsp-late-descriptor-tag-compare.S', 0x3BFB8, 0x14),
    'atomic-event-bit': (TOOLS / 'adsp-late-atomic-event-bit.S', 0x3BFCC, 0x44),
    'packet-type-check': (TOOLS / 'adsp-late-packet-type-check.S', 0x3D048, 0x14),
    'stack-marshaller': (TOOLS / 'adsp-late-stack-marshaller.S', 0x3D05C, 0x3C),
    'controller-shutdown-adapter': (TOOLS / 'adsp-late-controller-shutdown-adapter.S', 0x3D098, 0x30),
    'controller-param-28': (TOOLS / 'adsp-late-controller-param-28.S', 0x3ee58, 0x1C),
    'controller-param-52': (TOOLS / 'adsp-late-controller-param-52.S', 0x3ee74, 0x1C),
    'controller-param-56': (TOOLS / 'adsp-late-controller-param-56.S', 0x3ee90, 0x1C),
    'controller-param-48': (TOOLS / 'adsp-late-controller-param-48.S', 0x3eeac, 0x1C),
    'controller-param-112': (TOOLS / 'adsp-late-controller-param-112.S', 0x3eec8, 0x1C),
    'callback-table-walk': (TOOLS / 'adsp-late-callback-table-walk.S', 0x3EC1C, 0x28),
    'controller-state-flush': (TOOLS / 'adsp-late-controller-state-flush.S', 0x3EC44, 0x20),
    'controller-state-tail': (TOOLS / 'adsp-late-controller-state-tail.S', 0x3EC64, 0x10),
    'call-descriptor-dispatch': (TOOLS / 'adsp-late-call-descriptor-dispatch.S', 0x3E0A4, 0x28),
    'controller-runtime-init': (TOOLS / 'adsp-late-controller-runtime-init.S', 0x3E0D0, 0x80),
    'controller-descriptor-refresh': (TOOLS / 'adsp-late-controller-descriptor-refresh.S', 0x3E480, 0x50),
    'checked-argument-forward': (TOOLS / 'adsp-late-checked-argument-forward.S', 0x3EFA8, 0x4C),
    'controller-descriptor-setup': (TOOLS / 'adsp-late-controller-descriptor-setup.S', 0x3EDD0, 0x88),
    'controller-field-initialize': (TOOLS / 'adsp-late-controller-field-initialize.S', 0x3ED1C, 0xAC),
    'controller-release-check': (TOOLS / 'adsp-late-controller-release-check.S', 0x3E4D0, 0xD8),
    'controller-reset-status': (TOOLS / 'adsp-late-controller-reset-status.S', 0x3E750, 0x48),
    'controller-allocate': (TOOLS / 'adsp-late-controller-allocate.S', 0x3E5A8, 0xB0),
    'controller-validate-input': (TOOLS / 'adsp-late-controller-validate-input.S', 0x3E658, 0xF8),
    'controller-indexed-init': (TOOLS / 'adsp-late-controller-indexed-init.S', 0x3E9E0, 0x6C),
    'controller-platform-init': (TOOLS / 'adsp-late-controller-platform-init.S', 0x3E910, 0xD0),
    'controller-mode-default': (TOOLS / 'adsp-late-controller-mode-default.S', 0x3E7A0, 0x0C),
    'controller-opcode-switch': (TOOLS / 'adsp-late-controller-opcode-switch.S', 0x3E7AC, 0x148),
    'controller-report-default': (TOOLS / 'adsp-late-controller-report-default.S', 0x3E8F4, 0x1C),
    'controller-service-root': (TOOLS / 'adsp-late-controller-service-root.S', 0x3EA50, 0x1B4),
    'controller-field-00-get': (TOOLS / 'adsp-late-controller-field-00-get.S', 0x3EC80, 0xC),
    'controller-field-00-forward': (TOOLS / 'adsp-late-controller-field-00-forward.S', 0x3EC8C, 0x10),
    'controller-field-01-get': (TOOLS / 'adsp-late-controller-field-01-get.S', 0x3ECA0, 0xC),
    'controller-field-01-forward': (TOOLS / 'adsp-late-controller-field-01-forward.S', 0x3ECAC, 0x10),
    'controller-field-02-get': (TOOLS / 'adsp-late-controller-field-02-get.S', 0x3ECC0, 0xC),
    'controller-field-02-forward': (TOOLS / 'adsp-late-controller-field-02-forward.S', 0x3ECCC, 0x10),
    'controller-field-03-get': (TOOLS / 'adsp-late-controller-field-03-get.S', 0x3ECE0, 0xC),
    'controller-field-04-get': (TOOLS / 'adsp-late-controller-field-04-get.S', 0x3ECF0, 0xC),
    'controller-field-03-forward': (TOOLS / 'adsp-late-controller-field-03-forward.S', 0x3ECFC, 0x10),
    'controller-field-04-forward': (TOOLS / 'adsp-late-controller-field-04-forward.S', 0x3ED0C, 0x10),
}

class LateAdspError(ValueError):
    """Format, architecture, revision or byte-identity failure."""


def checked_range(size: int, offset: int, length: int) -> None:
    if not (0 <= offset <= size and 0 <= length <= size - offset):
        raise LateAdspError(f'Invalid bounded interval at 0x{offset:x} len 0x{length:x}')


def validate_reference(data: bytes) -> None:
    if len(data) < 52 or data[:6] != b'\x7fELF\x01\x01':
        raise LateAdspError('Expected little-endian ELF32 reference')
    (
        kind, machine, version, entry, phoff, shoff, flags, ehsize,
        phentsize, phnum, shentsize, shnum, shstrndx
    ) = struct.unpack_from('<HHIIIIIHHHHHH', data, 16)
    if kind != 2 or machine != ELF_MACHINE_HEXAGON or version != 1 or flags != 0x66:
        raise LateAdspError('Wrong ADSP image ELF32/Hexagon-v66 identification')
    if entry != 0x8BE00000 or phentsize != 32 or phnum < 11 or ehsize < 52:
        raise LateAdspError('Wrong ADSP ELF32 entry/program-header layout')
    checked_range(len(data), phoff, phentsize * phnum)
    mappings = set()
    for i in range(phnum):
        kind, off, va, pa, fsz, msz, perms, align = struct.unpack_from(
            '<IIIIIIII', data, phoff + i * phentsize)
        if kind != 1:
            continue
        if fsz:
            checked_range(len(data), off, fsz)
        if msz < fsz:
            raise LateAdspError('ELF segment filesz exceeds memsz')
        if perms & 1:
            mappings.add((off, va, pa, fsz, msz, perms & 7))
    if not set(SEGMENTS).issubset(mappings):
        raise LateAdspError('ADSP later executable PT_LOAD mappings differ')
    digest = hashlib.sha256(data).hexdigest()
    if digest != REFERENCE_SHA256:
        raise LateAdspError(f'Wrong original ADSP revision: {digest}')


def extract_text(elf: bytes, length: int) -> bytes:
    # Agent04 preexisting parser requires exactly one executable .text,
    # rejects unresolved relocations and checks proper bounded ELF ranges.
    modfile = TOOLS / 'emit-adsp-entry.py'
    spec = importlib.util.spec_from_file_location('adsp_entry_parser', modfile)
    if spec is None or spec.loader is None:
        raise LateAdspError('ADSP ELF object parser unavailable')
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    try:
        return module.read_text_section(elf, expected_length=length)
    except Exception as e:
        raise LateAdspError(str(e)) from e


def compile_source(llvm_mc: str, source: Path, size: int) -> bytes:
    if not source.is_file():
        raise LateAdspError(f'Assembler source missing: {source}')
    asm = source.read_text()
    for forbidden in ('.word ', '.long ', '.byte ', '.incbin ', '.fill '):
        if forbidden in asm:
            raise LateAdspError(f'Private binary or opcode-word directive forbidden: {forbidden}')
    with tempfile.TemporaryDirectory(prefix='adsp-late-') as d:
        obj = Path(d) / 'program.o'
        result = subprocess.run(
            [llvm_mc, '-triple=hexagon', '-mcpu=hexagonv65', '-filetype=obj',
             str(source), '-o', str(obj)],
            text=True, capture_output=True, check=False)
        if result.returncode:
            raise LateAdspError(f'Hexagon LLVM6 assembler failed: {result.stderr.strip()}')
        return extract_text(obj.read_bytes(), size)


def verify_slice(code: bytes, data: bytes, offset: int) -> None:
    validate_reference(data)
    if not any(lo <= offset and offset + len(code) <= lo + size
               for lo, _va, _pa, size, _mem, _perms in SEGMENTS):
        raise LateAdspError(f'Generated fragment 0x{offset:x} outside ADSP executable segment')
    checked_range(len(data), offset, len(code))
    original = data[offset:offset + len(code)]
    if original != code:
        mismatch = next(i for i, (left, right) in enumerate(zip(code, original))
                        if left != right)
        raise LateAdspError(f'Mismatch at original file offset 0x{offset+mismatch:x}: '
                            f'generated={code[mismatch]:02x}, expected={original[mismatch]:02x}')


def assembled_inventory(llvm_mc: str, reference: bytes | None = None) -> tuple[dict, dict[str,bytes]]:
    """Return exact source-generated executable regions without filling holes.

    Never concatenate disjoint regions or reuse OEM code in output. The
    manifest accounts for executable instruction bytes only; interstitial
    alignment/BSS and authentication are not reconstructed artifacts.
    """
    if reference is not None:
        validate_reference(reference)
    spans=[]
    emitted={}
    for name,(source,off,size) in sorted(FRAGMENTS.items(), key=lambda row:row[1][1]):
        assembled=compile_source(llvm_mc,source,size)
        if reference is not None:
            verify_slice(assembled,reference,off)
        matched=[(lo,va,pa,filesz) for lo,va,pa,filesz,_mem,_perm in SEGMENTS
                 if lo <= off and off+size <= lo+filesz]
        if len(matched)!=1:
            raise LateAdspError(f'Fragment {name} has no unique executable segment')
        lo,va,pa,_sz=matched[0]
        spans.append({
            'name':name,'source':source.name,
            'file_offset_start':off,'file_offset_end':off+size,
            'virtual_address_start':va+(off-lo),'physical_address_start':pa+(off-lo),
            'source_generated_executable_bytes':size,
            'sha256':hashlib.sha256(assembled).hexdigest(),
            'reference_byte_identical':reference is not None,
        })
        emitted[name]=assembled
    for left,right in zip(spans,spans[1:]):
        if left['file_offset_end']>right['file_offset_start']:
            raise LateAdspError('Overlap among independently reconstructed source intervals')
    return ({
        'format_version':1,
        'firmware':'Xiaomi Nabu ADSP Hexagon v66',
        'reference_sha256':REFERENCE_SHA256,
        'compiler':'LLVM6 Hexagon v65 assembler, architecturally shared instructions only',
        'fragments':spans,
        'total_independently_assembled_executable_bytes':sum(x['source_generated_executable_bytes'] for x in spans),
        'gap_bytes_included_in_generated_executables':0,
        'external_original_code_bytes_in_output':0,
        'complete_adsp_image':False,
        'original_oem_authenticity_generated':False,
    },emitted)


def main(argv: list[str] | None = None) -> int:
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument('--segment', choices=['all', *sorted(FRAGMENTS)], default='queue-drain')
    p.add_argument('--llvm-mc', required=True)
    p.add_argument('--output', type=Path, required=True)
    p.add_argument('--reference', type=Path, help='private exact adsp.mbn (optional)')
    args = p.parse_args(argv)
    if args.segment == 'all':
        original = args.reference.read_bytes() if args.reference is not None else None
        inventory, outputs = assembled_inventory(args.llvm_mc,original)
        args.output.mkdir(parents=True,exist_ok=True)
        for name,code in outputs.items():
            (args.output/f'{name}.bin').write_bytes(code)
        (args.output/'manifest.json').write_text(json.dumps(inventory,indent=2)+'\n')
        print(f"Rebuilt {inventory['total_independently_assembled_executable_bytes']} "
              f"original executable bytes in {len(outputs)} disjoint fragments, "
              f"validated against reference={original is not None}; "
              f"directory={args.output}")
        return 0
    source, off, size = FRAGMENTS[args.segment]
    result = compile_source(args.llvm_mc, source, size)
    if args.reference is not None:
        verify_slice(result, args.reference.read_bytes(), off)
    args.output.write_bytes(result)
    print(f'{args.segment}: independently assembled {len(result)} executable bytes '
          f'at original file [0x{off:x},0x{off+size:x}); '
          f'sha256={hashlib.sha256(result).hexdigest()}; reference={args.reference is not None}')
    return 0

if __name__ == '__main__':
    try:
        raise SystemExit(main())
    except LateAdspError as e:
        print(f'ERROR: {e}', file=sys.stderr)
        raise SystemExit(1) from e
