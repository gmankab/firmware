#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Pinned source-coverage ledger, strict byte comparison and deterministic status.

A reported region must not silently become a verified region. Exact original
firmware can be used only as an external comparison input, never as generated
source or an implied new OEM signature.
"""
from __future__ import annotations

import hashlib
from pathlib import Path
import tomllib
from typing import Any

PROJECT_ROOT = Path(__file__).resolve().parents[2]
DEFAULT_MANIFEST = PROJECT_ROOT / "reference/xiaomi-nabu/coverage.toml"


class CoverageError(ValueError):
    """Invalid manifest, reference revision or unverified build operation."""


def holes(regions: list[dict[str, Any]], start: int, end: int) -> list[dict[str, int]]:
    current = start
    missing = []
    for region in sorted(regions, key=lambda item: item["start"]):
        a, b = region["start"], region["end"]
        if a < current:
            raise CoverageError(f"overlapping/unsorted coverage at 0x{a:x}")
        if b <= a or b > end:
            raise CoverageError(f"invalid coverage bounds 0x{a:x}–0x{b:x}")
        if current < a:
            missing.append({"start": current, "end": a, "bytes": a-current})
        current = b
    if current < end:
        missing.append({"start":current,"end":end,"bytes":end-current})
    return missing


def load(path: Path = DEFAULT_MANIFEST) -> dict[str, Any]:
    data = tomllib.loads(path.read_text(encoding="utf-8"))
    if data.get("schema_version") != 1:
        raise CoverageError("unsupported coverage manifest schema")
    images = data.get("images")
    if not isinstance(images, list) or not images:
        raise CoverageError("firmware image table is empty")
    seen: set[str] = set()
    for image in images:
        name = image.get("name", "")
        if not name or name in seen or Path(name).name != name:
            raise CoverageError(f"duplicate or invalid image name {name!r}")
        seen.add(name)
        size, credit = image.get("size"), image.get("credit_bytes")
        if not isinstance(size,int) or not 0 < size or not isinstance(credit,int) or not 0 <= credit <= size:
            raise CoverageError(f"invalid byte counts for {name}")
        sha = image.get("sha256", "")
        if len(sha) != 64 or any(c not in "0123456789abcdef" for c in sha):
            raise CoverageError(f"invalid SHA-256 for {name}")
        external_fields=("external_authentication_bytes","external_dsp_payload_bytes",
                         "external_calibration_bytes")
        for field in external_fields:
            value=image.get(field,0)
            if not isinstance(value,int) or value<0 or value>size:
                raise CoverageError(f"invalid external byte count: {name}")
        if credit+sum(image.get(field,0) for field in external_fields)>size:
            raise CoverageError(f"credit and external dependencies exceed image size: {name}")
        if image.get("complete_source", False) and (
            credit!=size or any(image.get(field,0) for field in external_fields)
        ):
            raise CoverageError(f"incorrect complete-source assertion: {name}")
        if "executable_load" in image:
            info = image["executable_load"]
            regions = image.get("regions", [])
            gaps = holes(regions, 0, info["runtime_end"])
            union = sum(r["end"]-r["start"] for r in regions)
            if union != credit:
                raise CoverageError(f"overlap/count mismatch: {name}: {union} vs {credit}")
            if union + sum(g["bytes"] for g in gaps) != info["runtime_end"]:
                raise CoverageError("incomplete executable partition")
    return data


def image_for(data: dict[str, Any], name: str) -> dict[str, Any]:
    for image in data["images"]:
        if image["name"] == name:
            return image
    raise CoverageError(f"unknown firmware image: {name}")


def _hash_file(path: Path) -> tuple[int, str]:
    """Hash without materializing an OEM firmware image in memory."""
    digest = hashlib.sha256()
    size = 0
    with path.open('rb') as source:
        for chunk in iter(lambda: source.read(1024 * 1024), b''):
            size += len(chunk)
            digest.update(chunk)
    return size, digest.hexdigest()


def verify_reference(data: dict[str,Any],name:str,original:Path) -> dict[str,Any]:
    image=image_for(data,name)
    size, actual=_hash_file(original)
    if size!=image['size']:
        raise CoverageError(f"{name}: reference size {size} differs from pinned {image['size']}")
    if actual!=image['sha256']:
        raise CoverageError(f"{name}: reference SHA-256 mismatch {actual}")
    return {"name":name,"reference_bytes":size,"sha256":actual,"matched_pinned_revision":True}


def verify_generated_region(data:dict[str,Any],name:str,reference:Path,generated:Path,offset:int) -> dict[str,Any]:
    original=verify_reference(data,name,reference)
    length=generated.stat().st_size
    if offset < 0 or not length or offset+length>original['reference_bytes']:
        raise CoverageError('generated region outside reference or empty')
    digest=hashlib.sha256()
    consumed=0
    with reference.open('rb') as source, generated.open('rb') as candidate:
        source.seek(offset)
        while part := candidate.read(1024 * 1024):
            expected=source.read(len(part))
            if part != expected:
                diff=next((i for i,(a,b) in enumerate(zip(part, expected)) if a!=b),
                          min(len(part),len(expected)))
                raise CoverageError(f"{name}: first generated/reference difference at file 0x{offset+consumed+diff:x}")
            digest.update(part)
            consumed+=len(part)
    if consumed!=length:
        raise CoverageError(f'{name}: generated file changed during verification')
    return {"name":name,"file_offset":offset,"generated_bytes":length,"matched":True,
            "generated_sha256":digest.hexdigest()}


def status(data: dict[str,Any]) -> str:
    images=data["images"]
    lines=["# Current firmware reconstruction status", "",
           "Generated from `reference/xiaomi-nabu/coverage.toml` by `python3 tools/firmware.py status`.", "",
           "**Caution:** counts include reported source encoding, metadata, literal tables and some modeled layout bytes. They are *not* proof of high-level source recovery, device behavior or newly valid OEM signatures.", "",
           "| Firmware | Original bytes | Reported generated/modelled | Not credited |", "| --- | ---: | ---: | ---: |"]
    for image in images:
        size=image["size"];credit=image["credit_bytes"]
        lines.append(f"| `{image['name']}` | {size:,} | {credit:,} | {size-credit:,} |")
    s=sum(x["size"] for x in images);c=sum(x["credit_bytes"] for x in images)
    lines.extend([f"| **Total** | **{s:,}** | **{c:,}** | **{s-c:,}** |","",
                  "## Venus first executable segment", ""])
    venus=image_for(data,"venus.mbn")
    code=venus["executable_load"]
    missing=holes(venus["regions"],0,code["runtime_end"])
    lines.extend([f"Reported source/layout: **{venus['credit_bytes']:,} / {code['runtime_end']:,} bytes**; uncovered: **{sum(x['bytes'] for x in missing):,} bytes**.","",
                  "| Uncovered ARM runtime addresses | Bytes |", "| --- | ---: |"])
    for x in missing:
        lines.append(f"| `0x{x['start']:x}–0x{x['end']:x}` | {x['bytes']:,} |")
    lines.extend(["", "## Important limits", "",
                  "- `modemuw.jsn` and the metadata-only `firmware-5.bin` are independently complete; the latter has no WLAN executable.",
                  "- The four 5,340-byte Cirrus WMDR coefficient configuration files are byte-identical from numeric tables, not HALO executable code.",
                  "- `board-2.bin` has 1,508 typed TLV/name framing bytes; 459,552 OEM calibration bytes remain external.",
                  "- A640 exact whole-file reproduction requires externally supplied original OEM authentication bytes.",
                  "- Cirrus HALO program bytes are still not independently decompiled; reference-assisted repacking is not firmware reconstruction.",
                  "- Several reference-dependent verification tests are skipped without supplied original firmwares and ISA toolchains.",
                  "- Novatek's pinned postmarketOS revision differs from the same-name older map220v image; verify with --override.",
                  "- The manifest includes all 19 source files, including nine presently unreconstructed binaries.",
                  "- The historical checkpoint-50 service source is absent from this recovered Git tree.",
                  "- No complete newly signed firmware build or hardware boot verification is claimed.",""])
    return "\n".join(lines)
