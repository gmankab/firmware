#!/usr/bin/env python3
"""Independent LLVM/Clang round-trip of reviewed Venus ARM-mode regions.

Inputs: privately provided original Venus MBN, explicit runtime range and
manually classified data ranges. The script reassembles *mnemonics*, never
copies instruction opcodes. Values in manually identified data pools come from
the private input reference and are not counted as independent executable code.

This is corroboration of an independently authored typed-Rust emitter, not a
replacement for that emitter or proof of semantic/source decompilation.
"""
from __future__ import annotations
import argparse,re,shutil,struct,subprocess,tempfile
from pathlib import Path

LINE=re.compile(r'^\s*([0-9a-f]+):\s+[0-9a-f]{8}\s+(\S+)\s*(.*)$',re.I)
BRANCH={'b','bl','blx','beq','bne','bhi','bcc','bcs','bls','blt','bgt','bge','ble','bpl','bmi'}
def integer(x:str)->int:return int(x,0)
def validate_ranges(start:int,end:int,data:list[tuple[int,int]])->None:
    if start%4 or end%4 or not(0xa8000<=start<end<=0x1029f4):raise ValueError('invalid owned ARM32 range')
    for a,b in data:
        if a%4 or b%4 or not(start<=a<b<=end):raise ValueError('unmapped or unaligned data range')
    for (_,b),(c,_) in zip(data,data[1:]):
        if b>c:raise ValueError('overlapping data ranges')

def run(src:Path,start:int,end:int,data:list[tuple[int,int]])->None:
    validate_ranges(start,end,data)
    firmware=src.read_bytes()
    ref=firmware[0x4000+start:0x4000+end]
    if len(ref)!=end-start:raise ValueError('truncated reference firmware')
    if not all(shutil.which(x) for x in ['clang','llvm-objdump','llvm-objcopy']):
        raise RuntimeError('clang, llvm-objdump and llvm-objcopy are required')
    with tempfile.TemporaryDirectory(prefix='agent20-06-clang-') as tmp:
        d=Path(tmp)
        (d/'ref.bin').write_bytes(ref)
        (d/'ref.S').write_text('.syntax unified\n.arm\n.text\n.global entry\nentry:\n.incbin "'+str(d/'ref.bin')+'"\n')
        subprocess.run(['clang','--target=armv7a-none-eabi','-c',str(d/'ref.S'),'-o',str(d/'ref.o')],check=True)
        disasm=subprocess.run(['llvm-objdump','-d',f'--adjust-vma=0x{start:x}',str(d/'ref.o')],capture_output=True,text=True,check=True).stdout
        instruction={}
        for line in disasm.splitlines():
            m=LINE.match(line)
            if m:instruction[int(m[1],16)]=(m[2],m[3].split('@')[0].strip())
        assemble=['.syntax unified','.arm','.text',f'.org 0x{start:x}','.global verifier_region','verifier_region:']
        code_bytes=0;data_bytes=0
        for addr in range(start,end,4):
            if any(a<=addr<b for a,b in data):
                value=struct.unpack_from('<I',firmware,0x4000+addr)[0]
                assemble.append(f'    .word 0x{value:08x} // 0x{addr:x} explicitly classified data')
                data_bytes+=4
                continue
            if addr not in instruction:raise ValueError(f'missing decoder output at ARM address 0x{addr:x}')
            mn,oper=instruction[addr]
            if mn=='<unknown>':raise ValueError(f'unrecognized ARM32 instruction at 0x{addr:x}')
            oper=re.sub(r'\s*<[^>]*>','',oper).strip()
            if mn in BRANCH and oper.startswith('0x'):
                distance=int(oper.split()[0],16)-addr
                oper='.'+('+' if distance>=0 else '-')+hex(abs(distance))
            assemble.append(f'    {mn} {oper} // 0x{addr:x}')
            code_bytes+=4
        (d/'reassembly.S').write_text('\n'.join(assemble)+'\n')
        subprocess.run(['clang','--target=armv7a-none-eabi','-c',str(d/'reassembly.S'),'-o',str(d/'reassembly.o')],check=True)
        subprocess.run(['llvm-objcopy','--only-section=.text','-O','binary',str(d/'reassembly.o'),str(d/'reassembly-full.bin')],check=True)
        reassembled=(d/'reassembly-full.bin').read_bytes()[start:end]
        if len(reassembled)!=end-start:raise ValueError('reassembled section incorrectly sized')
        if reassembled!=ref:
            idx=next(i for i,(x,y) in enumerate(zip(reassembled,ref)) if x!=y)
            raise ValueError(f'Clang assembled bytes differ from reference at 0x{start+idx:x}: {reassembled[idx]:02x} != {ref[idx]:02x}')
        print(f'Independent A32 Clang/LLVM round-trip exact: [{start:#x},{end:#x}), {code_bytes} instruction bytes, {data_bytes} classified data bytes, 0 differences')

if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('firmware',type=Path)
    p.add_argument('start',type=integer)
    p.add_argument('end',type=integer)
    p.add_argument('--data-range',action='append',default=[],metavar='START:END')
    args=p.parse_args()
    data=[tuple(integer(z) for z in item.split(':')) for item in args.data_range]
    run(args.firmware,args.start,args.end,data)
