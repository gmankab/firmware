#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""Regenerate Agent 20-18 data independently, using only bundled public source.

``independent_fragments(root)`` returns nine disjoint/adjacent fragments keyed
by firmware-file offset. It reads no OEM image and invokes neither Rust nor a
compiler. Integer recipes construct the coordinate/width/square tables. A
96-digit Decimal calculation constructs the three degree-indexed trig tables:
pi is computed by Machin's arctangent identity, and sine by direct factorial
Taylor terms. This arithmetic is independent of the Rust binary64 recurrence.

The logarithm data is parsed from unchanged, hash-pinned Arm C source. Its
physical field order is obtained separately from the bundled, hash-pinned
math_config.h declaration. The selected configuration is N=128, polynomial
orders 6 and 12, without fast FMA. The C source retains its original license;
see reference/xiaomi-nabu/novatek/ilm-upper/arm-log-data-provenance.txt.

The CLI emits JSON accounting. If --reference is supplied, the exact original
image SHA-256 and length are required before its fragment bytes are compared.
"""
from __future__ import annotations

import argparse
from decimal import Decimal, ROUND_FLOOR, ROUND_HALF_EVEN, ROUND_HALF_UP, localcontext
import hashlib
import json
import math
from pathlib import Path
import re
import struct
import sys


PINNED_SHA256 = "8b845514621ebfd95759a0919f3aa3d825314abb478a7cad1c3fb4c6872b70b5"
IMAGE_SIZE = 196_608
DECIMAL_PRECISION = 96
LOG_SOURCE_SHA256 = "327cf1819870930486eecb5491c68d5887a712b954e5ec82f3f70bfd9e46fdd3"
LOG_HEADER_SHA256 = "8fcde1b1c5f64f7a624b398639b33215f5e76c1ab5be55e3d26b011e51c3bfb1"
LOG_OUTPUT_SHA256 = "c26af2ad9c57bf0b64d2d1b93b1e911170e63c39d7bf69bfe423b9e8770a8f4c"
REFERENCE_DIRECTORY = Path("reference/xiaomi-nabu/novatek/ilm-upper")
LOG_FIELD_ORDER = ("ln2hi", "ln2lo", "poly", "poly1", "tab", "tab2")
LOG_FIELD_COUNTS = {"ln2hi": 1, "ln2lo": 1, "poly": 5,
                    "poly1": 11, "tab": 256, "tab2": 256}
EXPECTED_LENGTHS = {
    0x195B0: 100, 0x19614: 100, 0x19678: 64, 0x196B8: 64,
    0x198DC: 184, 0x19994: 184, 0x19A4C: 184, 0x19B04: 184,
    0x19BF0: 4240,
}


def _checked_public_source(path: Path, expected_sha256: str) -> str:
    data = path.read_bytes()
    digest = hashlib.sha256(data).hexdigest()
    if digest != expected_sha256:
        raise ValueError(f"public source hash mismatch for {path.name}: {digest}")
    return data.decode("utf-8")


def _strip_comments(source: str) -> str:
    # These pinned files contain no string literals with comment delimiters in
    # the declarations being parsed. No executable C or Python is evaluated.
    return re.sub(r"/\*.*?\*/|//[^\n]*", "", source, flags=re.DOTALL)


def _condition(expression: str, constants: dict[str, int]) -> bool:
    """Evaluate the small, explicit conditional grammar used by log_data.c."""
    terms = expression.strip().split("&&")
    values = []
    for term in terms:
        term = term.strip()
        equal = re.fullmatch(r"([A-Za-z_]\w*)\s*==\s*(\d+)", term)
        negate = re.fullmatch(r"!\s*([A-Za-z_]\w*)", term)
        if equal and equal.group(1) in constants:
            values.append(constants[equal.group(1)] == int(equal.group(2)))
        elif negate and negate.group(1) in constants:
            values.append(not constants[negate.group(1)])
        else:
            raise ValueError(f"unsupported public-source conditional: {term!r}")
    return all(values)


def _select_configuration(source: str, constants: dict[str, int]) -> str:
    """Select known #if branches without requiring cpp or evaluating code."""
    selected = []
    # Each entry records parent activity, any previously selected branch, and
    # current branch activity, so #elif and nested #if cannot leak constants.
    stack: list[tuple[bool, bool, bool]] = []
    for line in _strip_comments(source).splitlines():
        directive = re.match(r"^\s*#\s*(\w+)\b(.*)$", line)
        active = not stack or stack[-1][2]
        if not directive:
            if active:
                selected.append(line)
            continue
        command, expression = directive.groups()
        if command == "if":
            match = _condition(expression, constants)
            stack.append((active, match, active and match))
        elif command == "elif":
            if not stack:
                raise ValueError("unmatched #elif in public source")
            parent, taken, _ = stack.pop()
            match = _condition(expression, constants)
            stack.append((parent, taken or match, parent and not taken and match))
        elif command == "else":
            if not stack:
                raise ValueError("unmatched #else in public source")
            parent, taken, _ = stack.pop()
            stack.append((parent, True, parent and not taken))
        elif command == "endif":
            if not stack:
                raise ValueError("unmatched #endif in public source")
            stack.pop()
        elif command not in ("include", "define"):
            raise ValueError(f"unexpected public-source directive: #{command}")
    if stack:
        raise ValueError("unterminated conditional in public source")
    return "\n".join(selected)


