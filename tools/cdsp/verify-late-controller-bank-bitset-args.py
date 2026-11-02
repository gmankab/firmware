#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent LLVM6 and Rizin V66 check of bitmap packet arguments."""
import argparse,hashlib,importlib.util,os,struct,subprocess,tempfile
from pathlib import Path
START=0x56c10;END=0x56c30;RWE=0x56000
SHA='febeae7b8e47bad4eeabc159a3c24b74cda26dfa12b384c31de5de30817e06ea'
def elf_text(b):
    p=Path(__file__).with_name('verify-cdsp-hexagon-entry.py');spec=importlib.util.spec_from_file_location('hexagon_bitmap_args',p)
    assert spec and spec.loader
    m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
    return m.parse_elf32_text(b)
def decode(exe,payload,addr,expected):
    env=os.environ.copy();rt=next((p/'ubuntu-runtime/build-deps-flat' for p in exe.resolve().parents if (p/'ubuntu-runtime/build-deps-flat').is_dir()),None)
    if rt:env['LD_LIBRARY_PATH']=str(rt)+':'+env.get('LD_LIBRARY_PATH','')
    proc=subprocess.run([str(exe),'-a','hexagon','-b','32','-o',hex(addr),'-D',payload.hex()],check=True,capture_output=True,text=True,env=env)
    lines=[x for x in proc.stdout.splitlines() if x.startswith('0x')]
    if len(lines)!=expected or any('invalid' in x.lower() for x in lines):raise ValueError('bitmap args fail V66 decode: '+proc.stdout)
    return lines
def main():
    ap=argparse.ArgumentParser(description=__doc__)
    for name in ('generated','prefix','reference','llvm-mc','rz-asm'):ap.add_argument('--'+name,required=True,type=Path)
    a=ap.parse_args();ref=a.reference.read_bytes();gen=a.generated.read_bytes();prefix=a.prefix.read_bytes()
    assert hashlib.sha256(ref).hexdigest()==SHA,'not the pinned CDSP reference'
    assert len(gen)==32 and gen==ref[START:END],'32 byte typed source mismatch'
    assert len(prefix)==END-RWE and prefix==ref[RWE:END],'full RWE source layout mismatch'
    orig=decode(a.rz_asm,gen,0x96cc8c10,8)
    assertions={0:'R25:24 = extractu(R7:6,#0x9,#0x11)',1:'R1 = memw(R21+##0x0)',
        2:'R2 = extractu(R2,#0x4,#0x18)',3:'R23:22 = extractu(R7:6,#0x6,#0x14)',
        4:'R0 = add(R29,#0x1c) ; R3 = R16',5:'R24 = or(R2,and(R24,##0x30))',
        6:'R9 = or(R8,and(R9,##0x8))',7:'R2 = add(R5,##0x18)'}
    for idx,needle in assertions.items():assert needle in orig[idx],f'bad original word {idx}: {orig[idx]}'
    with tempfile.TemporaryDirectory(prefix='cdsp-bitmap-args-') as td:
        obj=Path(td)/'fixture.o'
        subprocess.run([str(a.llvm_mc),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj',str(Path(__file__).with_name('late-controller-bank-bitset-args-semantic.s')),'-o',str(obj)],check=True,capture_output=True,text=True)
        llvm=elf_text(obj.read_bytes())[START-RWE:END-RWE+4]
    indep=decode(a.rz_asm,llvm,0x96cc8c10,9)
    ow=struct.unpack('<8I',gen);lw=struct.unpack('<9I',llvm)
    assert ow[1]==lw[1],'independent controller-table word load differs'
    for i,j in [(0,0),(2,2),(3,3),(5,6),(6,7),(7,8)]:
        assert (ow[i]^lw[j])==0x8000,f'independent operand encoding changed at original word {i} beyond parse marker'
    assert 'R0 = add(FP,##0x1c)' in indep[4] and 'R3 = R16' in indep[5], 'LLVM6 expanded duplex semantics differ'
    print('Source 32 bytes plus 3120-byte RWE prefix: exact pinned CDSP reference match')
    print('LLVM6: one unchanged load; six marker-only changes; one duplex expanded to two words')
    print('Rizin V66: eight original words decoded, pair extractions and compact duplex verified')
if __name__=='__main__':main()
