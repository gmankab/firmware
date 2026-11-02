#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Assemble full Novatek upper ILM from *verified source* and *external OEM calibration*.

Only the sparse source fragments are reconstructed; irregular calibration
is an explicitly external dependency, never embedded in Git or supplied as a
deliverable. This tool insists on exact pinned digests, no implicit gap fill,
and no overlaps, and rejects a target path inside the working Git repository.
Full-file matching via reference-owned OEM data is NOT proof of independently
reconstructed firmware source or of hardware behavior.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import tempfile

START, END = 0x10000, 0x1ac80
CAL_START, CAL_END = 0x18620, 0x195b0
PINNED_FIRMWARE_SHA256 = '8b845514621ebfd95759a0919f3aa3d825314abb478a7cad1c3fb4c6872b70b5'
PINNED_SOURCE_SHA256 = '205b76b293a597c191e667aa23bd6fcadf8c67eb90d3a891576361377445ee18'
PINNED_CAL_SHA256 = 'b61f09fc42c18ff4f1bc768ed1f59657a57f5e3d16d473aaeb3b3c94005b5007'
PINNED_COMPLETE_SHA256 = '257855053abdadc874d2b24d05f52dfcec80d76da6ca4455b8614081f82a8fbd'
CAL_GROUPS = [
    (0x18620,0x18c60,'fe8f99f8148af1791cf7d41dc0a88b7f61423e5c11746149bf95f9ba5db09b85'),
    (0x18c60,0x192a0,'b2e20830431ead4ceb1c3479b54abbe3f3a28904049bf2484a2024ebb4c54a8d'),
    (0x192a0,0x195b0,'dce8d43bd5df2bcec520636071b567c8e24e6d3861a29bc5f8cd2a1510e40648'),
]
ROOT = Path(__file__).resolve().parents[3]


def digest(data:bytes)->str:
    return hashlib.sha256(data).hexdigest()


def checked_external_calibration(raw:bytes, *, expected_hash=PINNED_CAL_SHA256,
                                 components=CAL_GROUPS)->bytes:
    if len(raw)!=CAL_END-CAL_START:
        raise ValueError(f'external calibration has {len(raw)} bytes, required {CAL_END-CAL_START}')
    if digest(raw)!=expected_hash:
        raise ValueError('external calibration SHA-256 does not match pinned revision')
    for start,end,expected in components:
        lo,hi = start-CAL_START,end-CAL_START
        if lo<0 or hi>len(raw) or lo>=hi or digest(raw[lo:hi])!=expected:
            raise ValueError(f'external calibration component SHA-256 mismatch at {start:#x}')
    return raw