def _log_fragment(root: Path) -> bytes:
    directory = root / REFERENCE_DIRECTORY
    source = _checked_public_source(directory / "arm-log_data.c", LOG_SOURCE_SHA256)
    header = _checked_public_source(directory / "arm-math_config.h", LOG_HEADER_SHA256)
    parameters = {}
    for name in ("LOG_TABLE_BITS", "LOG_POLY_ORDER", "LOG_POLY1_ORDER"):
        match = re.search(r"^#define\s+" + name + r"\s+(\d+)\s*$", header, re.MULTILINE)
        if not match:
            raise ValueError(f"missing {name} declaration in public header")
        parameters[name] = int(match.group(1))
    if parameters != {"LOG_TABLE_BITS": 7, "LOG_POLY_ORDER": 6, "LOG_POLY1_ORDER": 12}:
        raise ValueError("public header selects a different logarithm configuration")
    constants = {**parameters, "N": 1 << parameters["LOG_TABLE_BITS"], "HAVE_FAST_FMA": 0}

    declaration = re.search(r"extern\s+const\s+struct\s+log_data\s*\{(.*?)\}\s*__log_data\b",
                            _strip_comments(header), re.DOTALL)
    if not declaration:
        raise ValueError("public header lacks the expected struct log_data")
    declaration = _select_configuration(declaration.group(1), constants)
    # Collapse each paired-field table to its outer field name before reading
    # the declared layout; do not mistake invc/logc or chi/clo for outer fields.
    flattened = re.sub(r"struct\s*\{[^}]*\}\s*(\w+)\s*\[[^]]+\]\s*;",
                       lambda match: f"double {match.group(1)};", declaration)
    field_order = tuple(re.findall(r"\bdouble\s+(\w+)", flattened))
    if field_order != LOG_FIELD_ORDER:
        raise ValueError(f"unexpected public struct field order: {field_order}")

    initializer = _select_configuration(source, constants)
    fields = list(re.finditer(r"\.(\w+)\s*=", initializer))
    values: dict[str, list[float]] = {}
    hex_float = r"[-+]?0x[\da-fA-F]+(?:\.[\da-fA-F]*)?[pP][-+]?\d+"
    for index, match in enumerate(fields):
        end = fields[index + 1].start() if index + 1 < len(fields) else len(initializer)
        name = match.group(1)
        if name in values:
            raise ValueError(f"duplicate public initializer field: {name}")
        tokens = re.findall(hex_float, initializer[match.end():end])
        values[name] = [float.fromhex(token) for token in tokens]
    if {key: len(items) for key, items in values.items()} != LOG_FIELD_COUNTS:
        raise ValueError("public logarithm initializer field counts changed")
    scalars = [value for name in field_order for value in values[name]]
    if len(scalars) != 530 or not all(math.isfinite(value) for value in scalars):
        raise ValueError("public logarithm constants must be 530 finite binary64 values")
    result = b"".join(struct.pack("<d", value) for value in scalars)
    if hashlib.sha256(result).hexdigest() != LOG_OUTPUT_SHA256:
        raise ValueError("selected public logarithm constants have an unexpected digest")
    return result


def _arctangent_reciprocal(reciprocal: int, epsilon: Decimal) -> Decimal:
    """Alternating arctan series for 1/5 and 1/239 in Machin's identity."""
    power = Decimal(1) / reciprocal
    square = reciprocal * reciprocal
    total = Decimal(0)
    for index in range(10_000):
        term = power / (2 * index + 1)
        total += term if index % 2 == 0 else -term
        if abs(term) < epsilon:
            return total
        power /= square
    raise ValueError("Machin arctangent failed to converge")


def _sine_degrees(degrees: int, pi: Decimal, epsilon: Decimal) -> Decimal:
    if degrees == 0:
        return Decimal(0)
    if degrees == 90:
        return Decimal(1)
    angle = pi * degrees / 180
    total = Decimal(0)
    for index in range(200):
        # Direct powers and exact integer factorials deliberately differ from
        # the Rust emitter's binary64 paired-term recurrence.
        term = angle ** (2 * index + 1) / Decimal(math.factorial(2 * index + 1))
        total += term if index % 2 == 0 else -term
        if abs(term) < epsilon:
            return total
    raise ValueError("Decimal sine failed to converge")


