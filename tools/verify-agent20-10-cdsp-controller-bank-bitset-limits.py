#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Independent LLVM6/Rizin validation of the bitmap limit check packets."""
import argparse,hashlib,importlib.util,os,struct,subprocess,tempfile
from pathlib import Path
START=0x56bd0;END=0x56bf0;RWE=0x56000
SHA='febeae7b8e47bad4eeabc159a3c24b74cda26dfa12b384c31de5de30817e06ea'
def elf_text(raw):
    path=Path(__file__).with_name('verify-cdsp-hexagon-entry.py');spec=importlib.util.spec_from_file_location('cdsp_limits_elf',path)
    assert spec and spec.loader
    m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
    return m.parse_elf32_text(raw)
def decode(exe,data,addr):
    env=os.environ.copy(); runtime=next((p/'ubuntu-runtime/build-deps-flat' for p in exe.resolve().parents if (p/'ubuntu-runtime/build-deps-flat').is_dir()),None)
    if runtime:env['LD_LIBRARY_PATH']=str(runtime)+':'+env.get('LD_LIBRARY_PATH','')
    cp=subprocess.run([str(exe),'-a','hexagon','-b','32','-o',hex(addr),'-D',data.hex()],check=True,capture_output=True,text=True,env=env)
    lines=[x for x in cp.stdout.splitlines() if x.startswith('0x')]
    if len(lines)!=8 or any('invalid' in x.lower() for x in lines):raise ValueError('bitmap limit code failed Rizin V66 decode: '+cp.stdout)
    return lines
def main():
    ap=argparse.ArgumentParser(description=__doc__)
    for k in ('generated','prefix','reference','llvm-mc','rz-asm'):ap.add_argument('--'+k,required=True,type=Path)
    a=ap.parse_args();ref=a.reference.read_bytes();gen=a.generated.read_bytes();joined=a.prefix.read_bytes()
    assert hashlib.sha256(ref).hexdigest()==SHA,'wrong pinned CDSP revision'
    assert len(gen)==32 and gen==ref[START:END],'source-built bitmap limit 32 bytes differ'
    assert len(joined)==END-RWE and joined==ref[RWE:END],'source-built full prefix mismatch'
    original=decode(a.rz_asm,gen,0x96cc8bd0)
    expectations={0:'R3 = sub(##0x0,R16)',1:'R4 = and(R1,R3)',2:'R17 = extractu(R4,#0x14,#0x0)',
                 3:'0x96cc8d40',4:'R4 = R17',5:'R4 += add(R16,##-0x1)',6:'0x96cc8d40',7:'R4 = extractu(R0,#0x17,#0x1)'}
    for k,needle in expectations.items():assert needle in original[k],f'original instruction {k}: {original[k]}'
    with tempfile.TemporaryDirectory(prefix='cdsp-bank-limits-') as td:
        obj=Path(td)/'limits.o'
        subprocess.run([str(a.llvm_mc),'-triple=hexagon','-mcpu=hexagonv60','-filetype=obj',str(Path(__file__).with_name('agent20-10-cdsp-controller-bank-bitset-limits-semantic.s')),'-o',str(obj)],check=True,capture_output=True,text=True)
        llvm=elf_text(obj.read_bytes())[START-RWE:END-RWE]
    decoded=decode(a.rz_asm,llvm,0x96cc8bd0)
    orig_w=struct.unpack('<8I',gen);llvm_w=struct.unpack('<8I',llvm)
    assert orig_w[:7]==llvm_w[:7], 'LLVM first seven words differ'
    assert (orig_w[7]^llvm_w[7])==0x8000,'LLVM trailing extractu differs beyond packet marker'
    assert '0x96cc8d40' in decoded[3] and '0x96cc8d40' in decoded[6]
    print('CDSP bitmap limits 32 bytes and RWE prefix 3056 bytes: exact reference match')
    print('LLVM6: 7/8 original instruction words byte-identical, last differs only in parse marker')
    print('Rizin V66: original 8 words decoded, shared out-of-range target checked twice')
if __name__=='__main__':main()
