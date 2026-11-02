#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Prove deterministic Nabu source output from two isolated, offline builds.

The report contains hashes and explicit holes, never borrowed firmware bytes.
An optional directory of the six pinned original MBNs enables byte comparisons
and original embedded-chain checks. Only A640 can then be reassembled in
temporary storage, using precisely its original authentication field. No new
signature or device trust is claimed. No incomplete full image is written.

Run from a committed checkout with a clean tracked tree. The two builds use
separate copies of the committed Git tree, empty Cargo homes, separate target
directories, fixed SOURCE_DATE_EPOCH and remapped source paths. Untracked files
are never build inputs. The manifest is canonical JSON without wall-clock
timestamps or machine-local paths. Host executable hashes are reported
separately from the actual firmware-source output comparison.

This is a comparison of two builds with recorded tools, not a hermetic
toolchain claim: the host linker, loader, operating system and compiler
installation are not packaged or cryptographically pinned by this tool.
"""

import argparse
import hashlib
import importlib.util
import io
import json
import os
from pathlib import Path
import platform
import shutil
import subprocess
import sys
import tarfile
import tempfile


ROOT = Path(__file__).resolve().parents[1]
IMAGES = ("a640_zap.mbn", "adsp.mbn", "cdsp.mbn", "modem.mbn", "venus.mbn", "wlanmdsp.mbn")
FILE_BYTES = {
    "a640_zap.mbn": 14_256,
    "adsp.mbn": 14_430_684,
    "cdsp.mbn": 9_455_252,
    "modem.mbn": 5_101_612,
    "venus.mbn": 1_159_200,
    "wlanmdsp.mbn": 4_331_136,
}
AUTH_RANGE = (0x1138, 0x2A38)
EMBEDDED_ROOT_SHA256 = "c924a35f39ce1cdd1b8d5a9f3b8e3c51317930431d7a9dd5a55028cf6965fe65"
BUILD_COMMAND = ["cargo", "build", "--workspace", "--release", "--offline", "--locked"]
EXAMPLE_BUILD_COMMAND = ["cargo", "build", "-p", "firmware-tools",
                         "--release", "--offline", "--locked",
                         "--example", "mbn_hash_readiness",
                         "--example", "mbn_verify_pinned_a640"]

# Kept in this standalone tool to avoid editing the shared CLI. It links only
# the just-built library, and exports generated slices through the checked API.
# No function reads a reference or fills an ImagePlan hole.
PROBE_SOURCE = r'''
use firmware_tools::{devices::xiaomi_nabu::elf_specs::KNOWN_ELFS,
    image_plan::RegionOrigin, mbn_source_slices::{a640_source_plan,MbnSourceSlices}};
use std::{env, error::Error, fs, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 { return Err("one output directory is required".into()); }
    let output = Path::new(&args[1]);
    fs::create_dir(output)?;
    let sizes = [("a640_zap.mbn",14256usize),("adsp.mbn",14430684),
        ("cdsp.mbn",9455252),("modem.mbn",5101612),
        ("venus.mbn",1159200),("wlanmdsp.mbn",4331136)];
    let mut images = Vec::new();
    for (name,total) in sizes {
        let spec = KNOWN_ELFS.iter().find(|s|s.name==name).ok_or("missing ELF profile")?;
        let builder = if name=="a640_zap.mbn" { a640_source_plan()? }
            else { MbnSourceSlices::new(spec,total)? };
        let plan = builder.plan();
        let coverage = plan.coverage();
        let geometry = builder.report();
        let provenance = geometry.regions.iter().map(|r|format!(
            "{{\"start\":{},\"end\":{},\"kind\":\"{}\"}}",
            r.file_range.start,r.file_range.end,r.kind.as_str()))
            .collect::<Vec<_>>().join(",");
        let permissions = &geometry.load_permissions;
        let unassigned = geometry.unassigned_file_ranges.iter()
            .map(|r|format!("[{},{}]",r.start,r.end)).collect::<Vec<_>>().join(",");
        let strict_geometry = format!(concat!(
            "{{\"executable_not_writable\":{},\"writable_not_executable\":{},",
            "\"executable_and_writable\":{},\"neither_executable_nor_writable\":{},",
            "\"unassigned_file_ranges\":[{}],\"memory_only_bytes\":{},",
            "\"authentication_file_range\":[{},{}],\"hash_file_range\":[{},{}]}}"),
            permissions.executable_not_writable,permissions.writable_not_executable,
            permissions.executable_and_writable,permissions.neither_executable_nor_writable,
            unassigned,geometry.memory_only_bytes,geometry.authentication_file_range.start,
            geometry.authentication_file_range.end,geometry.hash_file_range.start,geometry.hash_file_range.end);
        if coverage.externally_supplied_bytes != 0 {
            return Err("source probe encountered external bytes".into());
        }
        if plan.build_independently_generated().is_ok() || plan.build_complete().is_ok() {
            return Err("known incomplete source plan unexpectedly emitted a full image".into());
        }
        let directory = output.join(name);
        fs::create_dir(&directory)?;
        let mut regions = Vec::new();
        let mut ordered: Vec<_> = plan.regions().iter().collect();
        ordered.sort_by_key(|r|r.offset);
        for region in ordered {
            if region.origin != RegionOrigin::Generated {
                return Err("external source region".into());
            }
            let end = region.offset.checked_add(region.len).ok_or("region overflow")?;
            let bytes = plan.generated_slice(region.offset..end)?;
            let file = format!("region-{:08x}-{:08x}.bin",region.offset,end);
            fs::write(directory.join(&file),&bytes)?;
            regions.push(format!("{{\"start\":{},\"end\":{},\"file\":\"{}\"}}",
                region.offset,end,file));
        }
        let holes = coverage.holes.iter().map(|r|format!("[{},{}]",r.start,r.end))
            .collect::<Vec<_>>().join(",");
        let segments = spec.headers.iter().enumerate().map(|(i,h)|format!(
            "{{\"index\":{},\"type\":{},\"start\":{},\"file_bytes\":{},\"memory_bytes\":{},\"virtual_address\":{},\"physical_address\":{},\"flags\":{}}}",
            i,h.kind,h.offset,h.filesz,h.memsz,h.vaddr,h.paddr,h.flags))
            .collect::<Vec<_>>().join(",");
        images.push(format!(concat!("{{\"image\":\"{}\",\"file_bytes\":{},",
            "\"generated_bytes\":{},\"missing_bytes\":{},\"source_complete\":false,",
            "\"full_image_emission_refused\":true,\"holes\":[{}],",
            "\"regions\":[{}],\"segments\":[{}],\"strict_geometry\":{},\"provenance_regions\":[{}]}}"),name,total,
            coverage.generated_bytes,coverage.missing_bytes,holes,regions.join(","),segments,strict_geometry,provenance));
    }
    println!("[{}]",images.join(","));
    Ok(())
}
'''


class ReproducibilityError(ValueError):
    """An unproven or inconsistent build is never published as a success."""


def canonical_json(value):
    return (json.dumps(value, sort_keys=True, separators=(",", ":"),
                       ensure_ascii=True, allow_nan=False) + "\n").encode("ascii")


def sha256(data):
    return hashlib.sha256(data).hexdigest()


def digest_file(path):
    digest = hashlib.sha256()
    with Path(path).open("rb") as source:
        for block in iter(lambda: source.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def run(command, *, cwd=None, env=None, binary=False):
    result = subprocess.run(command, cwd=cwd, env=env, capture_output=True,
                            text=not binary, check=False)
    if result.returncode:
        reason = result.stderr or result.stdout
        if isinstance(reason, bytes):
            reason = reason.decode("utf-8", errors="replace")
        raise ReproducibilityError(f"{Path(str(command[0])).name} failed: {reason.strip()[:1200]}")
    return result.stdout


def source_identity(root):
    root = Path(root)
    status = run(["git", "status", "--porcelain", "--untracked-files=no"], cwd=root)
    if status.strip():
        raise ReproducibilityError("tracked source tree is dirty; commit the tested source first")
    commit = run(["git", "rev-parse", "HEAD"], cwd=root).strip()
    tree = run(["git", "rev-parse", "HEAD^{tree}"], cwd=root).strip()
    epoch = int(run(["git", "show", "-s", "--format=%ct", "HEAD"], cwd=root).strip())
    # Prevent an untracked/modified harness from attributing its behavior to HEAD.
    committed = run(["git", "show", "HEAD:tools/mbn-reproducibility.py"], cwd=root, binary=True)
    if committed != (root / "tools" / "mbn-reproducibility.py").read_bytes():
        raise ReproducibilityError("running reproducibility harness differs from the committed source")
    return {"commit": commit, "tree": tree, "commit_epoch": epoch,
            "harness_sha256": sha256(committed), "tracked_tree_clean": True,
            "untracked_files_used": False}


def extract_snapshot(archive, destination):
    """Extract only regular Git files/directories; never follow archive links."""
    destination = Path(destination)
    destination.mkdir()
    with tarfile.open(fileobj=io.BytesIO(archive), mode="r:") as source:
        for member in source.getmembers():
            name = Path(member.name)
            if name.is_absolute() or ".." in name.parts or not name.parts:
                raise ReproducibilityError("unsafe source archive path")
            target = destination / name
            if member.isdir():
                target.mkdir(parents=True, exist_ok=True)
            elif member.isfile():
                target.parent.mkdir(parents=True, exist_ok=True)
                stream = source.extractfile(member)
                if stream is None:
                    raise ReproducibilityError("source archive member has no contents")
                with target.open("xb") as output:
                    shutil.copyfileobj(stream, output)
                target.chmod(member.mode & 0o777)
            else:
                raise ReproducibilityError("source snapshot contains unsupported links or special files")


def build_environment(folder, epoch, rustc):
    env = os.environ.copy()
    # Clear inherited profile, target/linker, wrapper and flag overrides. In
    # particular a CARGO_PROFILE_RELEASE_* variable must not silently change
    # the advertised release profile. The compiler/toolchain still comes from
    # the host; this normalization does not make the toolchain hermetic.
    compiler_variables = {"CC", "CXX", "LD", "AR", "RANLIB", "NM", "STRIP", "OBJCOPY",
                          "CFLAGS", "CPPFLAGS", "CXXFLAGS", "LDFLAGS"}
    for key in list(env):
        compiler_override = any(
            key == variable or key in ("HOST_" + variable, "TARGET_" + variable)
            or (variable != "LD" and key.startswith(variable + "_"))
            for variable in compiler_variables)
        if (key.startswith("CARGO_") or key.startswith("RUSTC_") or
                key in ("RUSTFLAGS", "RUSTDOCFLAGS", "RUSTC", "RUSTDOC") or
                key.startswith("PKG_CONFIG_") or compiler_override or
                key in ("COMPILER_PATH", "GCC_EXEC_PREFIX", "LIBRARY_PATH")):
            env.pop(key)
    env.update({
        "CARGO_HOME": str(folder / "cargo-home"),
        "CARGO_TARGET_DIR": str(folder / "target"),
        "CARGO_INCREMENTAL": "0",
        "RUSTC": str(rustc),
        "RUSTFLAGS": f"--remap-path-prefix={folder}=/mbn-reproducibility",
        "SOURCE_DATE_EPOCH": str(epoch), "TZ": "UTC", "LC_ALL": "C",
    })
    return env


def build_once(folder, archive, epoch, cargo, rustc):
    folder = Path(folder)
    folder.mkdir()
    checkout = folder / "source"
    extract_snapshot(archive, checkout)
    env = build_environment(folder, epoch, rustc)
    run([str(cargo), *BUILD_COMMAND[1:]], cwd=checkout, env=env)
    executable = folder / "target" / "release" / "firmware-tools"
    library = folder / "target" / "release" / "libfirmware_tools.rlib"
    if not executable.is_file() or not library.is_file():
        raise ReproducibilityError("offline build did not produce its executable and library")
    # Build the actual source-hash / original-identity helpers from the same
    # committed snapshot. Otherwise only the legacy standalone probe and CLI
    # would be in the two-build proof, silently excluding newly added gates.
    run([str(cargo), *EXAMPLE_BUILD_COMMAND[1:]], cwd=checkout, env=env)
    examples = folder / "target" / "release" / "examples"
    hash_readiness = examples / "mbn_hash_readiness"
    pinned_a640 = examples / "mbn_verify_pinned_a640"
    if not hash_readiness.is_file() or not pinned_a640.is_file():
        raise ReproducibilityError("new source-readiness and pinned-original verifier examples were not built")
    # Do not read original firmware in this stage: the source-readiness tool
    # emits only source-derived hashes and holes. Strict reference comparisons
    # happen later and require explicitly supplied pinned firmware files.
    readiness_bytes = run([str(hash_readiness)], cwd=checkout, env=env)
    try:
        readiness_rows = [json.loads(line) for line in readiness_bytes.splitlines() if line.strip()]
    except json.JSONDecodeError as exc:
        raise ReproducibilityError("source-hash readiness tool emitted invalid JSONL") from exc
    validate_readiness_rows(readiness_rows)
    probe_source = folder / "source-probe.rs"
    probe_source.write_text(PROBE_SOURCE, encoding="utf-8")
    probe = folder / "source-probe"
    run([str(rustc), "--edition=2021", str(probe_source),
         "--crate-name", "mbn_source_probe", "--extern", f"firmware_tools={library}",
         "-L", f"dependency={library.parent / 'deps'}", "-C", "opt-level=2",
         "--remap-path-prefix", f"{folder}=/mbn-reproducibility", "-o", str(probe)],
        cwd=checkout, env=env)
    output = folder / "source-output"
    report = validate_probe_output(run([str(probe), str(output)], cwd=checkout, env=env), output)
    return {"checkout": checkout, "executable": executable, "source_output": report,
            "hash_readiness": readiness_rows, "pinned_a640_example": pinned_a640,
            "host_artifacts": {"firmware-tools": digest_file(executable),
                               "source-probe": digest_file(probe),
                               "mbn-hash-readiness": digest_file(hash_readiness),
                               "mbn-pinned-a640": digest_file(pinned_a640)}}


def validate_readiness_rows(rows):
    """Check deterministic source-hash JSONL structure without OEM references.

    SHA-384 values here are independently generated claims, *not* reference
    equality. When --references is supplied, verify_originals cross-checks
    each claimed ready hash against the pinned original ELF bytes.
    """
    if not isinstance(rows, list) or len(rows) != len(IMAGES):
        raise ReproducibilityError("source-hash readiness must list all six Nabu images")
    for expected_name, row in zip(IMAGES, rows):
        if not isinstance(row, dict) or row.get("image") != expected_name:
            raise ReproducibilityError("source-hash readiness images are missing or out of order")
        total = integer(row.get("file_bytes"), "source readiness file size")
        if total != FILE_BYTES[expected_name]:
            raise ReproducibilityError("source readiness file length differs from pinned MBN policy")
        segments = row.get("segments")
        if not isinstance(segments, list) or not segments or len(segments) > 256:
            raise ReproducibilityError("source readiness is missing bounded program-header list")
        ready = pending = skipped = ready_bytes = 0
        for index, segment in enumerate(segments):
            if not isinstance(segment, dict) or integer(segment.get("index"), "source segment index") != index:
                raise ReproducibilityError("source readiness contains duplicate/disordered segment indices")
            integer(segment.get("kind"), "source segment type")
            pair = segment.get("file_range")
            if not isinstance(pair, list) or len(pair) != 2:
                raise ReproducibilityError("source readiness has malformed file span")
            start, end = [integer(v, "source segment offset") for v in pair]
            if start > end or (start != end and end > total):
                raise ReproducibilityError("source readiness segment extends outside original file")
            status = segment.get("status")
            if status == "ready":
                digest = segment.get("sha384")
                if (start == end or not isinstance(digest, str) or len(digest) != 96 or
                        any(c not in "0123456789abcdef" for c in digest)):
                    raise ReproducibilityError("source readiness has malformed SHA-384 digest")
                ready += 1
                ready_bytes += end - start
            elif status == "pending":
                if start == end:
                    raise ReproducibilityError("source readiness cannot have pending empty segment")
                hole = segment.get("first_missing")
                if not isinstance(hole, list) or len(hole) != 2:
                    raise ReproducibilityError("source readiness missing first unimplemented range")
                low, high = [integer(v, "source missing byte") for v in hole]
                missing = integer(segment.get("missing_bytes"), "source missing bytes")
                if not (start <= low < high <= end) or not high - low <= missing <= end - start:
                    raise ReproducibilityError("source readiness first hole/count is inconsistent")
                pending += 1
            elif status == "placeholder":
                if "sha384" in segment or "first_missing" in segment:
                    raise ReproducibilityError("source readiness placeholder cannot claim generated bytes")
                skipped += 1
            else:
                raise ReproducibilityError("source readiness has unknown entry status")
        for key, expected in (("ready_entries", ready), ("pending_entries", pending),
                              ("zero_placeholders", skipped), ("ready_hashed_bytes", ready_bytes)):
            if integer(row.get(key), key) != expected:
                raise ReproducibilityError(f"source readiness has inconsistent {key}")
        if row.get("all_input_hashes_ready") is not (pending == 0):
            raise ReproducibilityError("source readiness falsely claims all input hashes complete")
    return rows


def integer(value, label, minimum=0):
    if type(value) is not int or value < minimum:
        raise ReproducibilityError(f"invalid {label}")
    return value


def checked_ranges(ranges, total, label):
    if not isinstance(ranges, list):
        raise ReproducibilityError(f"invalid {label} interval list")
    checked = []
    for entry in ranges:
        if not isinstance(entry, list) or len(entry) != 2:
            raise ReproducibilityError(f"invalid {label} interval")
        start, end = (integer(v, label) for v in entry)
        if start >= end or end > total:
            raise ReproducibilityError(f"invalid {label} extent")
        if checked and start < checked[-1][1]:
            raise ReproducibilityError(f"overlapping or unordered {label} intervals")
        checked.append([start, end])
    return checked


def complement(ranges, total):
    holes, end = [], 0
    for start, stop in ranges:
        if start > end:
            holes.append([end, start])
        end = stop
    if end < total:
        holes.append([end, total])
    return holes


def union_length(ranges):
    total, end = 0, 0
    for start, stop in sorted(ranges):
        total += max(0, stop - max(start, end))
        end = max(end, stop)
    return total


def segment_inventory(segments, total):
    file_spans, executable, writable, readonly, bss, loads = [], [], [], [], 0, []
    for index, segment in enumerate(segments):
        if not isinstance(segment, dict):
            raise ReproducibilityError("invalid segment record")
        for key in ("index", "type", "start", "file_bytes", "memory_bytes",
                    "virtual_address", "physical_address", "flags"):
            integer(segment.get(key), f"segment {key}")
        if segment["index"] != index:
            raise ReproducibilityError("segment indices are not canonical")
        start, size = segment["start"], segment["file_bytes"]
        if size and start + size > total:
            raise ReproducibilityError("file-backed segment exceeds pinned image length")
        if size:
            file_spans.append([start, start + size])
        if segment["type"] == 1:
            if size > segment["memory_bytes"]:
                raise ReproducibilityError("PT_LOAD filesz exceeds memsz")
            bss += segment["memory_bytes"] - size
            if size:
                loads.append((start, start + size, segment["flags"]))
                if segment["flags"] & 1:
                    executable.append([start, start + size])
                if segment["flags"] & 2:
                    writable.append([start, start + size])
                if not segment["flags"] & 3:
                    readonly.append([start, start + size])
    boundaries = sorted({value for start, end, _ in loads for value in (start, end)})
    disjoint = {"executable_not_writable": 0, "writable_not_executable": 0,
                "executable_and_writable": 0, "neither_executable_nor_writable": 0}
    for start, end in zip(boundaries, boundaries[1:]):
        active = [flags for low, high, flags in loads if low <= start < high]
        if not active:
            continue
        executable_here, writable_here = any(f & 1 for f in active), any(f & 2 for f in active)
        key = ("executable_and_writable" if executable_here and writable_here else
               "executable_not_writable" if executable_here else
               "writable_not_executable" if writable_here else "neither_executable_nor_writable")
        disjoint[key] += end - start
    merged = []
    for start, end in sorted(file_spans):
        if merged and start <= merged[-1][1]:
            merged[-1][1] = max(merged[-1][1], end)
        else:
            merged.append([start, end])
    return {"file_backed_unique_bytes": union_length(file_spans),
            "executable_file_bytes": union_length(executable),
            "writable_file_bytes": union_length(writable),
            "readonly_nonexecuting_file_bytes": union_length(readonly),
            "memory_only_pt_load_bytes_sum": bss,
            "permission_counts_may_overlap": True,
            "disjoint_load_permission_bytes": disjoint,
            "unassigned_file_ranges": complement(merged, total),
            "bss_is_file_content": False}


def source_categories(name, provenance, source_spans, total):
    """Validate explicit typed provenance separately from segment permissions.

    Every category is a source author's claim, not an inference from PF_X or
    byte values. In particular the A640 QRISC header and terminal shader NOP
    padding retain their specific metadata/layout classifications.
    """
    keys = {"source_code": "code", "source_data": "data",
            "source_metadata": "metadata_and_hashes", "explicit_layout_fill": "explicit_layout_padding"}
    counts = {key: 0 for key in keys.values()}
    counts["external_authentication"] = 0
    if not isinstance(provenance, list) or not all(isinstance(r, dict) for r in provenance):
        raise ReproducibilityError("typed provenance-region ledger is missing")
    ranges = checked_ranges([[r.get("start"), r.get("end")] for r in provenance], total, "provenance")
    if complement(ranges, total) != complement(source_spans, total):
        raise ReproducibilityError("typed provenance and emitted source regions cover different bytes")
    for region, (start, end) in zip(provenance, ranges):
        if region.get("kind") not in keys:
            raise ReproducibilityError(f"{name}: unrecognized or external source provenance")
        counts[keys[region["kind"]]] += end - start
    return counts


def validate_probe_output(output, directory):
    try:
        records = json.loads(output)
    except (TypeError, json.JSONDecodeError) as exc:
        raise ReproducibilityError("source probe did not return valid JSON") from exc
    if (not isinstance(records, list) or not all(isinstance(r, dict) for r in records)
            or [r.get("image") for r in records] != list(IMAGES)):
        raise ReproducibilityError("source probe must report exactly the six known images in order")
    reports = []
    for record in records:
        name = record["image"]
        total = integer(record.get("file_bytes"), "file size", 1)
        if total != FILE_BYTES[name] or record.get("source_complete") is not False:
            raise ReproducibilityError(f"{name}: incorrect pinned size or source-completeness claim")
        if record.get("full_image_emission_refused") is not True:
            raise ReproducibilityError(f"{name}: incomplete full-image emission was not refused")
        regions = record.get("regions")
        segments = record.get("segments")
        if (not isinstance(regions, list) or not regions or not all(isinstance(r, dict) for r in regions)
                or not isinstance(segments, list) or not segments):
            raise ReproducibilityError("source probe is missing regions or ELF geometry")
        spans = checked_ranges([[r.get("start"), r.get("end")] for r in regions], total, "generated source")
        holes = checked_ranges(record.get("holes", []), total, "hole")
        if holes != complement(spans, total):
            raise ReproducibilityError(f"{name}: source holes contradict generated spans")
        generated = sum(end - start for start, end in spans)
        if record.get("generated_bytes") != generated or record.get("missing_bytes") != total - generated:
            raise ReproducibilityError(f"{name}: inconsistent source byte accounting")
        if generated != (7856 if name == "a640_zap.mbn" else 4264):
            raise ReproducibilityError(f"{name}: changed source coverage requires reviewed harness update")
        joined = hashlib.sha256()
        source_regions = []
        for region, (start, end) in zip(regions, spans):
            filename = f"region-{start:08x}-{end:08x}.bin"
            if region.get("file") != filename:
                raise ReproducibilityError("source probe returned a noncanonical artifact name")
            path = Path(directory) / name / filename
            if path.is_symlink() or not path.is_file():
                raise ReproducibilityError("generated source artifact missing or symlinked")
            data = path.read_bytes()
            if len(data) != end - start:
                raise ReproducibilityError("generated source artifact has the wrong byte length")
            joined.update(data)
            source_regions.append({"start": start, "end": end, "sha256": sha256(data)})
        provenance = record.get("provenance_regions")
        categories = source_categories(name, provenance, spans, total)
        if sum(categories.values()) != generated:
            raise ReproducibilityError("source provenance categories do not partition coverage")
        inventory = segment_inventory(segments, total)
        strict = record.get("strict_geometry")
        if not isinstance(strict, dict):
            raise ReproducibilityError("strict source-slice geometry report missing")
        if (any(strict.get(key) != amount for key, amount in inventory["disjoint_load_permission_bytes"].items()) or
                strict.get("unassigned_file_ranges") != inventory["unassigned_file_ranges"] or
                strict.get("memory_only_bytes") != inventory["memory_only_pt_load_bytes_sum"]):
            raise ReproducibilityError("strict Rust and independent Python segment geometry disagree")
        checked_ranges([strict.get("authentication_file_range")], total, "OEM authentication")
        checked_ranges([strict.get("hash_file_range")], total, "signed hash")
        reports.append({"image": name, "file_bytes": total,
            "generated_bytes": generated, "missing_bytes": total - generated,
            "source_complete": False, "full_image_emission_refused": True,
            "holes": holes, "source_regions": source_regions,
            "ordered_generated_bytes_sha256": joined.hexdigest(),
            "generated_byte_categories": categories,
            "provenance_regions": provenance,
            "executable_permissions_are_instruction_count": False,
            "semantics_fully_validated": False,
            "semantic_qualification": ("A640 legacy IR3 barrier encoding remains provisional; byte identity and typed code classification do not establish complete semantics."
                if name == "a640_zap.mbn" else "Only the generic ELF/header/policy source is covered by this report; executable source integration remains separate."),
            "segment_inventory": inventory, "strict_geometry": strict, "segments": segments})
    return reports


def compare_builds(first, second):
    if first["source_output"] != second["source_output"]:
        raise ReproducibilityError("independent builds disagree on generated firmware bytes or geometry")
    if "hash_readiness" not in first or "hash_readiness" not in second:
        raise ReproducibilityError("new per-segment source-hash readiness is missing")
    if first["hash_readiness"] != second["hash_readiness"]:
        raise ReproducibilityError("independent builds disagree on generated per-segment SHA-384 readiness")
    validate_readiness_rows(first["hash_readiness"])
    return {"firmware_source_outputs_byte_identical": True, "build_count": 2,
            "source_output_fingerprint_sha256": sha256(canonical_json(first["source_output"])),
            "per_segment_sha384_readiness_identical": True,
            "per_segment_sha384_readiness_fingerprint_sha256": sha256(canonical_json(first["hash_readiness"])),
            "host_tools_byte_identical": first["host_artifacts"] == second["host_artifacts"],
            "host_tool_sha256s": [first["host_artifacts"], second["host_artifacts"]]}


def load_module(path, name):
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise ReproducibilityError(f"cannot load {Path(path).name}")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def pinned_references(directory, revisions):
    """Read once and hash the exact bytes subsequently passed to verifiers."""
    directory = Path(directory)
    values = {}
    for name in IMAGES:
        path = directory / name
        if not path.is_file():
            raise ReproducibilityError(f"missing original reference: {name}")
        data = path.read_bytes()
        if len(data) != FILE_BYTES[name] or sha256(data) != revisions[name]:
            raise ReproducibilityError(f"wrong pinned original reference revision: {name}")
        values[name] = data
    return values


def compare_source_with_reference(report, original):
    """Bind every exported source-region digest to the exact pinned bytes.

    Comparing two strict builders or comparing their counts to the legacy
    audit is insufficient: both could consistently emit the same wrong bytes.
    This check is deliberately independent of the legacy audit and assembler.
    """
    if len(original) != report["file_bytes"]:
        raise ReproducibilityError("strict source comparison has the wrong original length")
    compared = 0
    for region in report["source_regions"]:
        start, end = region["start"], region["end"]
        if start < 0 or end <= start or end > len(original):
            raise ReproducibilityError("strict source comparison range exceeds original bytes")
        if sha256(original[start:end]) != region["sha256"]:
            raise ReproducibilityError(
                f"{report['image']}: strict generated source differs from pinned original at 0x{start:x}..0x{end:x}")
        compared += end - start
    if compared != report["generated_bytes"]:
        raise ReproducibilityError("strict source reference comparison did not cover every generated byte")
    return {"compared_generated_bytes": compared,
            "compared_source_regions": len(report["source_regions"]),
            "all_source_region_sha256s_match_pinned_reference": True,
            "generated_byte_mismatches": 0}


def verify_originals(directory, builds, temporary, epoch):
    checkout = builds[0]["checkout"]
    auditor = load_module(checkout / "tools/verify" / "audit-nabu-mbn-sources.py", "mbn_repro_auditor")
    verifier = load_module(checkout / "tools/verify" / "verify-mbn-signatures.py", "mbn_repro_verifier")
    revisions = auditor.pinned_sha256s()
    originals = pinned_references(directory, revisions)
    private = Path(temporary) / "originals"
    private.mkdir(mode=0o700)
    for name, data in originals.items():
        (private / name).write_bytes(data)
    reports = [auditor.audit(private, build["executable"]) for build in builds]
    if reports[0] != reports[1]:
        raise ReproducibilityError("independent reference-coverage audits disagree")
    strict_comparisons = []
    for build in builds:
        strict_comparisons.append({report["image"]: compare_source_with_reference(
            report, originals[report["image"]]) for report in build["source_output"]})
    readiness_checker = load_module(checkout / "tools/verify" / "verify-mbn-hash-readiness.py",
                                    "mbn_repro_readiness_checker")
    source_hash_comparisons = [readiness_checker.verify_reports(
        build["hash_readiness"], private, revisions) for build in builds]
    if source_hash_comparisons[0] != source_hash_comparisons[1]:
        raise ReproducibilityError("independent source-hash reports disagree on the pinned originals")
    # This independent checked Rust command has a SHA-384 pin built into its
    # source; no source or authentication bytes are written to the manifest.
    pinned_a640_guards = []
    for build in builds:
        raw = run([str(build["pinned_a640_example"]), str(private / "a640_zap.mbn")])
        try:
            receipt = json.loads(raw)
        except json.JSONDecodeError as exc:
            raise ReproducibilityError("pinned A640 exact-original helper did not emit JSON") from exc
        if (receipt.get("image") != "a640_zap.mbn" or receipt.get("file_bytes") != 14_256 or
                receipt.get("generated_bytes") != 7_856 or
                receipt.get("borrowed_oem_authentication_bytes") != 6_400 or
                receipt.get("external_auth_range") != list(AUTH_RANGE) or
                receipt.get("byte_mismatches") != 0 or
                receipt.get("original_sha384_pinned") is not True or
                receipt.get("segment_hashes_verified") is not True or
                receipt.get("hardware_trust_verified") is not False):
            raise ReproducibilityError("pinned A640 helper returned incomplete or misleading original-equivalence receipt")
        pinned_a640_guards.append(receipt)
    if pinned_a640_guards[0] != pinned_a640_guards[1]:
        raise ReproducibilityError("pinned A640 helper receipts differ across clean builds")
    verifications = []
    for report in reports[0]:
        name = report["image"]
        source = next(r for r in builds[0]["source_output"] if r["image"] == name)
        if (report["independently_generated_file_bytes"] != source["generated_bytes"] or
                report["missing_file_bytes"] != source["missing_bytes"]):
            raise ReproducibilityError("reference audit disagrees with source-only export")
        result = verifier.verify_image(originals[name], expected_root_sha256=EMBEDDED_ROOT_SHA256,
                                       verification_time=epoch)
        if (result.get("oem_signature_verified") is not True or
                result.get("certificate_chain_verified_to_embedded_root") is not True or
                result.get("device_hardware_trust_verified") is not False or
                result.get("embedded_root_sha256") != EMBEDDED_ROOT_SHA256):
            raise ReproducibilityError("original signature verifier returned inconsistent trust claims")
        verifications.append({"image": name, "reference_sha256": revisions[name],
                              "generated_byte_mismatches": 0,
                              "strict_source_comparisons": [checks[name] for checks in strict_comparisons],
                              "coverage": report, "original_authentication": result})
    # Only the original A640 authentication interval enters the Rust builder.
    auth = originals["a640_zap.mbn"][AUTH_RANGE[0]:AUTH_RANGE[1]]
    auth_path = private / "a640-original-authentication"
    auth_path.write_bytes(auth)
    rebuilt = []
    for index, build in enumerate(builds):
        candidate = private / f"a640-run-{index}.mbn"
        run([str(build["executable"]), "assemble-a640-with-external-auth",
             str(auth_path), str(candidate)])
        data = candidate.read_bytes()
        if data != originals["a640_zap.mbn"] or sha256(data) != revisions["a640_zap.mbn"]:
            raise ReproducibilityError("A640 source plus original authentication does not match every pinned byte")
        rebuilt.append(sha256(data))
    return {"verification_scope": "six_pinned_original_references",
            "certificate_validity_epoch": epoch, "images": verifications,
            "per_segment_hash_readiness": source_hash_comparisons[0],
            "pinned_original_a640_guard": pinned_a640_guards[0],
            "a640_reassembly": {"two_outputs_byte_identical": rebuilt[0] == rebuilt[1],
                "sha256": rebuilt[0], "file_bytes": FILE_BYTES["a640_zap.mbn"],
                "generated_bytes": 7856, "borrowed_original_authentication_bytes": 6400,
                "borrowed_original_authentication_range": list(AUTH_RANGE),
                "borrowed_authentication_sha256": sha256(auth), "byte_mismatches": 0,
                "independently_signed": False, "hardware_trust_verified": False,
                "full_image_published": False}}


def publish_manifest(path, data):
    """Atomic no-clobber publication, including a destination-creation race."""
    path = Path(path)
    if path.exists() or path.is_symlink():
        raise ReproducibilityError("refusing to overwrite an existing manifest")
    if not path.parent.is_dir():
        raise ReproducibilityError("manifest destination directory does not exist")
    with tempfile.NamedTemporaryFile(prefix=".mbn-manifest-", dir=path.parent, delete=False) as output:
        temporary = Path(output.name)
        try:
            output.write(data)
            output.flush()
            os.fsync(output.fileno())
            os.link(temporary, path)
        except FileExistsError as exc:
            raise ReproducibilityError("refusing to overwrite a manifest created during publication") from exc
        finally:
            temporary.unlink(missing_ok=True)


def reproduce(root, references=None, require_source_complete=False):
    root = Path(root).resolve()
    identity = source_identity(root)
    cargo, rustc = shutil.which("cargo"), shutil.which("rustc")
    if cargo is None or rustc is None:
        raise ReproducibilityError("cargo and rustc are required on PATH")
    tools = {"cargo": run([cargo, "--version"]).strip(),
             "rustc": run([rustc, "--version", "--verbose"]).strip(),
             "python": platform.python_version()}
    if references is not None:
        tools["openssl"] = run(["openssl", "version"]).strip()
    archive = run(["git", "archive", "--format=tar", identity["commit"]], cwd=root, binary=True)
    with tempfile.TemporaryDirectory(prefix="mbn-reproducibility-") as temporary:
        builds = [build_once(Path(temporary) / f"run-{index}", archive,
                             identity["commit_epoch"], cargo, rustc) for index in range(2)]
        comparison = compare_builds(*builds)
        if require_source_complete:
            incomplete = [r["image"] for r in builds[0]["source_output"] if not r["source_complete"]]
            if incomplete:
                raise ReproducibilityError("source-complete image gate failed: " + ", ".join(incomplete))
        original_audit = None if references is None else verify_originals(
            references, builds, temporary, identity["commit_epoch"])
        if source_identity(root) != identity:
            raise ReproducibilityError("source revision changed while the proof was running")
        return {"schema": "nabu-mbn-reproducibility-v1", "source": identity,
                "tools": tools, "build": {"command": BUILD_COMMAND,
                    "example_command": EXAMPLE_BUILD_COMMAND,
                    "offline": True, "locked": True, "isolated_cargo_homes": True,
                    "isolated_target_directories": True, "source_path_remap": "/mbn-reproducibility",
                    "source_date_epoch": identity["commit_epoch"], "cargo_incremental": False,
                    "inherited_cargo_profile_target_and_linker_overrides_cleared": True,
                    "inherited_compiler_flags_and_wrappers_cleared": True,
                    "hermetic_toolchain_verified": False,
                    "toolchain_scope": "Recorded compiler and Cargo versions; host linker, loader, system libraries and compiler installation are not packaged or digest-pinned.",
                    "probe_source_sha256": sha256(PROBE_SOURCE.encode("utf-8"))},
                "reproducibility": comparison, "images": builds[0]["source_output"],
                "per_segment_source_hash_readiness": builds[0]["hash_readiness"],
                "original_reference_verification": original_audit,
                "new_oem_signature_created": False, "device_hardware_trust_verified": False,
                "full_images_published": [], "source_complete_images": []}


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--references", type=Path, help="private directory containing all six pinned original MBNs")
    parser.add_argument("--output", type=Path, help="new manifest file; defaults to stdout; never overwrites")
    parser.add_argument("--require-source-complete", action="store_true",
                        help="fail instead of reporting any incomplete source image (currently always fails)")
    args = parser.parse_args(argv)
    try:
        if args.output is not None and (args.output.exists() or args.output.is_symlink()):
            raise ReproducibilityError("refusing to overwrite an existing manifest")
        report = reproduce(ROOT, args.references, args.require_source_complete)
        encoded = canonical_json(report)
        if args.output is None:
            sys.stdout.write(encoded.decode("ascii"))
        else:
            publish_manifest(args.output, encoded)
    except (OSError, ValueError, subprocess.SubprocessError) as exc:
        parser.exit(1, f"FAIL: {exc}\n")


if __name__ == "__main__":
    main()