def _rounded_scaled(value: Decimal) -> int:
    scaled = value * 100
    lower = scaled.to_integral_value(rounding=ROUND_FLOOR)
    # The actual tables have margins above 0.003 from every half-integer.
    # Refuse a numerically ambiguous sample instead of silently trusting it.
    if abs(scaled - lower - Decimal("0.5")) < Decimal("1e-70"):
        raise ValueError("trigonometric sample is too close to a rounding boundary")
    sample = int(scaled.to_integral_value(rounding=ROUND_HALF_UP))
    if not 0 <= sample <= 65_535:
        raise ValueError("trigonometric sample does not fit u16")
    return sample


def _pack_u16(values: list[int], align: bool = False) -> bytes:
    result = b"".join(struct.pack("<H", value) for value in values)
    if align:
        result += bytes((-len(result)) % 4)
    return result


def _mathematical_fragments(precision: int = DECIMAL_PRECISION) -> dict[int, bytes]:
    if precision < 80:
        raise ValueError("independent trigonometry requires at least 80 decimal digits")
    centers_50 = [(256 * index + 130) // 5 for index in range(50)]
    # Solve each cell's opposite edge from its midpoint, using exact integers.
    edges = [0]
    for center in centers_50:
        edges.append(2 * center - edges[-1])
    widths_50 = [right - left for left, right in zip(edges, edges[1:])]
    if edges[-1] != 2560 or any(width <= 0 for width in widths_50):
        raise ValueError("centered coordinates do not define a positive tiling")
    fragments = {
        0x195B0: _pack_u16(centers_50),
        0x19614: _pack_u16(widths_50),
        0x19678: _pack_u16([25 + 50 * index for index in range(32)]),
        0x196B8: _pack_u16([50] * 32),
    }
    with localcontext() as context:
        context.prec = precision + 16
        context.rounding = ROUND_HALF_EVEN
        epsilon = Decimal(1).scaleb(-(precision + 8))
        pi = (16 * _arctangent_reciprocal(5, epsilon)
              - 4 * _arctangent_reciprocal(239, epsilon))
        sine = [_sine_degrees(degrees, pi, epsilon) for degrees in range(91)]
        cosine = list(reversed(sine))
        fragments[0x198DC] = _pack_u16([_rounded_scaled(value) for value in sine], True)
        fragments[0x19994] = _pack_u16([_rounded_scaled(value) for value in cosine], True)
        tangent = [_rounded_scaled(sine[index] / cosine[index]) for index in range(90)]
        tangent.append(10_000)  # Explicit finite endpoint marker, not tan(90).
        fragments[0x19A4C] = _pack_u16(tangent, True)
    fragments[0x19B04] = _pack_u16([index * index for index in range(91)], True)
    return fragments


def independent_fragments(root: Path) -> dict[int, bytes]:
    """Construct eight mathematical fragments and one public-source log table.

    ``root`` is the repository root. The only files read are the two pinned
    public Arm source files in its reference directory; no firmware is needed.
    All offsets are absolute file offsets in the pinned Novatek image.
    """
    fragments = _mathematical_fragments()
    fragments[0x19BF0] = _log_fragment(Path(root))
    if {start: len(data) for start, data in fragments.items()} != EXPECTED_LENGTHS:
        raise ValueError("independent data fragment accounting changed")
    previous_end = 0
    for start, data in sorted(fragments.items()):
        if start < previous_end:
            raise ValueError("independent data fragments overlap")
        previous_end = start + len(data)
    return fragments


def checked_reference(data: bytes) -> bytes:
    digest = hashlib.sha256(data).hexdigest()
    if len(data) != IMAGE_SIZE or digest != PINNED_SHA256:
        raise ValueError(f"wrong reference: size={len(data)}, sha256={digest}")
    return data


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[3])
    parser.add_argument("--reference", type=Path,
                        help="optional original image, checked against the exact pinned SHA-256")
    args = parser.parse_args()
    try:
        fragments = independent_fragments(args.root)
        reference = checked_reference(args.reference.read_bytes()) if args.reference else None
        entries = []
        for start, data in sorted(fragments.items()):
            if reference is not None and reference[start:start + len(data)] != data:
                actual = reference[start:start + len(data)]
                first = next(index for index, pair in enumerate(zip(data, actual))
                             if pair[0] != pair[1])
                raise ValueError(f"independent data mismatch at file {start + first:#x}")
            entries.append({"file_start": start, "file_end": start + len(data),
                            "bytes": len(data), "sha256": hashlib.sha256(data).hexdigest()})
        report = {"independent_data_bytes": sum(len(data) for data in fragments.values()),
                  "mathematical_sample_bytes": 1056, "alignment_zero_bytes": 8,
                  "public_source_log_bytes": 4240, "decimal_precision": DECIMAL_PRECISION,
                  "fragments": entries, "reference_compared": reference is not None,
                  "reference_sha256": PINNED_SHA256 if reference is not None else None,
                  "mismatched_bytes": 0 if reference is not None else None}
        print(json.dumps(report, indent=2))
    except (OSError, ValueError) as error:
        print(f"Agent 20-18 independent data verification failed: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
