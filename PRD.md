# Mac Disk Space Assistant (MDSA)

**Document version:** 1.0\
**Last updated:** 27 Jun 2025\
**Author:** \<your‑name>

## 1. Purpose

Modern Apple‑silicon Macs are extremely fast but ship with finite, non‑upgradeable SSDs. Users often discover too late that large artifacts (e.g. Docker volumes, Xcode caches, and media projects) silently consume tens or hundreds of gigabytes. **MDSA** is a lightweight Rust CLI that performs a *deep inaugural audit* and *fast, daily deltas*, producing actionable Markdown reports that help users reclaim space before hitting the “Low Disk” dialog.

## 2. Goals

| #  | Goal                                                                                     | Metric of Success                        |
| -- | ---------------------------------------------------------------------------------------- | ---------------------------------------- |
| G1 | Find ≥90 % of space‑consuming files/dirs in a one‑hour inaugural scan of a 2 TB disk     | Precision/recall measured in internal QA |
| G2 | Finish daily delta scan in ≤20 s on same machine                                         | Wall‑clock time                          |
| G3 | Shave ≥20 GB on first run for 80 % of test users (through recommendations)               | User pilot survey                        |
| G4 | Never exceed 250 MB RAM during any scan (default settings)                               | Memory profile                           |
| G5 | Report file (`~/MDSA.md`) is human‑readable, git‑friendly, and chronologically prepended | Qualitative UX review                    |

## 3. Non‑Goals

- GUI front‑end (can be added later).
- Automated deletion (tool recommends but never removes by itself).
- Support for macOS < 13.5 or Intel Macs.
- Network‑based aggregation of multiple machines.

## 4. Personas & User Stories

- **Indie Developer (Alex, 34)** — *“Docker ate my SSD again.”*\
  Runs `mdsa deep` once, reads recommendations, deletes unwanted containers.

- **Video Creator (María, 28)**\
  Schedules `mdsa daily` in `launchd` to warn when imports in `~/Movies` exceed 200 GB.

## 5. Functional Requirements

| ID  | Requirement                                                                                                      |
| --- | ---------------------------------------------------------------------------------------------------------------- |
| F‑1 | CLI exposes `` and `` sub‑commands.                                                                              |
| F‑2 | `deep` performs two phases: **Fast Triaged Scan** (common culprits) + **Full FS Scan**.                          |
| F‑3 | `daily` only scans triaged paths and diffs vs. yesterday’s snapshot (stored in `~/.mdsa/state.json`).            |
| F‑4 | Reports written to ``; daily run prepends a *changelog section* (date ‑ size difference ‑ top growth sources).   |
| F‑5 | Each report includes metrics: total bytes scanned, dirs/files visited, peak parallel workers, elapsed wall time. |
| F‑6 | Common‑culprit path set is customizable via `--config <path.toml>`.                                              |
| F‑7 | Exit codes: `0=OK`, `1=warnings`, `>1=fatal error (logged)`.                                                     |
| F‑8 | Universal binary (arm64 + x86\_64 for Rosetta fallback).                                                         |

## 6. Non‑Functional Requirements

- **Performance:** Heavily parallel (thread‑pool sized to logical cores).
- **Reliability:** Handles transient “Operation not permitted” gracefully; continues other workers.
- **Security & Privacy:** Never transmits data; state file stored with `0600` permissions.
- **M‑Series‑Aware:** Uses `fstatfs64`, `getattrlistbulk`, and APFS cloning metadata for speed where available.
- **Extensibility:** Pluggable *scanner modules* (e.g. future Slack cache, Lightroom library).
- **Localization:** English only v1.

## 7. Key “Common Culprit” Paths (default)

```
~/Desktop
~/Downloads
~/Movies
~/Library/Caches
~/Library/Containers/*
/Library/Developer
~/Library/Developer
~/Documents
~/Pictures/Photos Library.photoslibrary
~/Library/Application Support/Docker
~/Library/Containers/com.docker.docker
```

(Plus wildcard scans for `*.qcow2`, `*.dmg`, `*.ipa`, `node_modules`, `target`, `Pods`, etc.)

## 8. Competitive & Prior Art

- DaisyDisk, GrandPerspective – visual but slow on deltas; GUI‑only.
- `du -sh /*` – fast but no recommendations.
- MDSA differentiates by *Rust‑level parallelism, triaged deltas,* and *plain‑text reporting*.

## 9. Risks & Mitigations

| Risk                          | Likelihood | Impact | Mitigation                                        |
| ----------------------------- | ---------- | ------ | ------------------------------------------------- |
| APFS cloning hides true size  | Med        | Med    | Use `totalSize` & `rsrcSize` APIs + ACL traversal |
| macOS privacy TCC blocks dirs | Med        | Low    | Show readable instructions & re‑prompt            |
| Scan starves SSD / battery    | Low        | Med    | Nice‑level + I/O throttling; brief by default     |

## 10. Acceptance Criteria

- All functional requirements satisfied.
- 95 % unit‑test coverage of scanning core.
- Verified on macOS 15.0‑beta & 14.5.
- Pilot users reclaim space & report good UX (survey ≥4 / 5).

## 11. Milestones

1. **M0** – Project scaffolding & CLI skeleton (Week 1)
2. **M1** – Fast Triaged Scanner (Week 2)
3. **M2** – Full FS Scanner + Metrics (Week 3‑4)
4. **M3** – Report Generator & Markdown formatting (Week 4)
5. **M4** – State diff engine & daily mode (Week 5)
6. **M5** – Install scripts, codesigning, release (Week 6)

