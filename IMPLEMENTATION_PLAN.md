# Implementation Plan — Mac Disk Space Assistant

**Branch strategy:** `main` (stable) / `dev` (PRs) / feature branches.\
**CI:** GitHub Actions – macOS‑14 runner, `cargo test`, `cargo clippy`, `cargo audit`, `codesign --verify`.

---

## 1 High‑Level Architecture

```
                 ┌─────────────────┐
                 │   CLI Frontend  │  ← clap derive
                 └────────┬────────┘
                          ▼
┌──────────────────────────────────────────────┐
│            Core Service Layer               │
├──────────────────────────────────────────────┤
│ 1. Path Scheduler (rayon)  │ 2. Scanner Pool │
├────────────────────────────┴─────────────────┤
│ 3. Analyzer & Heuristics   │ 4. Diff Engine  │
├──────────────────────────────────────────────┤
│           Report Generator (pulldown‑cmark) │
└──────────────────────────────────────────────┘
                          ▼
                 ~/MDSA.md + ~/.mdsa/state.json
```

- **Scheduler** — breadth‑first path queue; steals work across Rayon threads.
- **Scanner** — stat‑lightweight (“size only”) → optional deep checksum.
- **Analyzer** — pattern rules (`*.qcow2`, `node_modules`) + per‑module logic (Docker: use `docker system df -v`).
- **Diff** — JSON snapshot keyed by inode + size + mtime; produces delta summary.
- **ReportGen** — Converts structs to GitHub‑flavoured Markdown.

---

## 2 Core Crates & Dependencies

|  Purpose               |  Crate                                                   |
| ---------------------- | -------------------------------------------------------- |
|  CLI parsing           |  `clap` (derive)                                         |
|  Parallelism           |  `rayon`                                                 |
|  Filesystem traversal  |  `walkdir` (+ `same‑file`)                               |
|  APFS size             |  `fsext` (internal wrapper over `statfs`/`getattrlist`)  |
|  Serialization         |  `serde`, `serde_json`, `toml`                           |
|  Time & metrics        |  `chrono`, `indicatif`                                   |
|  Markdown              |  `pulldown‑cmark`                                        |
|  Docker inspection     |  `bollard` (async Docker API)                            |
|  Testing               |  `assert_cmd`, `tempfile`, `insta`                       |

*No unsafe code planned; build with **`#![forbid(unsafe_code)]`**.*

---

## 3 Detailed Module Work‑Breakdown

|  ID    |  Module                 |  Key Tasks                                                              |  Est. Effort                                 |       |
| ------ | ----------------------- | ----------------------------------------------------------------------- | -------------------------------------------- | ----- |
|  3‑1   |  **cli**                |  \`mdsa [deep                                                           | daily] [--config ]\` parsing; global flags.  |  1 d  |
|  3‑2   |  **paths**              |  Default culprit list; config overlay.                                  |  0.5 d                                       |       |
|  3‑3   |  **scheduler**          |  Rayon `ThreadPoolBuilder`, dynamic chunk sizing, polite I/O throttle.  |  2 d                                         |       |
|  3‑4   |  **scanner\_fs**        |  Metadata gather, symlink resolution, error bubbling.                   |  2 d                                         |       |
|  3‑5   |  **scanner\_docker**    |  Call `bollard::system::df`, map sizes; guard if Docker not running.    |  1 d                                         |       |
|  3‑6   |  **analyzer**           |  Rule DSL (TOML): size threshold, glob match, recommendation string.    |  1.5 d                                       |       |
|  3‑7   |  **snapshot**           |  JSON struct persisted in `~/.mdsa/state.json`.                         |  1 d                                         |       |
|  3‑8   |  **diff**               |  Compare snapshots; compute growth/shrink sets.                         |  1 d                                         |       |
|  3‑9   |  **report**             |  Markdown templates, atomic prepend.                                    |  1.5 d                                       |       |
|  3‑10  |  **integration tests**  |  Synthetic large files; golden Markdown.                                |  2 d                                         |       |
|  3‑11  |  **release**            |  Universal binary via `lipo`; notarize.                                 |  1 d                                         |       |

*Total engineering effort ≈ 14 developer‑days.*

---

## 4 Threading & Performance Strategy

- **Rayon thread‑pool** size = `num_cpus::get_physical()` (override with `--threads`).
- Work item granularity: directory path. Worker pops path, scans entries, pushes subdirs.
- Use `stat` first; avoid content reads.
- Throttle disk by sleeping 1 ms after every 500 metadata calls.
- Metrics via `Instant` + atomic counters.

---

## 5 File & Folder Conventions

|  Path                   |  Purpose                          |
| ----------------------- | --------------------------------- |
|  `~/MDSA.md`            |  Human report (prepend on daily)  |
|  `~/.mdsa/state.json`   |  Machine snapshot                 |
|  `~/.mdsa/mdsa.log`     |  Verbose logs (`--verbose`)       |
|  `/usr/local/bin/mdsa`  |  Recommended install symlink      |

---

## 6 Cron / Launchd Integration

A helper command installs a launchd job:

```bash
mdsa install‑daily
```

Creates `~/Library/LaunchAgents/com.mdsa.daily.plist`:

```xml
<key>ProgramArguments</key>
<array>
  <string>/usr/local/bin/mdsa</string>
  <string>daily</string>
</array>
<key>StartCalendarInterval</key>
<dict>
  <key>Hour</key><integer>18</integer>
  <key>Minute</key><integer>0</integer>
</dict>
```

---

## 7 Testing Matrix

|  macOS      |  Chip             |  Filesystem        |  Status        |
| ----------- | ----------------- | ------------------ | -------------- |
|  15.0 beta  |  M3 Max           |  APFS              |  Must pass     |
|  14.5       |  M1               |  APFS (FileVault)  |  Must pass     |
|  14.5       |  Intel (Rosetta)  |  APFS              |  Nice‑to‑have  |

---

## 8 Documentation & DX

- Inline Rustdoc for each public function.
- `docs/` folder with usage, config examples, FAQ.
- Release README links to Homebrew tap formula.

---

## 9 Roll‑out

1. **0.1.0‑alpha** — internal dogfood.
2. **0.2.0‑beta** — public GitHub + Homebrew; opt‑in telemetry.
3. **1.0.0** — notarized DMG, blog post.

---

## 10 Open Questions

- Handle iCloud “optimize storage” phantom sizes?
- Parse Photo Library sub‑packages (SQLite) — v2 plugin?

---

## 11 Appendix A – Example Deep Report Snippet

```md
# Mac Disk Space Assistant Report
Run type: deep
Start: 2025‑07‑01T10:12:33‑05:00
Duration: 00:07:18
Parallel workers: 8
Paths scanned: 312 912 files in 12 455 dirs
Total scanned size (logical): 1.44 TB
-----------------------------------------------------------------
## Top Recommendations
| Rank | Path | Size | Why |
| 1 | ~/Library/Containers/com.docker.docker/Data/vms/0/Docker.raw | 52 GB | Unused for 14 days – run `docker system prune --volumes`. |
| 2 | ~/Movies/FinalCut/ProjectA | 34 GB | Completed project – consider archiving to external drive. |
```

---

