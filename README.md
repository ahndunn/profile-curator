# profile-curator

A high-performance, LLM-friendly CLI engine written in Rust that enables conversational AI agents to iteratively interview candidates, curate their professional background, preserve an interview story vault, track quantified impact metrics, log dual-format audit changelogs, and export directly to `cv-writer`.

## Key Capabilities

1. **CLI-First Architecture with LLM Guidance**:
   - Designed for direct invocation by shell scripts, CI/CD, and conversational AI agents.
   - Built-in `profile-curator guide --state profile.json` inspects completeness and recommends next conversational questions to ask the candidate.
   - Self-describing `--help` flags provide step-by-step guidance tailored for LLM tool-calling contexts.

2. **Interview Story Vault (`stories`)**:
   - Stores structured STAR experiences (Situation, Task, Action, Result, Learnings, Tags, Related Links).
   - Prevents rich interview answers (trade-off decisions, outage post-mortems, conflict resolution) from being lost into single bullet lines.

3. **First-Class Impact Metrics & KPIs**:
   - Structured `impact_metrics` on roles and projects (e.g. `latency_p99_reduction: "-38%"`, `throughput: "1.8M req/s"`, `daily_volume: "$2B+"`) to power quantifiable, high-impact resume generation.

4. **Atomic State I/O & Dual-Format Changelog**:
   - Safe in-place file mutation via temporary file renaming to prevent state corruption during interruptions.
   - `--changelog-dir <DIR>` records both machine-readable `.json` diff events and scannable `.md` narrative logs with `--message` context.

5. **Downstream Interoperability with `cv-writer`**:
   - `profile-curator export --state profile.json --output cv.json` outputs the exact schema required by `../cv-writer`.

6. **Optional MCP Server Compatibility**:
   - `profile-curator serve-mcp` runs the stdio JSON-RPC MCP server for compatibility with MCP clients.

---

## CLI Workflow for AI Agents & Users

```bash
# 1. Initialize profile (scaffold or realistic sample)
profile-curator init --state profile.json
# or bootstrap with sample:
profile-curator init --sample --output profile.json

# 2. Get conversational guidance on what questions to ask next
profile-curator guide --state profile.json

# 3. Patch profile incrementally as new details are learned
profile-curator patch \
  --state profile.json \
  --patch-json '{"contact": {"name": "Alex Chen"}, "career_orientation": {"add_target_roles": ["AI Engineer"]}}' \
  --changelog-dir ./history \
  --message "Discovered candidate name and target role"

# 4. Add a rich STAR interview story
profile-curator patch \
  --state profile.json \
  --patch-json '{
    "add_stories": [{
      "id": "story-cache-scaling",
      "title": "Scaling Distributed Cache",
      "situation": "Contention during peak Black Friday load",
      "task": "Rebalance hash partitions dynamically",
      "action": "Implemented consistent hashing with virtual nodes in Rust",
      "result": "Zero hotspots and 40% memory rebalance efficiency",
      "tags": ["distributed-systems", "rust", "caching"]
    }]
  }' \
  --changelog-dir ./history \
  --message "Recorded distributed cache scaling STAR story"

# 5. View changelog and session audit history
profile-curator history --changelog-dir ./history

# 6. Validate profile completeness
profile-curator validate --state profile.json

# 7. Export to cv-writer for PDF resume compilation
profile-curator export --state profile.json --output cv_profile.json
```

---

## Subcommand Reference

| Subcommand | Purpose |
| :--- | :--- |
| `init` | Create an empty or sample CuratedProfile scaffold. |
| `guide` | Calculate readiness score, detect gaps, and suggest next conversational questions. |
| `patch` | Apply incremental updates, atomically save state, and emit dual `.json` + `.md` changelog records. |
| `validate` | Inspect completeness score (0-100), critical missing fields, and section coverage. |
| `export` | Transform curated profile into `cv-writer`'s exact format. |
| `history` | List and review past changelog narrative entries. |
| `schema` | Dump JSON schemas for `profile`, `patch`, `cv_writer`, or `all`. |
| `serve-mcp`| Launch the stdio JSON-RPC MCP server. |

---

## Quality Assurance & Verification

```bash
# Run unit & integration tests (including CLI process testing)
cargo test

# Enforce strict zero-warning standard
cargo clippy --all-targets -- -D warnings
```
