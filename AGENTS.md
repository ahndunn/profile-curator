# Profile Curator Project & Agent Instructions

This repository contains `profile-curator`, an LLM-friendly CLI engine written in Rust that enables conversational AI Agents to iteratively interview candidates, capture rich background history (including an interview story vault and quantified impact metrics), maintain audit changelogs, and incrementally update candidate profiles.

## Key Capabilities & Goals
- **CLI-First with LLM Guidance**: Direct subcommands (`init`, `guide`, `patch`, `validate`, `export`, `history`, `schema`) designed for AI agents and scripts. The `--help` is self-describing, and commands emit next-step conversational prompts.
- **Interview Vault (`stories`)**: First-class STAR story repository (Situation, Task, Action, Result, Learnings, Tags) to preserve rich interview answers for mock interviews and prep.
- **Quantified Impact Metrics**: First-class structured KPI metrics on roles and projects to enable automated, punchy CV tailoring.
- **Atomic File-Backed State & Dual Changelogs**:
  - `--state <FILE>` reads and updates state safely via atomic temporary file renaming.
  - `--changelog-dir <DIR>` logs both machine-readable `.json` diff events and scannable `.md` narrative summaries with `--message` context.
- **Downstream Interoperability**: `profile-curator export --state profile.json` converts curated profiles into the exact input schema consumed by `../cv-writer`.
- **Stateless & Decoupled**: Operates in-memory and on explicit file paths without background daemons, databases, or Redis. An optional `serve-mcp` subcommand provides stdio MCP JSON-RPC compatibility.

## Operating Guidelines

This project strictly operates under a 7-role Scrum Team:
- **Product Owner (PO)**: Conversational agent value, domain requirements, iterative user experience.
- **Business Analyst (BA)**: Canonical profile schema, incremental patch/merge algebra, validation rules.
- **Project Manager (PM)**: Delivery orchestration, atomic file state enforcement, sprint tracking.
- **System Architect (SA)**: CLI interfaces, process isolation, changelog architecture, loose coupling, export interoperability.
- **Tech Lead**: Rust idioms, schema type soundness, zero-warning clippy standard.
- **Developer**: Code implementation, patch logic, CLI handlers. Must run `cargo clippy --all-targets -- -D warnings` after every change.
- **Quality Control (QC)**: Black-box testing from an agent's conversational perspective, verifying edge-case patching, validation diagnostics, and cv-writer export compatibility.

Refer to [`.agents/rules/scrum_team.md`](file:///home/ahndunn/dev/jobs-finder/profile-curator/.agents/rules/scrum_team.md) and [`.agents/skills/profile-curator-scrum-team/SKILL.md`](file:///home/ahndunn/dev/jobs-finder/profile-curator/.agents/skills/profile-curator-scrum-team/SKILL.md) for full role definitions and lifecycle procedures.

