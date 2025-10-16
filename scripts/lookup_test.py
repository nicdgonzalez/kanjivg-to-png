#!/usr/bin/env python3

"""
A script to test each of the KanjiVG SVG files to make sure `lookup.py` is
working correctly.

From the project root, run:

    KVG_LOOKUP="./data/kanji" python3 ./scripts/lookup_test.py

"""

import os
import pathlib
import shutil
import subprocess
import sys
from concurrent.futures import ProcessPoolExecutor, as_completed


def main() -> None:
    kanji_dir = pathlib.Path(os.getenv("KVG_LOOKUP", "./kanji"))
    files = filter(
        lambda f: f.startswith("0") and len(f) == 9,
        os.listdir(kanji_dir),
    )
    kanji = map(lambda f: f.removesuffix(".svg"), files)
    kanji = list(map(lambda k: chr(int(k[1:], 16)), kanji))

    done = 0
    total = len(kanji)

    with ProcessPoolExecutor(max_workers=os.cpu_count() or 4) as executor:
        futures = [executor.submit(test_kanji, k) for k in kanji]

        for future in as_completed(futures):
            _ = future.result(timeout=60.0)

            done += 1
            show_progress(done, total)

    # Final newline so bash prompt doesn't write on top of the bar.
    sys.stdout.write("\n")


def test_kanji(kanji: str) -> str | None:
    result = subprocess.run(
        ["python3", "./scripts/lookup.py", kanji],
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
    )

    try:
        result.check_returncode()
    except subprocess.CalledProcessError:
        unicode = ord(kanji)
        hexadecimal = hex(unicode)

        sys.stderr.write(
            f"error: check failed for Kanji: {kanji} ({unicode}, {hexadecimal})\n"  # noqa: E501
        )


def show_progress(
    done: int,
    total: int,
    columns: int | None = None,
) -> None:
    if columns is None:
        terminal_size = shutil.get_terminal_size()
        limit = 60
        columns = min(limit, max(limit, terminal_size.columns))

    percent = done / total
    filled = int(columns * percent)
    bar = ("#" * filled).ljust(columns, " ")
    done_str = str(done).rjust(len(str(total)), "0")

    sys.stdout.write(f"\r|{bar}| {done_str}/{total} ({percent:.0%})")
    sys.stdout.flush()


if __name__ == "__main__":
    main()
