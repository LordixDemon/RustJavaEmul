#!/usr/bin/env python3
"""Merge unique docs/missing.json members into java_runtime/src/coverage_stub_data.rs."""

from __future__ import annotations

import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
MISSING = ROOT / "docs" / "missing.json"
STUBS = ROOT / "java_runtime" / "src" / "coverage_stub_data.rs"

KIND = {"class": 0, "method": 1, "field": 2}


def rust_str(value: str) -> str:
    return '"' + value.replace("\\", "\\\\").replace('"', '\\"') + '"'


def load_existing() -> set[tuple[str, int, str, str]]:
    text = STUBS.read_text(encoding="utf-8")
    out: set[tuple[str, int, str, str]] = set()
    for match in re.finditer(
        r'\("((?:\\.|[^"\\])*)",\s*(\d+),\s*"((?:\\.|[^"\\])*)",\s*"((?:\\.|[^"\\])*)"\)',
        text,
    ):
        cls, kind, name, desc = match.groups()
        out.add((cls.replace("\\\\", "\\"), int(kind), name.replace("\\\\", "\\"), desc.replace("\\\\", "\\")))
    return out


def load_missing() -> set[tuple[str, int, str, str]]:
    data = json.loads(MISSING.read_text(encoding="utf-8"))
    out: set[tuple[str, int, str, str]] = set()
    jars_with_holes = 0
    for members in data.values():
        if not members:
            continue
        jars_with_holes += 1
        for member in members:
            kind = KIND[member["kind"]]
            out.add((member["class"], kind, member.get("name") or "", member.get("descriptor") or ""))
            if kind != 0:
                out.add((member["class"], 0, "", ""))
    return out, jars_with_holes


def write_stubs(entries: set[tuple[str, int, str, str]]) -> None:
    lines = [
        "// Generated from docs/missing.json unique API members. Do not edit by hand.",
        "pub(super) const COVERAGE_STUBS: &[(&str, u8, &str, &str)] = &[",
    ]
    for cls, kind, name, desc in sorted(entries):
        lines.append(f"    ({rust_str(cls)}, {kind}, {rust_str(name)}, {rust_str(desc)}),")
    lines.append("];")
    lines.append("")
    STUBS.write_text("\n".join(lines), encoding="utf-8", newline="\n")


def main() -> None:
    existing = load_existing()
    missing, jars_with_holes = load_missing()
    added = missing - existing
    merged = existing | missing
    write_stubs(merged)
    print(
        f"jars with holes={jars_with_holes} unique_missing={len(missing)} "
        f"new={len(added)} total_stubs={len(merged)}"
    )


if __name__ == "__main__":
    main()
