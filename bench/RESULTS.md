# Benchmark Results: exiftool-rs vs exiftool

`exiftool-rs` (this project) compared against the installed Perl `exiftool` on a
corpus of real-world JPEGs. Reproduce with `python3 bench/bench.py`.

## Environment

| | |
|---|---|
| **Machine** | MacBook Pro (`MacBookPro18,3`) |
| **Chip** | Apple M1 Pro — 10 cores (8 performance + 2 efficiency) |
| **Memory** | 32 GB |
| **OS** | macOS 26.0 (build 25A354) |
| **exiftool** | 13.55 (Homebrew, `/opt/homebrew/bin/exiftool`, Perl) |
| **exiftool-rs** | 0.1.0, `--release` build (opt-level 3), rustc 1.90.0 |
| **Date** | 2026-06-04 |

## Input

- **688** JPG/JPEG files from `~/Downloads`
- **1,226.4 MB** total (ranges from a few KB to ~15 MB per file)

## Method

- Each tool's stdout/stderr is discarded; only wall-clock time is measured.
- Times are **best-of-N** (N=3 batch, N=2 per-file) to reduce noise.
- Two modes:
  - **Batch** — all files passed to a single invocation (exiftool amortises its
    interpreter/module startup over the whole run; its best case).
  - **Per-file** — one process invocation per file (includes startup cost each
    time; common in shell loops and pipelines). Run over a 60-file subset to
    keep total wall time reasonable.

## Results

| Mode | Files | exiftool | exiftool-rs | Speedup | ET files/s | RS files/s | ET MB/s | RS MB/s |
|------|-------|----------|-------------|---------|-----------|-----------|---------|---------|
| Batch (all files, 1 invocation) | 688 | 4,262 ms | 214 ms | **19.9×** | 161 | 3,217 | 288 | 5,734 |
| Per-file (1 invocation/file)    | 60  | 11,342 ms | 465 ms | **24.4×** | 5 | 129 | 8 | 186 |

*(speedup = exiftool ÷ exiftool-rs)*

A prior run on the same machine measured 21.3× / 24.9×; run-to-run variance is a
few percent.

## Interpretation

- **Batch:** exiftool-rs is ~20× faster (214 ms vs 4.3 s; ~3,200 vs ~160
  files/s). This is the throughput of the parsing core itself.
- **Per-file:** the gap widens to ~24× because exiftool pays ~190 ms of Perl
  interpreter + module-load startup *per invocation*, while exiftool-rs starts in
  ~1 ms.

## Fairness caveat

This is **not** a same-output comparison. exiftool-rs currently extracts the
standard EXIF/GPS/JFIF/File tags but **skips** maker-note, XMP, IPTC, ICC, and
Composite-tag parsing that exiftool performs, so it does less work per file.
Part of the speedup reflects that reduced scope. The remainder — and all of the
per-file startup advantage — is structural (native Rust vs interpreted Perl) and
will persist as more parsers are added.

See `bench-output.txt` for the captured raw run and `bench.py` for the harness.
