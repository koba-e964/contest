#!/usr/bin/env python3
"""
Rust で定義されている関数を出現回数順に表示する
"""
import os
import re
import sys
from pathlib import Path


FN_RE = re.compile(r"(\A|\n)fn\s+([A-Za-z_][A-Za-z0-9_]*)\s*\(")


def iter_rs_files(root: Path):
    for dirpath, _, filenames in os.walk(root):
        for name in filenames:
            if name.endswith(".rs"):
                yield Path(dirpath) / name


def strip_block_comments(text: str) -> str:
    # Naive removal to reduce false positives in commented-out code.
    return re.sub(r"/\*.*?\*/", "", text, flags=re.S)


def strip_line_comments(text: str) -> str:
    return re.sub(r"//.*", "", text)


def extract_fn_names(text: str):
    """
    Returns all function names
    """
    text = strip_block_comments(text)
    text = strip_line_comments(text)
    return FN_RE.findall(text)


def main(argv: list[str]) -> int:
    "main"
    root = Path(argv[1]).resolve() if len(argv) > 1 else Path.cwd()
    counts: dict[str, int] = {}
    for path in iter_rs_files(root):
        try:
            content = path.read_text(encoding="utf-8", errors="ignore")
        except OSError:
            continue
        for groups in extract_fn_names(content):
            name = groups[1]
            counts[name] = counts.get(name, 0) + 1
    for name, count in sorted(counts.items(), key=lambda item: (-item[1], item[0])):
        print(f"{name}\t{count}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
