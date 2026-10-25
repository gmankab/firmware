#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Verify sparse Agent 18 source against a pinned image and independent tools.

Rust emission consumes no original firmware. The private image is read only
here. GNU assembly and independent high-precision/public-source data generation
are mandatory checks; unresolved gaps remain explicit and are never filled.
"""
import argparse
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import tempfile

PINNED_SHA256 = "8b845514621ebfd95759a0919f3aa3d825314abb478a7cad1c3fb4c6872b70b5"
IMAGE_SIZE = 196608
START, END = 0x10016, 0x17DC0
BASE_COMMIT = "51b1f91f32d32542aa03e114f5a2f4d94b559e2a"
# start, end, executable bytes, data bytes, fill bytes, instruction count
EXPECTED_SPANS = [
    (0x10000,0x10016,22,0,0,7),
    (0x10016,0x17dc0,32102,67,1,11506),
    (0x17dc0,0x185c0,2048,0,0,512),
    (0x185c0,0x18620,0,96,0,0),
    (0x195b0,0x19614,0,100,0,0),
    (0x19614,0x19678,0,100,0,0),
    (0x19678,0x196b8,0,64,0,0),
    (0x196b8,0x196f8,0,64,0,0),
    (0x198dc,0x19994,0,182,2,0),
    (0x19994,0x19a4c,0,182,2,0),
    (0x19a4c,0x19b04,0,182,2,0),
    (0x19b04,0x19bbc,0,182,2,0),
    (0x19bf0,0x1ac80,0,4240,0,0),
]
EXPECTED_GAPS = [(0x18620,0x195b0),(0x196f8,0x198dc),(0x19bbc,0x19bf0)]
EXPECTED_HASHES = {
    0x10016: "6b78bd0f2cd788b7c82a65119859a5e70111dd496f986b1727daed13e6f2aab4",
    0x17dc0: "a58613094f4ce93767d70120205520453bc193fea1c7f0fcaf7f1f0cb124a23c",
    0x185c0: "74f24e4c3557fc128a16f3e76414fd1856327262f3dd49e937c54edf6a755637",
    0x19bf0: "c26af2ad9c57bf0b64d2d1b93b1e911170e63c39d7bf69bfe423b9e8770a8f4c",
}


def run(*args):
    return subprocess.check_output([str(a) for a in args], text=True).strip()


def checked_reference(data):
    digest = hashlib.sha256(data).hexdigest()
    if len(data) != IMAGE_SIZE or digest != PINNED_SHA256:
        raise ValueError(f"wrong reference: size={len(data)}, sha256={digest}")
    return data


def compare(expected, actual, label, start=START):
    if len(actual) != len(expected):
        raise ValueError(f"{label}: length {len(actual)} != {len(expected)}")
    bad = [(i,a,b) for i,(a,b) in enumerate(zip(expected,actual)) if a != b]
    if bad:
        i,a,b = bad[0]
        raise ValueError(f"{label}: {len(bad)} mismatches; first file {start+i:#x}: reference={a:02x}, generated={b:02x}")
    return 0


def validate_manifest(manifest):
    if (manifest["reserved_start"],manifest["reserved_end"]) != (0x10000,0x1ac80):
        raise ValueError("wrong reserved interval")
    actual = [(f["file_start"],f["file_end"],f["executable_bytes"],f["data_bytes"],
               f["layout_fill_bytes"],f["instructions"]) for f in manifest["fragments"]]
    if actual != EXPECTED_SPANS:
        raise ValueError("sparse spans or byte classes differ from audited ownership map")
    gaps = [(g["file_start"],g["file_end"]) for g in manifest["unresolved"]]
    if gaps != EXPECTED_GAPS:
        raise ValueError("unresolved gaps were changed or silently filled")
    if any(g["bytes"] != g["file_end"]-g["file_start"] for g in manifest["unresolved"]):
        raise ValueError("incorrect gap byte count")
    for field,expected in [("covered_bytes",39640),("executable_bytes",34172),
                           ("data_bytes",5459),("layout_fill_bytes",9)]:
        if manifest[field] != expected:
            raise ValueError(f"incorrect {field}")
    if manifest["complete_image"] or manifest["checkpoint50_integration"]:
        raise ValueError("unestablished complete-image or integration claim")
    for f in manifest["fragments"]:
        if Path(f["file"]).name != f["file"]:
            raise ValueError("fragment filename must remain inside export directory")
        if f["runtime_start"] != f["file_start"]-0x200 or f["runtime_end"] != f["file_end"]-0x200:
            raise ValueError("incorrect runtime mapping")


def data_validator():
    path = Path(__file__).with_name("verify_agent20_18_data.py")
    spec = importlib.util.spec_from_file_location("agent18_independent_data", path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def assemble(args, source, runtime, entry, temp, name):
    obj, elf, binary = [temp/(name+ext) for ext in (".o",".elf",".bin")]
    # A complete, explicit layout makes this check independent of linker
    # installation directories and any target-default linker script.
    script = temp/(name+".lds")
    script.write_text('OUTPUT_FORMAT("elf32-nds32le")\nOUTPUT_ARCH(nds32)\n'
                      + f'SECTIONS {{ .text {runtime:#x} : {{ *(.text) }} }}\n')
    run(args.assembler,"-mno-all-relax","-o",obj,source)
    run(args.linker,"--no-relax","-T",script,"-e",entry,"-o",elf,obj)
    run(args.objcopy,"-O","binary","--only-section=.text",elf,binary)
    return binary.read_bytes()


def main():
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument("reference",type=Path)
    p.add_argument("--emitter",type=Path,required=True)
    p.add_argument("--as",dest="assembler",default="nds32le-elf-as")
    p.add_argument("--ld",dest="linker",default="nds32le-elf-ld")
    p.add_argument("--objcopy",default="nds32le-elf-objcopy")
    p.add_argument("--rustc",default=os.environ.get("RUSTC","rustc"))
    p.add_argument("--report",type=Path)
    p.add_argument("--source-only",action="store_true",
        help="explicitly skip GNU assembler/synthetic ISA checks when NDS32 tools are unavailable")
    p.add_argument("--fragment-dir",type=Path)
    args=p.parse_args()
    root=Path(__file__).resolve().parents[3]
    reference=checked_reference(args.reference.read_bytes())
    with tempfile.TemporaryDirectory(prefix="agent20-18-verify-") as temp:
        temp=Path(temp)
        output=args.fragment_dir.resolve() if args.fragment_dir else temp/"fragments"
        run(args.emitter.resolve(),"export",output)
        manifest=json.loads((output/"manifest.json").read_text())
        validate_manifest(manifest)
        generated={}
        digest_list=[]
        for fragment in manifest["fragments"]:
            start,end=fragment["file_start"],fragment["file_end"]
            content=(output/fragment["file"]).read_bytes()
            compare(reference[start:end],content,"typed source",start)
            digest=hashlib.sha256(content).hexdigest()
            if start in EXPECTED_HASHES and digest != EXPECTED_HASHES[start]:
                raise ValueError(f"unexpected audited span digest at {start:#x}")
            generated[start]=content
            digest_list.append(dict(fragment,sha256=digest,byte_mismatches=0))
        sources=root/"reference/xiaomi-nabu/agent20-18"
        gnu_bytes=0
        if not args.source_only:
            gnu_main=assemble(args,sources/"upper-ilm-main.S",0xfe16,"agent18_upper_start",temp,"main")
            gnu_ex9=assemble(args,sources/"upper-ilm-ex9.S",0x17bc0,"agent18_ex9_table",temp,"ex9")
            compare(generated[0x10016],gnu_main,"GNU main",0x10016)
            compare(generated[0x17dc0],gnu_ex9,"GNU EX9",0x17dc0)
            gnu_bytes=len(gnu_main)+len(gnu_ex9)
        independent=data_validator().independent_fragments(root)
        path=Path(__file__).with_name("verify_agent20_18_allocations.py")
        spec=importlib.util.spec_from_file_location("agent18_reference_decoded_allocations",path)
        module=importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        allocation_bytes=module.serialize()
        compare(generated[0x185c0],allocation_bytes,"decoded controller allocation address structure",0x185c0)
        seam_path=Path(__file__).with_name("verify_agent20_18_boundary.py")
        spec=importlib.util.spec_from_file_location("agent18_independent_seam",seam_path)
        boundary_module=importlib.util.module_from_spec(spec)
        spec.loader.exec_module(boundary_module)
        seam_bytes=boundary_module.emit()
        compare(generated[0x10000],seam_bytes,"independent NDS32 bitfield suffix",0x10000)
        if set(independent) != set(generated)-{0x10000,0x10016,0x17dc0,0x185c0}:
            raise ValueError("independent data validator does not cover exactly the declared data spans")
        for start,content in independent.items():
            compare(generated[start],content,"independent data",start)
        # Exercise each of the 161 encoding variants on synthetic operands,
        # independent of the actual firmware's chosen operands.
        matrix_bytes=0
        if not args.source_only:
            run(args.rustc,"--edition=2021","--crate-name","agent18_isa_matrix",
                sources/"upper-isa-matrix.rs","-o",temp/"isa-matrix")
            run(temp/"isa-matrix",temp/"isa-rust.bin")
            matrix=assemble(args,sources/"upper-isa-matrix.S",0x10000,"_start",temp,"isa-gnu")
            compare((temp/"isa-rust.bin").read_bytes(),matrix,"synthetic ISA matrix",0x10000)
            if len(matrix)!=522:
                raise ValueError("synthetic ISA matrix unexpectedly changed length")
            matrix_bytes=len(matrix)
        report={
            "agent":"20-18","checkpoint":4,
            "git_head":run("git","-C",root,"rev-parse","HEAD"),
            "actual_parent":BASE_COMMIT,"checkpoint50_integration":False,
            "reference_sha256":PINNED_SHA256,"reference_size":len(reference),
            "reserved_file_start":0x10000,"reserved_file_end":0x1ac80,
            "ordinary_instructions":11506,"ex9_instructions":512,
            "executable_bytes":34172,"data_bytes":5459,"layout_fill_bytes":9,
            "covered_bytes":39640,"uncovered_bytes":4520,
            "reference_payload_bytes_embedded":0,"byte_mismatches":0,
            "first_uncovered_file":0x18620,"first_uncovered_data_file":0x18620,
            "reference_decoded_absolute_metadata_bytes":96,
            "cross_boundary_owned_instruction_suffix_bytes":22,
            "independent_seam_bitfield_bytes":len(seam_bytes),
            "gnu_assembled_bytes_this_run":gnu_bytes,
            "gnu_checks_explicitly_skipped":args.source_only,
            "independently_generated_data_and_alignment_bytes":sum(map(len,independent.values())),
            "synthetic_isa_variants_this_run":0 if args.source_only else 161,
            "synthetic_isa_bytes_this_run":matrix_bytes,
            "rust_version":run(args.rustc,"--version"),
            "assembler_version":None if args.source_only else run(args.assembler,"--version").splitlines()[0],
            "fragments":digest_list,"unresolved":manifest["unresolved"],
            "behavior_verified_on_hardware":False,"complete_firmware":False,
        }
    rendered=json.dumps(report,indent=2)+"\n"
    if args.report:
        args.report.parent.mkdir(parents=True,exist_ok=True)
        args.report.write_text(rendered)
    print(rendered,end="")


if __name__=="__main__":
    main()
