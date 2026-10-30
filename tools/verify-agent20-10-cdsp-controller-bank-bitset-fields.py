#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""LLVM6/Rizin V66 check of CDSP bitmap extraction with compressed duplex."""
import argparse,hashlib,importlib.util,os,struct,subprocess,tempfile
from pathlib import Path
START=0x56bf0;END=0x56c10;RWE=0x56000
SHA='febeae7b8e47bad4eeabc159a3c24b74cda26dfa12b384c31de5de30817e06ea'
def elf_text(data):
    p=Path(__file__).with_name('verify-cdsp-hexagon-entry.py');s=importlib.util.spec_from_file_location('hexagon_bitmap_fields',p)
    assert s and s.loader
    m=importlib.util.module_from_spec(s);s.loader.exec_module(m)
    return m.parse_elf32_text(data)
def decode(exe,b,addr,expected):
    env=os.environ.copy();rt=next((p/'ubuntu-runtime/build-deps-flat' for p in exe.resolve().parents if (p/'ubuntu-runtime/build-deps-flat').is_dir()),None)
    if rt:env['LD_LIBRARY_PATH']=str(rt)+':'+env.get('LD_LIBRARY_PATH','')
    proc=subprocess.run([str(exe),'-a','hexagon','-b','32','-o',hex(addr),'-D',b.hex()],check=True,capture_output=True,text=True,env=env)
    lines=[x for x in proc.stdout.splitlines() if x.startswith('0x')]
    if len(lines)!=expected or any('invalid' in x.lower() for x in lines):
        raise ValueError('Hexagon bitmap field code failed packet decode: '+proc.stdout)
    return lines
def main():
    ap=argparse.ArgumentParser(description=__doc__)
    for k in ('generated','prefix','reference','llvm-mc','rz-asm'):ap.add_argument('--'+k,type=Path,required=True)
    a=ap.parse_args();ref=a.reference.read_bytes();gen=a.generated.read_bytes();pre=a.prefix.read_bytes()
    assert hashlib.sha256(ref).hexdigest()==SHA,'incorrect pinned CDSP revision'
    assert len(gen)==32 and gen==ref[START:END],'bitmap-field bytes differ'
    assert len(pre)==END-RWE and pre==ref[RWE:END],'RWE prefix differs'
    d=decode(a.rz_asm,gen,0x96cc8bf0,8)
    exp={0:'R7:6 = lsr(R1:0,#0x26)',1:'memw(R20+#0x0) = R17',2:'R8 = lsr(R0,#0x1d)',3:'R9 = lsr(R0,#0x19)',4:'R3 = and(R4,R3)',5:'R18 = R6',6:'immext(##0x800000)',7:'R18 = or(R3,and(R18,##0x800000))'}
    for i,t in exp.items():
        if i==1:
            assert 'R5 = memw(R20+#0x0)' in d[i] and 'memw(R29+#0x0) = R17' in d[i],d[i]
        else:assert t in d[i],f'Rizin word {i}: {d[i]}'
    with tempfile.TemporaryDirectory(prefix='cdsp-bitfields-') as td:
        obj=Path(td)/'independent.o'
        subprocess.run([str(a.llvm_mc),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj',str(Path(__file__).with_name('agent20-10-cdsp-controller-bank-bitset-fields-semantic.s')),'-o',str(obj)],check=True,capture_output=True,text=True)
        llvm=elf_text(obj.read_bytes())[START-RWE:END-RWE+4]
    indep=decode(a.rz_asm,llvm,0x96cc8bf0,9)
    ow=struct.unpack('<8I',gen);lw=struct.unpack('<9I',llvm)
    assert (ow[0]^lw[0])==0x8000,'original 64-bit shift differs in more than packet marker'
    assert ('R5 = memw(R20+##0x0)' in indep[1] or 'R5 = memw(R20+#0x0)' in indep[1]) and ('memw(FP+##0x0) = R17' in indep[2] or 'memw(FP+#0x0) = R17' in indep[2]), 'LLVM6 independent load/store operations differ'
    assert ow[2]==lw[3], 'LLVM6 scalar shift R8 changed'
    assert (ow[3]^lw[4])==0x8000,'LLVM6 scalar shift R9 differs other than parse marker'
    assert ow[4:7]==lw[5:8], 'LLVM6 register AND/copy/immext words differ'
    assert (ow[7]^lw[8])==0x8000,'LLVM6 final OR compound differs other than parse marker'
    print('Source 32 bytes and 3088-byte prefix: exact; pinned CDSP SHA verified')
    print('LLVM6: equivalent 9 operations, duplex expanded; four parse-marker differences')
    print('Rizin V66: eight original words valid and original two-memory-op duplex verified')
if __name__=='__main__':main()
