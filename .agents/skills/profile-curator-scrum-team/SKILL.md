---
name: profile-curator-scrum-team
description: >-
  Orchestrates the Scrum Team workflow (PO, BA, PM, SA, Tech Lead, Developer, QC)
  for building and maintaining the LLM-friendly Rust CLI Profile Curator engine.
---

# Profile Curator Scrum Team Skill

This skill defines the operational workflow, role checklists, and review gates for the `profile-curator` CLI project.

## Operational Lifecycle

1. **Sprint Planning & Vision (PO + PM)**:
   - Establish sprint goals tailored for conversational, iterative profile building.
   - Enforce CLI-first ergonomics: subcommands (`init`, `guide`, `patch`, `validate`, `export`, `history`, `schema`, `serve-mcp`).
   - Guard state integrity: atomic file writing via temporary file renaming, zero background daemons.
   - Enforce dual-format audit logging (`--changelog-dir`) producing `.json` and `.md` logs.

2. **Analysis & Schema Definition (BA + PO)**:
   - Define canonical `CuratedProfile` schema:
     - **Professional Background**:
       - Personal / Contact Info (name, email, phone, location, github, linkedin, website).
       - Summary / Bio.
       - Education (institution, degree, dates, highlights).
       - Certificates (name, issuer, date, url).
       - Languages (language, proficiency/fluency).
       - Past Companies / Experience (company, location, roles: [title, dates, highlights, impact_metrics]).
       - Past Projects (name, url, dates, highlights, technologies, impact_metrics).
       - Skills (categorized: Technical, Languages, Frameworks, Platforms, etc.).
       - Publications & Awards.
       - **Interview Story Vault (`stories`)**: STAR format experiences (situation, task, action, result, learnings, tags, links).
     - **Career Orientation & Future Goals**:
       - Seniority Level (e.g. Intern, Junior, Mid-Level, Senior, Staff, Principal, Lead).
       - Target Work Locations (list of preferred cities/regions).
       - Target Domains / Industries (e.g. ["FinTech", "AI/ML Infrastructure"]).
       - Target Roles (e.g. ["AI Engineer", "Backend Engineer"]).
       - Availability & Work Preference (remote, hybrid, on-site).

3. **Architectural & Interoperability Review (SA + Tech Lead)**:
   - CLI architecture: `clap` derive parser, Unix piping (`-` for stdin/stdout), and atomic file writes.
   - Conversational Assistance:
     - `guide`: calculates completeness scores, detects gaps, and returns actionable conversational interview prompts.
     - `export`: transforms profile background into `cv-writer` compatible `CvProfile` JSON format.
     - `history`: scans changelog directory and displays chronologically ordered change narratives.

4. **Implementation (Developer)**:
   - Rust crates: `clap`, `chrono`, `serde`, `serde_json`, `schemars`, `tokio`, `thiserror`, `tracing`.
   - **Quality Gate**: Execute `cargo clippy --all-targets -- -D warnings` after every code change to guarantee zero warnings.

5. **Black Box Quality Control (QC)**:
   - Test CLI subcommands (`init`, `guide`, `patch`, `validate`, `export`, `history`).
   - Verify atomic file writing, in-place update, and dual-format changelog creation.
   - Test export compatibility: verify output conforms to `cv-writer`'s expected `CvProfile` structure.
   - Verify MCP backwards compatibility (`serve-mcp`).

