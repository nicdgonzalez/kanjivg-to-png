#!/usr/bin/env python3

"""
A script for finding all KanjiVG SVGs related to a specified Kanji.

From the root of the project, run:

    python3 ./scripts/lookup.py "本"  # Switch 本 to target Kanji
"""

import argparse
import os
import pathlib
import sys


def is_kanji(char: str) -> bool:
    """Check if a single character is a valid Japanese Kanji."""
    if len(char) != 1:
        raise ValueError("Input must be a single character.")

    code_point = ord(char)

    return (
        # KanjiVG has the full-width characters as half-width.
        0x0021 <= code_point <= 0x007A
        # Radicals?
        or 0x2E89 <= code_point <= 0x2ED6
        # Japanese-style Punctuation
        or 0x3000 <= code_point <= 0x303F
        # Hiragana
        or 0x3040 <= code_point <= 0x309F
        # Katakana
        or 0x30A0 <= code_point <= 0x30FF
        # CJK Unified Ideographs
        or 0x4E00 <= code_point <= 0x9FFF
        # CJK Unified Ideographs Extension A
        or 0x3400 <= code_point <= 0x4DBF
        # CJK Compatibility Ideographs
        or 0xF900 <= code_point <= 0xFAFF
        # Full-width roman characters and half-width Katakana
        or 0xFF00 <= code_point <= 0xFFEF
        # CJK Unified Ideographs Extensions B–F
        or 0x20000 <= code_point <= 0x2FA1F
    )


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "-d",
        "--directory",
        type=pathlib.Path,
        default=pathlib.Path(os.getenv("KVG_LOOKUP", "./kanji")),
        help="Path to the KanjiVG 'kanji' directory",
    )
    parser.add_argument(
        "kanji",
        type=str,
        help="Kanji to search for",
    )
    args = parser.parse_args()

    if len(args.kanji) > 1:
        sys.stderr.write("Expected a single Kanji character\n")
        return 1
    elif not is_kanji(args.kanji):
        sys.stderr.write("Invalid Kanji character\n")
        return 1
    else:
        pass

    # Convert the Kanji into its hexadecimal unicode value.
    kanji = hex(ord(args.kanji)).replace("0x", "")

    results = filter(
        # KanjiVG SVGs are named based on the hexadecimal Unicode value padded
        # to 5 characters using zeros.
        #
        # For 99% of Kanji, skipping the first character works fine;
        # I still need to investigate the files with a 2 in front...
        #
        # <https://kanjivg.tagaini.net/files.html>
        lambda f: f[1:].startswith(kanji),
        os.listdir(args.directory),
    )

    for result in results:
        sys.stdout.write(result + "\n")

    return 0


if __name__ == "__main__":
    sys.exit(main())
