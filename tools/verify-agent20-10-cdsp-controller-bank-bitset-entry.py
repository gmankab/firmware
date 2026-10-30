#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent CDSP bank-bitset entry check via LLVM6 and Rizin V66.

LLVM6 expands one original two-store duplex into separate words. Its
non-duplex operand fields match, with two parse-marker reschedules. This
is an instruction semantic cross-check, not claimed exact LLVM reassembly.
"""
import argparse,hashlib,importlib.util,os,subprocess,struct,tempfile
from pathlib import Path
START=0x56b98;END=0x56bb4;RWE=0x56000
SHA='febeae7b8e47bad4eeabc159a3c24b74cda26dfa12b384c31de5de30817e06ea'
def elf_text(b):
    p=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
    sp=importlib.util.spec_from_file_location('cdsp_bite_elf',p)
    assert sp and sp.loader
    m=importlib.util.module_from_spec(sp);sp.loader.exec_module(m)
    return m.parse_elf32_text(b)
def decode(exe,b,addr):
    env=os.environ.copy()
    runtime=next((p/'ubuntu-runtime/build-deps-flat' for p in exe.resolve().parents
       if (p/'ubuntu-runtime/build-deps-flat').is_dir()),None)
    if runtime:env['LD_LIBRARY_PATH']=str(runtime)+':'+env.get('LD_LIBRARY_PATH','')
    cp=subprocess.run([str(exe),'-a','hexagon','-b','32','-o',hex(addr),'-D',b.hex()],
       check=True,capture_output=True,text=True,env=env)
    lines=[x for x in cp.stdout.splitlines() if x.startswith('0x')]
    if not lines or any('invalid' in x.lower() for x in lines):raise ValueError('invalid V66 decode')
    return lines
def main():
    ap=argparse.ArgumentParser(description=__doc__)
    for n in ('generated','prefix','reference','llvm-mc','rz-asm'):
        ap.add_argument('--'+n,required=True,type=Path)
    a=ap.parse_args()
    ref=a.reference.read_bytes();generated=a.generated.read_bytes();prefix=a.prefix.read_bytes()
    assert hashlib.sha256(ref).hexdigest()==SHA,'wrong pinned CDSP SHA-256'
    assert len(generated)==28 and generated==ref[START:END],'typed bitmap entry differs from original'
    assert len(prefix)==2996 and prefix==ref[RWE:END],'joined CDSP RWE prefix mismatch'
    lines=decode(a.rz_asm,generated,0x96cc8b98)
    assert len(lines)==7
    for idx,needle in [(0,'immext(##0xc028c100)'),(1,'R27:26 = combine(#0x0,##0xc028c100)'),
       (2,'immext(##0xc0364ac0)'),(3,'R20 = ##-0x3fc9b520'),
       (4,'R23:22 = combine(##0x0,#0x0)'),
       (5,'memw(R29+#0x18) = R18 ; memw(R29+#0x14) = R19'),
       (6,'R1:0 = memd(R26+##0x0)')]:
        assert needle in lines[idx], f'wrong source V66 word {idx}: {lines[idx]}'
    with tempfile.TemporaryDirectory(prefix='cdsp-bank-bitset-') as td:
        obj=Path(td)/'independent.o'
        subprocess.run([str(a.llvm_mc),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj',
           str(Path(__file__).with_name('agent20-10-cdsp-controller-bank-bitset-entry-semantic.s')),
           '-o',str(obj)],check=True,capture_output=True,text=True)
        llvm=elf_text(obj.read_bytes())[START-RWE:START-RWE+32]
    ilines=decode(a.rz_asm,llvm,0x96cc8b98)
    assert len(ilines)==8
    llvm_words=struct.unpack('<8I',llvm);orig=struct.unpack('<7I',generated)
    for i in [0,1,2,3,4]:
        assert (llvm_words[i]&~0xc000)==(orig[i]&~0xc000),f'LLVM operand mismatch at first word {i}'
    assert llvm_words[7]==orig[6], 'LLVM source 64-bit load differs'
    for needle in ('memw(FP+##0x18) = R18','memw(FP+##0x14) = R19'):
        assert any(needle in x or needle.replace('FP','R29').replace('##','#') in x for x in ilines), 'LLVM independent fixture missing '+needle
    print('CDSP bitmap: exact 28/28 source bytes and 2996-byte RWE prefix; correct reference SHA')
    print('LLVM6: five checked initial words + shifted memd match; two-store duplex expanded')
    print('Rizin V66: seven original words parsed, real two-store duplex and byte offsets verified')
if __name__=='__main__':main()
