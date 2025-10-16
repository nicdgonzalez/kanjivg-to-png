#!/usr/bin/env python3

"""
A script for converting all of the KanjiVG SVGs into PNG stroke diagrams.
"""

import argparse
import logging
import os
import pathlib
import shutil
import subprocess
import sys
from concurrent.futures import ProcessPoolExecutor, as_completed
from typing import Sequence

logger = logging.getLogger(name="kanjivg-to-png")


def main() -> None:
    """The main entry point to the program."""
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "-i",
        "--input",
        type=pathlib.Path,
        required=True,
        help="Path to the KanjiVG 'kanji' directory",
    )
    parser.add_argument(
        "-o",
        "--output",
        default=pathlib.Path.cwd().joinpath("kanji_output"),
        type=pathlib.Path,
        help="Path to directory to output the generated PNGs",
    )
    parser.add_argument(
        "-w",
        "--workers",
        default=os.cpu_count() or 4,
        type=int,
        help="Max number of processes to spread the work across",
    )
    parser.add_argument(
        "--no-progress",
        action="store_false",
        help="Disable the progress bar",
        dest="progress",
    )
    args = parser.parse_args()

    logging_setup(
        level=logging.WARNING,
        handlers=[
            logging.StreamHandler(stream=sys.stderr),
        ],
    )

    args.output.mkdir(parents=True, exist_ok=True)
    file_names = list(filter(is_valid_kanji_svg, os.listdir(args.input)))

    if args.progress:
        done = 0
        total = len(file_names)

    with ProcessPoolExecutor(max_workers=args.workers) as executor:
        futures = [
            executor.submit(
                process_svg,
                args.input.joinpath(file_name),
                args.output.joinpath(file_name.replace(".svg", ".png", 1)),
            )
            for file_name in file_names
        ]

        for future in as_completed(futures):
            _ = future.result(timeout=60.0)

            if args.progress:
                done += 1
                show_progress(done, total)

    # Final newline so bash prompt doesn't write on top of the bar.
    sys.stdout.write("\n")


def logging_setup(level: int, handlers: Sequence[logging.Handler]) -> None:
    formatter = logging.Formatter(
        fmt="%(asctime)s %(levelname)s %(name)s: %(message)s",
        datefmt="%Y-%m-%dT%H:%M:%S%z",  # ISO 8601 format
        style="%",
    )

    for handler in handlers:
        handler.setFormatter(fmt=formatter)
        logger.addHandler(hdlr=handler)

    logger.setLevel(level=level)


def is_valid_kanji_svg(file_name: str, /) -> bool:
    return file_name.startswith("0") and file_name.endswith(".svg")


def process_svg(input: pathlib.Path, output: pathlib.Path) -> None:
    try:
        _ = subprocess.run(
            [
                (
                    pathlib.Path.cwd()
                    .joinpath("target", "release", "kanjivg-to-png")
                    .as_posix()
                ),
                "--input",
                input.as_posix(),
                "--output",
                output.as_posix(),
            ],
        )
    except Exception as err:
        logger.error(f"[{input.stem}]: {err}", exc_info=True)


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
    bar = ("#" * filled).ljust(columns, "-")
    done_str = str(done).rjust(len(str(total)), "0")

    sys.stdout.write(f"\r|{bar}| {done_str}/{total} ({percent:.0%})")
    sys.stdout.flush()


if __name__ == "__main__":
    main()
