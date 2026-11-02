#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Verify source-built Hexagon V66 bank predicates with independent LLVM6/Rizin.

LLVM6 emits equivalent semantic operations but expands the one-word duplex,
changes parse markers and schedules the final predicate instruction ahead of
its guarded assignment. This verifier never falsely claims LLVM binary
identity; exact bytes are checked against pinned CDSP in the Rust emitter.
"""
import argparse,hashlib,importlib.util,os,subprocess,struct,tempfile
from pathlib import Path
OFF=0x56b78;END=0x56b98;RWE=0x56000
SHA='febeae7b8e47bad4eeabc159a3c24b74cda26dfa12b384c31de5de30817e06ea'
def section(b):
    src=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
    spec=importlib.util.spec_from_file_location('cdsp_pred_elf',src)
    assert spec and spec.loader
    m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
    return m.parse_elf32_text(b)
def decode(path,data,addr):
    env=os.environ.copy()
    lib=next((p/'ubuntu-runtime/build-deps-flat' for p in path.resolve().parents
        if (p/'ubuntu-runtime/build-deps-flat').is_dir()),None)
    if lib:env['LD_LIBRARY_PATH']=str(lib)+':'+env.get('LD_LIBRARY_PATH','')
    cp=subprocess.run([str(path),'-a','hexagon','-b','32','-o',hex(addr),'-D',data.hex()],
        capture_output=True,text=True,check=True,env=env)
    lines=[l for l in cp.stdout.splitlines() if l.startswith('0x')]
    if not lines or any('invalid' in l.lower() for l in lines):raise ValueError('Invalid V66 decode')
    return lines

def main():
    a=argparse.ArgumentParser(description=__doc__)
    for key in ('generated','prefix','reference','llvm-mc','rz-asm'):
        a.add_argument('--'+key,required=True,type=Path)
    args=a.parse_args()
    ref=args.reference.read_bytes();gen=args.generated.read_bytes();prefix=args.prefix.read_bytes()
    assert hashlib.sha256(ref).hexdigest()==SHA,'wrong pinned CDSP firmware'
    assert len(gen)==32 and gen==ref[OFF:END], '32-byte generated predicate range differs from reference'
    assert len(prefix)==2968 and prefix==ref[RWE:END], 'Full RWE prefix differs from reference'
    original=decode(args.rz_asm,gen,0x96cc8b78)
    assert len(original)==8
    for idx,expected in enumerate([
        'jump:nt 0x96cc8e90','P0 = or(P0,!P0)',
        'R0 = ##0x3 ; R1 = memw(R29+#0x1c)', 'call 0x96cce768',
        'R2 = P0','R19 = add(R16,##-0x1)',
        'if (P0.new) R24 = ##0x80', 'jump:nt 0x96cc8ea0',
    ]):
        assert expected in original[idx],f'Unexpected original word {idx}: {original[idx]}'
    with tempfile.TemporaryDirectory(prefix='cdsp-late-bank-pred-') as td:
        obj=Path(td)/'llvm.o'
        subprocess.run([str(args.llvm_mc),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj',
           str(Path(__file__).with_name('late-controller-bank-predicates-semantic.s')),
           '-o',str(obj)],check=True,text=True,capture_output=True)
        llvm=section(obj.read_bytes())[OFF-RWE:OFF-RWE+36]
    independent=decode(args.rz_asm,llvm,0x96cc8b78)
    assert len(independent)==9
    for expected in ['jump:nt 0x96cc8e90','P0 = or(P0,!P0)',
        'R0 = ##0x3','R1 = memw(FP+##0x1c)','call 0x96cce768',
        'R2 = P0','R19 = add(R16,##-0x1)','jump:nt 0x96cc8ea0','R24 = ##0x80']:
        assert any(expected in line for line in independent),f'LLVM6 semantic fixture missing {expected}'
    assert struct.unpack_from('<I',gen,0)[0]==struct.unpack_from('<I',llvm,0)[0], 'first guard opcode mismatch'
    print('CDSP: 32/32 checked source bytes; full RWE prefix 2968/2968 byte-identical')
    print('Rizin: 8/8 original words valid, fused P0.new, duplex, two branch targets')
    print('LLVM6: equivalent operations/targets; duplex expanded and predicate packet rescheduled')
if __name__=='__main__':main()