def assemble_source_and_calibration(manifest:dict, files:dict[str,bytes], calibration:bytes,
                                    *, source_hash:str=PINNED_SOURCE_SHA256,
                                    complete_hash:str=PINNED_COMPLETE_SHA256,
                                    calibration_hash:str=PINNED_CAL_SHA256,
                                    calibration_parts=CAL_GROUPS)->tuple[bytes,dict]:
    if manifest.get('reserved_start')!=START or manifest.get('reserved_end')!=END:
        raise ValueError('invalid upper-ILM reservation boundaries')
    if manifest.get('complete_image') is not False or manifest.get('checkpoint50_integration') is not False:
        raise ValueError('unexpected unsupported full image / checkpoint50 claim')
    if manifest.get('covered_bytes')!=40_176:
        raise ValueError('incorrect source-derived coverage count')
    fragments=manifest.get('fragments')
    if not isinstance(fragments,list) or len(fragments)!=15:
        raise ValueError('unexpected fragment count')
    if manifest.get('unresolved')!=[{'file_start':CAL_START,'file_end':CAL_END,'bytes':CAL_END-CAL_START}]:
        raise ValueError('only the pinned external-calibration gap may be missing')
    calibration=checked_external_calibration(calibration,expected_hash=calibration_hash,components=calibration_parts)
    parts=[]; sourced=[];cursor=START; source_exec=source_data=source_fill=0
    for fragment in fragments:
        start,end=fragment['file_start'],fragment['file_end']
        filename=fragment['file']
        if not isinstance(filename,str) or filename!=Path(filename).name:
            raise ValueError('unsafe fragment filename')
        if start==CAL_END and cursor==CAL_START:
            parts.append(calibration)
            cursor=CAL_END
        if start!=cursor or end<=start or end>END:
            raise ValueError(f'nonadjacent/overlapping/out-of-range source fragment at {start:#x}')
        raw=files.get(filename)
        if raw is None or len(raw)!=end-start:
            raise ValueError(f'missing/truncated source fragment {filename}')
        source_exec += fragment['executable_bytes']
        source_data += fragment['data_bytes']
        source_fill += fragment['layout_fill_bytes']
        if (fragment['executable_bytes']+fragment['data_bytes']+fragment['layout_fill_bytes']) != len(raw):
            raise ValueError(f'bad source byte classes at {start:#x}')
        sourced.append(raw)
        parts.append(raw)
        cursor=end
    if cursor!=END:
        raise ValueError(f'last fragment ends at {cursor:#x}, expected {END:#x}')
    sourced_blob=b''.join(sourced)
    if len(sourced_blob)!=40_176 or digest(sourced_blob)!=source_hash:
        raise ValueError('generated source-fragment hash differs from the reviewed version')
    complete=b''.join(parts)
    if len(complete)!=END-START or digest(complete)!=complete_hash:
        raise ValueError('source/external upper-ILM combination does not match the pinned revision')
    if (source_exec,source_data,source_fill)!=(34_172,5_995,9):
        raise ValueError('source byte-class counts no longer match review')
    report={
        'schema':'agent20-18.reference-assisted.v1',
        'scope':'upper ILM fragment only; not whole firmware',
        'file_start':START,'file_end':END,'runtime_start':START-0x200,
        'runtime_end':END-0x200,'bytes':len(complete),
        'sha256':digest(complete),'pin_firmware_sha256':PINNED_FIRMWARE_SHA256,
        'source_generated_bytes':len(sourced_blob),'source_sha256':digest(sourced_blob),
        'typed_executable_bytes':source_exec,'serialized_data_bytes':source_data,
        'source_alignment_bytes':source_fill,
        'external_original_calibration_bytes':len(calibration),
        'external_calibration_sha256':digest(calibration),
        'byte_mismatches_against_pinned_upper_interval':0,
        'source_only_complete':False,'uses_reference_calibration':True,
        'full_firmware_complete':False,'signing_or_hardware_verified':False,
        'checkpoint50_integration':False,
    }
    return complete, report


def refuse_inside_git_tree(output:Path):
    resolved=output.resolve()
    if resolved.is_relative_to(ROOT):
        raise ValueError('output containing external OEM calibration must not be placed inside Git source tree')


def load_fragments(directory:Path):
    doc=json.loads((directory/'manifest.json').read_text())
    files={}
    for f in doc['fragments']:
        name=f['file']
        if name!=Path(name).name:raise ValueError('invalid fragment file path')
        files[name]=(directory/name).read_bytes()
    return doc, files


def write_atomic(output:Path,contents:bytes):
    output.parent.mkdir(parents=True,exist_ok=True)
    with tempfile.NamedTemporaryFile(dir=output.parent,prefix='.upper-ilm-',delete=False) as temp:
        name=Path(temp.name);temp.write(contents)
    try:os.replace(name,output)
    finally:name.unlink(missing_ok=True)


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--source-fragments',type=Path,required=True,
                        help='directory exported by the Rust novatek-upper-ilm emitter')
    parser.add_argument('--external-calibration',type=Path,required=True,
                        help='original 3984-byte calibration outside Git, SHA-256 pinned')
    parser.add_argument('--output',type=Path,required=True)
    parser.add_argument('--report',type=Path)
    parser.add_argument('--reference',type=Path,help='optional pinned whole original for independent comparison')
    args=parser.parse_args()
    refuse_inside_git_tree(args.output)
    manifest,files=load_fragments(args.source_fragments)
    calibration=args.external_calibration.read_bytes()
    result,report=assemble_source_and_calibration(manifest,files,calibration)
    if args.reference:
        reference=args.reference.read_bytes()
        if len(reference)!=196_608 or digest(reference)!=PINNED_FIRMWARE_SHA256:
            raise ValueError('wrong complete reference revision or truncated firmware')
        if result!=reference[START:END]:
            raise ValueError('source/external combination differs from original firmware')
        report['verified_against_original_firmware']=True
    else:report['verified_against_original_firmware']=False
    write_atomic(args.output,result)
    if args.report:
        refuse_inside_git_tree(args.report)  # avoid accidentally committing reference-dependent traces
        args.report.parent.mkdir(parents=True,exist_ok=True)
        args.report.write_text(json.dumps(report,indent=2)+'\n')
    print(json.dumps(report,indent=2))

if __name__=='__main__':main()
