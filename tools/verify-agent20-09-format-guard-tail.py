#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independently corroborate CDSP format classifier tail.

Uses original pinned CDSP MBN for byte equality, Rizin V66 semantics for all
82 words, and separately assembled LLVM6 Hexagon source for every operand
field. LLVM may reorder cmp+branch or jump+ext assignment inside packets;
explicit position mapping accounts for that and full raw comparison tests
packet parse markers independently.
"""
from pathlib import Path
import argparse,hashlib,importlib.util,re,subprocess,tempfile

FILE_START=0x5538; FILE_END=0x5680; PC=0x96c0_2538
SHA='febeae7b8e47bad4eeabc159a3c24b74cda26dfa12b384c31de5de30817e06ea'
# Start, LSR, AND-mask, ASL, expected format ID, branch destination.
FORMATS=[
 (0x5538,25,127,25,0x5c000000,0x5564),
 (0x5564,26,63,26,0x50000000,0x55b8),
 (0x558c,24,1,31,0x80000000,0x55b8),
 (0x55b8,24,255,24,0x61000000,0x55ec),
 (0x55ec,21,2047,21,0x96000000,0x5680),
 (0x5618,11,1,31,0x80000000,0x564c),
 (0x564c,11,3,30,0x80000000,0x5680),
]
# Source emits an error-code assignment and jump in one packet; the LLVM
# assembler sometimes schedules jump ahead of the immediate assignment.
ERRORS=[(0x55e0,-0x1002,0x5878),(0x5640,-0x1021,0x5878),(0x5674,-0xa1,0x5878)]
JUMPS=[(0x5560,0x55e0),(0x55b4,0x55e0)]

def signed_hex(u):
 return f'-0x{(1<<32)-u:x}' if u>=1<<31 else f'0x{u:x}'
def sem(s):return ''.join(s.lower().split())

def assemble(mc:Path,source:str)->bytes:
 with tempfile.TemporaryDirectory() as td:
  d=Path(td);(d/'code.s').write_text('.text\n.global entry\nentry:\n'+source+'\n')
  p=subprocess.run([str(mc),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj',
    '-o',str(d/'code.o'),str(d/'code.s')],capture_output=True,text=True)
  if p.returncode:raise ValueError('LLVM6 rejected '+repr(source)+': '+p.stderr)
  spec=importlib.util.spec_from_file_location('elf',Path(__file__).with_name('verify-cdsp-hexagon-entry.py'))
  if not spec or not spec.loader:raise ValueError('Missing ELF32 .text parser')
  mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)
  return mod.parse_elf32_text((d/'code.o').read_bytes())

def build_expected():
 expected={};tests={}
 def insert(address,text,src=None):
  if address in expected:raise AssertionError(f'duplicate word at {address:x}')
  expected[address]=text
  if src is not None: tests.setdefault(src,[]).append(address)
 for start,right,mask,left,fmt,target in FORMATS:
  a=start
  def emit(message,source=None):
   nonlocal a
   insert(a,message,source);a+=4
  emit('R9 = ##0x0','{r9=#0}')
  emit('R11 = R0','{r11=r0}')
  emit(f'R11 = lsr(R11,#0x{right:x})',f'{{r11=lsr(r11,#{right})}}')
  if mask>255:
   emit(f'immext(##0x{mask&~63:x})',f'{{r11=and(r11,##0x{mask:x})}}')
   # The body is independently verified together with the extender below.
   emit(f'R11 = and(R11,##0x{mask:x})')
  else:
   emit(f'R11 = and(R11,##0x{mask:x})',f'{{r11=and(r11,#{mask})}}')
  emit(f'R11 = asl(R11,#0x{left:x})',f'{{r11=asl(r11,#{left})}}')
  emit(f'immext(##0x{fmt&~63:x})',f'{{p0=cmp.eq(r11,##0x{fmt:x})}}')
  emit(f'P0 = cmp.eq(R11,##{signed_hex(fmt)})')
  emit('if (P0) R9 = ##0x1','{if(p0)r9=#1}')
  cmppos=a
  emit('P0 = cmp.eq(R9,##0x1)')
  emit(f'if (!P0.new) jump:nt 0x{0x96c0_0000+target-0x3000:x}')
  # LLVM6 schedules the branch before the P0 producer. Packet length 8.
  space=target-cmppos-8
  assert space>=0 and space%4==0
  assembly=f'{{p0=cmp.eq(r9,#1)\n if(!p0.new) jump:nt target}}\n.space {space}\ntarget:\n{{nop}}'
  tests.setdefault((assembly,'cmpbranch'),[]).extend([cmppos+4,cmppos])
 for start,target in JUMPS:
  insert(start,f'jump 0x{0x96c0_0000+target-0x3000:x}',f'{{jump target}}\n.space {target-start-4}\ntarget:\n{{nop}}')
 for start,value,target in ERRORS:
  unsigned=value & 0xffffffff
  insert(start,f'immext(##0x{unsigned&~63:x})')
  insert(start+4,f'R0 = ##{signed_hex(unsigned)}')
  insert(start+8,f'jump 0x{0x96c0_0000+target-0x3000:x}')
  assembly=f'{{r0=##{value}\n jump target}}\n.space {target-start-12}\ntarget:\n{{nop}}'
  tests.setdefault((assembly,'extjump'),[]).extend([start+8,start,start+4])
 assert len(expected)==82,len(expected)
 assert sorted(expected)==list(range(FILE_START,FILE_END,4))
 return expected,tests

def check(output:Path,ref:Path,rz:Path,mc:Path):
 reference=ref.read_bytes()
 if hashlib.sha256(reference).hexdigest()!=SHA:raise ValueError('Incorrect pinned CDSP reference SHA')
 actual=output.read_bytes()
 if len(actual)!=FILE_END-FILE_START or actual!=reference[FILE_START:FILE_END]:raise ValueError('Source-built guard tail differs from original CDSP')
 expected,tests=build_expected()
 out=subprocess.run([str(rz),'-a','hexagon','-b','32','-o',hex(PC),'-D',actual.hex()],
                    capture_output=True,text=True,check=True).stdout
 rows=[]
 for line in out.splitlines():
  m=re.match(r'^0x([0-9a-fA-F]+)\s+4\s+[0-9a-fA-F]{8}\s+[?/\\\[|]\s+(.*)',line)
  if m:rows.append((int(m[1],16),sem(m[2]).split('<endloop')[0]))
 if len(rows)!=82:raise ValueError(f'Rizin decoded {len(rows)} words, expected 82')
 for address,mnemonic in rows:
  file_offset=FILE_START+address-PC
  if mnemonic!=sem(expected[file_offset]):
   raise ValueError(f'Rizin semantic discrepancy {file_offset:#x}: {mnemonic} expected {expected[file_offset]}')
 matched=set()
 for key,positions in tests.items():
  assembly,kind=key if isinstance(key,tuple) else (key,'scalar')
  compiled=assemble(mc,assembly)
  count=3 if kind=='extjump' else 2 if kind=='cmpbranch' else 2 if assembly.startswith('{p0=cmp.eq(r11,##') or assembly.startswith('{r11=and(r11,##') else 1
  if len(compiled)<count*4:raise ValueError('LLVM6 emitted too few words: '+assembly)
  for j,address in enumerate(positions):
   index=(j % count) if kind!='scalar' else 0
   # For coupled extender forms, compile once and compare the consecutive
   # two source positions. The first word's scalar source shares both positions.
   if kind=='scalar' and count==2: index=0
   opcode=int.from_bytes(compiled[4*index:4*index+4],'little')&~0xc000
   want=int.from_bytes(actual[address-FILE_START:address-FILE_START+4],'little')&~0xc000
   if opcode!=want:raise ValueError(f'LLVM6 independent mismatch file {address:#x}: {opcode:#x}, wanted {want:#x} source={assembly}')
   matched.add(address)
 # Explicitly test the second operand word of extended AND/CMP cases.
 for start,right,mask,left,fmt,target in FORMATS:
  a=start+4*5 + (4 if mask>255 else 0)
  # format extender follows five scalar operations (plus optional mask extender)
  a=start+4*(5+(1 if mask>255 else 0))
  b=assemble(mc,f'{{p0=cmp.eq(r11,##0x{fmt:x})}}')
  op=int.from_bytes(b[4:8],'little')&~0xc000
  want=int.from_bytes(actual[a+4-FILE_START:a+8-FILE_START],'little')&~0xc000
  if op!=want:raise ValueError(f'Extended compare consumer mismatch at {a+4:#x}: {op:#x} vs {want:#x}')
  matched.add(a+4)
  if mask>255:
   mask_address=start+12
   b=assemble(mc,f'{{r11=and(r11,##0x{mask:x})}}')
   op=int.from_bytes(b[4:8],'little')&~0xc000
   want=int.from_bytes(actual[mask_address+4-FILE_START:mask_address+8-FILE_START],'little')&~0xc000
   if op!=want:raise ValueError('Extended mask consumer mismatched at '+hex(mask_address+4))
   matched.add(mask_address+4)
 if len(matched)!=82:raise ValueError(f'Only {len(matched)}/82 LLVM word fields compared; missing {[hex(x) for x in sorted(set(expected)-matched)]}')
 print(f'CDSP codec format tail: {len(actual)} reference-exact bytes, {len(rows)} Rizin V66 decoded, {len(matched)} LLVM6 independent operand comparisons; function incomplete')

if __name__=='__main__':
 p=argparse.ArgumentParser()
 for n in ('rust-output','reference','rz-asm','llvm-mc'):p.add_argument('--'+n,required=True,type=Path)
 a=p.parse_args();check(a.rust_output,a.reference,a.rz_asm,a.llvm_mc)
