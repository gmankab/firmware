#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent LLVM6 and Rizin V66 audit for CDSP bitmap scan.

LLVM6 closes the first setbit packet earlier than the original, moving the
packet-relative predicate branch by one word. The following five instructions
are independently byte-identical. The reference is never embedded in source.
"""
import argparse,hashlib,importlib.util,os,struct,subprocess,tempfile
from pathlib import Path
SOURCE_DIR=Path(__file__).resolve().parents[2]/'sources/xiaomi-nabu/cdsp/late'
START=0x56bb4;END=0x56bd0;RWE=0x56000
SHA='febeae7b8e47bad4eeabc159a3c24b74cda26dfa12b384c31de5de30817e06ea'
def elf_text(raw):
    p=Path(__file__).with_name('verify-cdsp-hexagon-entry.py');spec=importlib.util.spec_from_file_location('hexagon_elf_scan',p)
    assert spec and spec.loader
    m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
    return m.parse_elf32_text(raw)
def decode(exe,payload,address):
    env=os.environ.copy();root=next((d/'ubuntu-runtime/build-deps-flat' for d in exe.resolve().parents if (d/'ubuntu-runtime/build-deps-flat').is_dir()),None)
    if root:env['LD_LIBRARY_PATH']=str(root)+':'+env.get('LD_LIBRARY_PATH','')
    p=subprocess.run([str(exe),'-a','hexagon','-b','32','-o',hex(address),'-D',payload.hex()],check=True,text=True,capture_output=True,env=env)
    lines=[x for x in p.stdout.splitlines() if x.startswith('0x')]
    if len(lines)!=7 or any('invalid' in x.lower() for x in lines):
        raise ValueError('bitmap scan has invalid Hexagon decoded words: '+p.stdout)
    return lines
def main():
    ap=argparse.ArgumentParser(description=__doc__)
    for name in ('generated','prefix','reference','llvm-mc','rz-asm'):ap.add_argument('--'+name,type=Path,required=True)
    a=ap.parse_args();ref=a.reference.read_bytes();gen=a.generated.read_bytes();prefix=a.prefix.read_bytes()
    assert hashlib.sha256(ref).hexdigest()==SHA,'wrong CDSP firmware revision'
    assert len(gen)==END-START and gen==ref[START:END],'typed scan is not original bytes'
    assert len(prefix)==END-RWE and prefix==ref[RWE:END],'RWE prefix mismatch'
    original=decode(a.rz_asm,gen,0x96cc8bb4)
    checks={0:'R2 = setbit(R0,#0x7)',1:'P0 = cmp.eq(R1:0,R23:22)',2:'0x96cc8d4c',
            3:'R3 = ct0(R2)',4:'0x96cc8e30',5:'R3 = asl(R3,#0x1)',6:'R16 = lsl(#0x1,R3)'}
    for k,value in checks.items():
        assert value in original[k],f'V66 original word {k} not decoded as expected: {original[k]}'
    with tempfile.TemporaryDirectory(prefix='cdsp-bankscan-') as d:
        obj=Path(d)/'fixture.o'
        subprocess.run([str(a.llvm_mc),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj',
                        str(SOURCE_DIR / 'late-controller-bank-bitset-scan-semantic.s'),
                        '-o',str(obj)],check=True,capture_output=True,text=True)
        llvm=elf_text(obj.read_bytes())[START-RWE:END-RWE]
    independent=decode(a.rz_asm,llvm,0x96cc8bb4)
    ow=struct.unpack('<7I',gen);lw=struct.unpack('<7I',llvm)
    assert ow[0] == (lw[0] & ~0x8000),'LLVM setbit differs other than its final packet marker'
    assert lw[1]==ow[1],'LLVM 64-bit pair predicate encoding differs'
    assert lw[2]+2==ow[2],'LLVM one-word later packet origin must shorten branch by four bytes'
    assert ow[3:]==lw[3:],'LLVM independent ct0, bounds guard, two shifts must match exactly'
    assert '0x96cc8d4c' in independent[2] and '0x96cc8e30' in independent[4], 'independent branch targets differ'
    print('Source 28 bytes + 3024-byte RWE prefix: EXACT; pinned CDSP SHA verified')
    print('LLVM6: five unchanged words, one setbit parse marker, one branch-distance adjustment')
    print('Rizin: seven original V66 words valid; bit index bound and two branch targets confirmed')
if __name__=='__main__':main()
