#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent instruction/packet audit of the CDSP cursor/return loop.

LLVM6 assembles the named input operations, but it moves the two new-value
consumer branches ahead of their loads and emits explicit P1 comparisons.
The reference's fused new-value and P0.new forms require separate Rizin V66
verification; LLVM6 output is NOT represented as an exact rebuild here.
"""
import argparse,importlib.util,os,subprocess,tempfile
from pathlib import Path
START=0x56aa8;END=0x56adc;PHYSICAL=0x96cc8aa8

def words(b:bytes)->list[int]:
 if len(b)%4:raise ValueError('unaligned packet region')
 return [int.from_bytes(b[i:i+4],'little') for i in range(0,len(b),4)]

def text_section(obj:bytes)->bytes:
 spec=importlib.util.spec_from_file_location('hexagon_entry',Path(__file__).with_name('verify-cdsp-hexagon-entry.py'))
 if spec is None or spec.loader is None:raise ValueError('missing independent ELF32 section parser')
 m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
 return m.parse_elf32_text(obj)

def decode(rz:Path,b:bytes)->str:
 env=os.environ.copy()
 lib=next((anc/'ubuntu-runtime/build-deps-flat' for anc in rz.resolve().parents
          if (anc/'ubuntu-runtime/build-deps-flat').is_dir()),None)
 if lib is not None:env['LD_LIBRARY_PATH']=str(lib)+':'+env.get('LD_LIBRARY_PATH','')
 p=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PHYSICAL),'-D',b.hex()],env=env,capture_output=True,text=True,check=True)
 lines=[l for l in p.stdout.splitlines() if l.startswith('0x')]
 if len(lines)!=len(b)//4 or any('invalid' in l.lower() for l in lines):raise ValueError('Rizin V66 packet decode is incomplete')
 return '\n'.join(lines).lower().replace(' ','')

def verify(llvm:Path,rz:Path,generated:Path,reference:Path,prefix:Path|None):
 data=generated.read_bytes();orig=reference.read_bytes()
 if len(data)!=52 or data!=orig[START:END]:raise ValueError('cursor region differs from pinned CDSP reference')
 if prefix is not None:
  expected=orig[0x56000:END]
  if len(expected)!=2780 or prefix.read_bytes()!=expected:raise ValueError('2780-byte RWE prefix mismatch')
 with tempfile.TemporaryDirectory(prefix='cdsp-record-cursor-') as t:
  obj=Path(t)/'cursor.o'
  subprocess.run([str(llvm),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj',
    str(Path(__file__).with_name('late-controller-record-advance-semantic.s')),'-o',str(obj)],
    check=True,capture_output=True,text=True)
  independently=text_section(obj.read_bytes())[0x118:0x118+52]
 if len(independently)!=52:raise ValueError('independent LLVM fixture does not contain 13 words')
 own,other=words(data),words(independently)
 for i in [0,2,7,10,11,12]:
  if own[i]!=other[i]:raise ValueError(f'independent LLVM differs at unchanged word {i}')
 for i in [1,8]:
  if own[i]&~0xc000!=other[i]&~0xc000:raise ValueError(f'LLVM differs beyond packet marker at word {i}')
 if own[5]^other[5]!=0xa000:
  raise ValueError('LLVM P0 predicate qualifier differs beyond expected new-value/parse bits')
 src,alt=decode(rz,data),decode(rz,independently)
 for t in ['r22=add(r22,#1)','r17=memw(r16+#0x10)','r2=add(r17,r23)',
    'r23=add(r23,##0x8)','jump:t0x96cc8a30','jump:t0x96cc8990',
    'r2=+mpyi(r21,##0xa0)','r2=memub(r20+r2<<#0x0)',
    'dealloc_return(fp)']:
  if t not in src or t not in alt:raise ValueError('Hexagon semantics/branch target missing: '+t)
 for t in ['if(!cmp.eq(r2.new,#0x0))','if(!p0.new)','if(p0.new)']:
  if t not in src:raise ValueError('reference V66 new-value condition missing: '+t)
 if 'p1=cmp.eq(r2,#0x0)' not in alt:raise ValueError('LLVM6 explicit predicate alternative missing')
 print('PASS: exact 52-byte source-generated cursor/return interval; all 13 original words decoded')
 print('PASS: LLVM6 fixture: six exact words, two parse-only differences and one P0.new qualifier; altered packet schedule documented')
 print('PASS: Rizin V66: physical backedges 0x96cc8a30, 0x96cc8990 and original new-value guards')
 if prefix:print('PASS: contiguous RWE prefix 2780 bytes (2724 executable + 56 layout)')

if __name__=='__main__':
 p=argparse.ArgumentParser(description=__doc__)
 for name in ('llvm-mc','rz-asm','generated','reference','prefix'):
  p.add_argument('--'+name,type=Path,required=name!='prefix')
 a=p.parse_args();verify(a.llvm_mc,a.rz_asm,a.generated,a.reference,a.prefix)
