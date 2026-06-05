#!/usr/bin/env python3
"""Benchmark exiftool-rs against the installed exiftool on a set of JPEGs."""
import glob
import os
import subprocess
import sys
import time

EXIFTOOL = "/opt/homebrew/bin/exiftool"
EXIFTOOL_RS = "/Users/dwalter/projects/exiftool/exiftool-rs/target/release/exiftool-rs"
DOWNLOADS = os.path.expanduser("~/Downloads")


def collect_files():
    files = []
    for pat in ("*.jpg", "*.JPG", "*.jpeg", "*.JPEG"):
        files.extend(glob.glob(os.path.join(DOWNLOADS, pat)))
    # de-dupe (case-insensitive globs can overlap) and keep readable regular files
    seen, out = set(), []
    for f in sorted(files):
        if f in seen or not os.path.isfile(f):
            continue
        seen.add(f)
        out.append(f)
    return out


def run_timed(cmd, runs):
    """Return the best (min) wall-clock time over `runs`, discarding output."""
    best = float("inf")
    for _ in range(runs):
        t0 = time.perf_counter()
        subprocess.run(cmd, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        best = min(best, time.perf_counter() - t0)
    return best


def batch(tool, files, runs=3):
    return run_timed([tool, *files], runs)


def per_file(tool, files, runs=2):
    """Best total wall time to invoke `tool` once per file."""
    best = float("inf")
    for _ in range(runs):
        t0 = time.perf_counter()
        for f in files:
            subprocess.run([tool, f], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        best = min(best, time.perf_counter() - t0)
    return best


def fmt_table(rows, headers):
    widths = [len(h) for h in headers]
    for r in rows:
        for i, c in enumerate(r):
            widths[i] = max(widths[i], len(str(c)))
    line = "| " + " | ".join(h.ljust(widths[i]) for i, h in enumerate(headers)) + " |"
    sep = "|-" + "-|-".join("-" * widths[i] for i in range(len(headers))) + "-|"
    print(line)
    print(sep)
    for r in rows:
        print("| " + " | ".join(str(c).ljust(widths[i]) for i, c in enumerate(r)) + " |")


def main():
    files = collect_files()
    total_bytes = sum(os.path.getsize(f) for f in files)
    total_mb = total_bytes / 1e6
    n = len(files)
    print(f"Files: {n}   Total size: {total_mb:.1f} MB   "
          f"(exiftool {subprocess.run([EXIFTOOL,'-ver'],capture_output=True,text=True).stdout.strip()}, "
          f"exiftool-rs {subprocess.run([EXIFTOOL_RS,'-ver'],capture_output=True,text=True).stdout.strip()})\n")

    # --- Batch mode: all files in one invocation ---
    rs_b = batch(EXIFTOOL_RS, files)
    et_b = batch(EXIFTOOL, files)

    # --- Per-file mode: subset to keep wall time reasonable ---
    subset = files[:: max(1, n // 60)][:60]
    sub_bytes = sum(os.path.getsize(f) for f in subset) / 1e6
    rs_p = per_file(EXIFTOOL_RS, subset)
    et_p = per_file(EXIFTOOL, subset)

    rows = [
        ["Batch (all files, 1 invocation)", n,
         f"{et_b*1000:.0f}", f"{rs_b*1000:.0f}",
         f"{et_b/rs_b:.1f}x",
         f"{n/et_b:.0f}", f"{n/rs_b:.0f}",
         f"{total_mb/et_b:.0f}", f"{total_mb/rs_b:.0f}"],
        ["Per-file (1 invocation/file)", len(subset),
         f"{et_p*1000:.0f}", f"{rs_p*1000:.0f}",
         f"{et_p/rs_p:.1f}x",
         f"{len(subset)/et_p:.0f}", f"{len(subset)/rs_p:.0f}",
         f"{sub_bytes/et_p:.0f}", f"{sub_bytes/rs_p:.0f}"],
    ]
    headers = ["Mode", "Files", "exiftool ms", "exiftool-rs ms", "speedup",
               "ET file/s", "RS file/s", "ET MB/s", "RS MB/s"]
    fmt_table(rows, headers)
    print("\n(times are best-of-N wall clock; output discarded; speedup = exiftool / exiftool-rs)")


if __name__ == "__main__":
    sys.exit(main())
