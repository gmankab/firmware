#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Verify source-built V66 bank-count guards and independent semantic assembly.

LLVM6 does not keep original fused `.new` register-producing branch forms:
it emits explicit P0/P1 compare packets and may shorten GP-relative loads.
The full 28-byte typed Rust fragment must still match the pinned CDSP MBN
exactly; all V66 words are independently decoded with Rizin.
"""
import argparse,hashlib,importlib.util,os,subprocess,struct,tempfile
from pathlib import Path
OFF=0x56b30;END=0x56b4c;RWE=0x56000
SHA='febeae7b8e47bad4eeabc159a3c24b74cda26dfa12b384c31de5de30817e06ea'

def text_section(blob:bytes)->bytes:
 src=Path(__file__).with_name('verify-cdsp-hexagon-entry.py')
 spec=importlib.util.spec_from_file_location('cdsp_entry_elf32',src)
 if spec is None or spec.loader is None:raise RuntimeError('missing standalone ELF32 parser')
 mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)
 return mod.parse_elf32_text(blob)

def decode(rz:Path,data:bytes,address:int)->str:
 env=os.environ.copy()
 lib=next((p/'ubuntu-runtime/build-deps-flat' for p in rz.resolve().parents if (p/'ubuntu-runtime/build-deps-flat').is_dir()),None)
 if lib:env['LD_LIBRARY_PATH']=str(lib)+':'+env.get('LD_LIBRARY_PATH','')
 return subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(address),'-D',data.hex()],
     capture_output=True,text=True,check=True,env=env).stdout

def main():
 a=argparse.ArgumentParser(description=__doc__)
 for name in ('llvm-mc','rz-asm','generated','prefix','reference'):
  a.add_argument('--'+name,required=True,type=Path)
 args=a.parse_args()
 ref=args.reference.read_bytes()
 if hashlib.sha256(ref).hexdigest()!=SHA:raise ValueError('wrong CDSP firmware version')
 gen=args.generated.read_bytes();prefix=args.prefix.read_bytes()
 if len(gen)!=28 or gen!=ref[OFF:END]:raise ValueError('V66 controller guards do not match all 28 OEM bytes')
 if len(prefix)!=END-RWE or prefix!=ref[RWE:END]:raise ValueError('joined 2892-byte RWE prefix mismatch')
 # Fused .new at 0x56b34 consumes R18 produced in previous fragment's
 # packet, so decode with the last 4 prefix bytes for valid V66 context.
 original=decode(args.rz_asm,prefix[(0x56b2c-RWE):(0x56b30-RWE)]+gen,0x96cc8b2c)
 lines=[x for x in original.splitlines() if x.startswith('0x')]
 if len(lines)!=8 or any('invalid' in x.lower() for x in lines):raise ValueError('Rizin could not decode all eight words with producer context')
 expected=[('0x96cc8b2c','lsr(R2'),('0x96cc8b30','immext('),('0x96cc8b34','cmp.eq(R18.new'),
           ('0x96cc8b38','immext('),('0x96cc8b3c','memw(GP+'),
           ('0x96cc8b40','lsr(R3'),('0x96cc8b44','immext('),('0x96cc8b48','cmp.gtu(R16.new')]
 for line,(address,mnemonic) in zip(lines,expected):
  if not line.startswith(address) or mnemonic not in line:
   raise ValueError(f'wrong Rizin original bank packet at {address}: {line}')
 if '0x96cc8e70' not in lines[2] or '0x96cc8e80' not in lines[7]:raise ValueError('bank range guards have wrong physical targets')
 with tempfile.TemporaryDirectory(prefix='cdsp-v66-bank-guards-') as tmp:
  obj=Path(tmp)/'semantic.o'
  subprocess.run([str(args.llvm_mc),'-triple=hexagon','-mcpu=hexagonv60',
     '-filetype=obj',str(Path(__file__).with_name('late-controller-bank-guards-semantic.s')),
     '-o',str(obj)],capture_output=True,text=True,check=True)
  independent=text_section(obj.read_bytes())[0xb2c:0xb40]
 other=decode(args.rz_asm,independent,0x96cc8b2c)
 for marker in ('lsr(R2,#0xc)','cmp.eq(R18,#0x0)','0x96cc8e70',
                'memw(GP+##0xc0f8)','lsr(R3,#0xc)','cmp.gtu(R16,R18)','0x96cc8e80'):
  if marker not in other:raise ValueError('LLVM6 independent source does not implement '+marker)
 print('CDSP bank guards: 28/28 byte-identical; joined prefix 2892/2892')
 print('Rizin V66: 8 context-inclusive original words decoded, both original fused .new guards/physical targets valid')
 print('LLVM6: independently reproduces both compares, the GP load and conditional jumps; not the same packet scheduling')
if __name__=='__main__':main()
