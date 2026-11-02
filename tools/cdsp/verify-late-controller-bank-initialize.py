#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Cross-check original Hexagon V66 bank initialisation and packet markers.

The LLVM6 source is independently written from mnemonic operands. Its packet
schedule uses four different parse markers, but all non-marker bits match
when compared with the typed Rust emitter. Rizin verifies the exact original
packet and fused load. No source firmware bytes are imported by the builder.
"""
import argparse,hashlib,importlib.util,os,subprocess,struct,tempfile
from pathlib import Path
START=0x56b4c
END=0x56b78
RWE_START=0x56000
SHA='febeae7b8e47bad4eeabc159a3c24b74cda26dfa12b384c31de5de30817e06ea'


def parse_section(b:bytes)->bytes:
    src=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
    spec=importlib.util.spec_from_file_location('independent_cdsp_elf32',src)
    assert spec and spec.loader
    module=importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module.parse_elf32_text(b)


def main():
    arg=argparse.ArgumentParser(description=__doc__)
    for n in ('reference','generated','prefix','llvm-mc','rz-asm'):
        arg.add_argument('--'+n,required=True,type=Path)
    a=arg.parse_args()
    ref=a.reference.read_bytes()
    assert hashlib.sha256(ref).hexdigest()==SHA,'Wrong CDSP revision'
    generated=a.generated.read_bytes()
    prefix=a.prefix.read_bytes()
    assert len(generated)==END-START==44 and generated==ref[START:END], 'Source-emitted bank initializer mismatches original'
    assert len(prefix)==END-RWE_START==2936 and prefix==ref[RWE_START:END], 'Joined RWE prefix differs from original'
    with tempfile.TemporaryDirectory(prefix='cdsp-late-bank-init-') as td:
        obj=Path(td)/'independent.o'
        subprocess.run([str(a.llvm_mc),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj',
            str(Path(__file__).with_name('late-controller-bank-initialize-semantic.s')),
            '-o',str(obj)],check=True,text=True,capture_output=True)
        llvm=parse_section(obj.read_bytes())[START-RWE_START:END-RWE_START]
    assert len(llvm)==len(generated)
    llvmwords=struct.unpack('<11I',llvm)
    generatedwords=struct.unpack('<11I',generated)
    altered=[]
    for i,(x,y) in enumerate(zip(llvmwords,generatedwords)):
        if x!=y:
            altered.append(i)
            assert (x^y)&~0xc000==0, f'LLVM6 changed instruction operands at +{i*4:#x}'
    assert altered==[0,4,7,9], f'Unexpected LLVM6 differences: {altered}'
    env=os.environ.copy()
    runtime=next((p/'ubuntu-runtime/build-deps-flat' for p in a.rz_asm.resolve().parents
         if (p/'ubuntu-runtime/build-deps-flat').is_dir()),None)
    if runtime:env['LD_LIBRARY_PATH']=str(runtime)+':'+env.get('LD_LIBRARY_PATH','')
    run=subprocess.run([str(a.rz_asm),'-a','hexagon','-b','32','-o','0x96cc8b4c','-D',generated.hex()],
         capture_output=True,text=True,check=True,env=env)
    lines=[x for x in run.stdout.splitlines() if x.startswith('0x')]
    assert len(lines)==11 and not any('invalid' in x.lower() for x in lines), 'Original Rizin decode fails'
    for idx,needle in [(0,'R3 = R16'),(1,'add(SP,##0x1c)'),
        (3,'memw(R21=##0xc0367850)'),(4,'R3 -= lsr(R2,#0xc)'),
        (5,'combine(##-0x1,#0x4)'),(6,'memw(R29+#0x0) = R18'),
        (7,'memw(SP+#0xc) = ##0x10'),(8,'memw(SP+#0x8) = ##0x0'),
        (9,'call 0x96cceaf0'),(10,'memw(SP+#0x4) = ##0x4')]:
        assert needle in lines[idx],f'Wrong Hexagon mnemonic at word {idx}: {lines[idx]}'
    print('CDSP: 44 typed executable bytes match the pinned source; 2936-byte RWE prefix identical')
    print('LLVM6: 11/11 operand words verified, four harmless packet-marker reschedules')
    print('Rizin V66: 11 original words valid, extended load, duplex and helper call verified')
if __name__=='__main__':main()
