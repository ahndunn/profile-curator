# Profile Curator Project & Agent Instructions

This repository contains a stateless Model Context Protocol (MCP) server written in Rust that enables conversational AI Agents to iteratively interview users, capture their professional background and future career orientation, and incrementally update their profile via MCP tools.

## Key Capabilities & Goals
- **Iterative Chat Curation**: Designed to be driven by an AI agent chatting with a candidate, calling tools incrementally as information is uncovered.
- **Comprehensive Profile Model**:
  - **Professional Background**: Education, certificates, languages, past companies, past projects, skills, publications, awards.
  - **Future Career Orientation**: Target seniority levels, preferred work cities/locations, target industries/domains, target roles (e.g. AI Engineer, BE Engineer).
- **Stateless & Decoupled**: The server maintains zero persistent state (no DB, no Redis); calling agents pass state through tool arguments and receive updated states and diff summaries.
- **Seamless Downstream Interoperability**: Provides an explicit tool `export_to_cv_writer` that converts a curated profile into the exact input schema consumed by `../cv-writer`.

## Operating Guidelines

This project strictly operates under a 7-role Scrum Team:
- **Product Owner (PO)**: Conversational agent value, domain requirements, iterative user experience.
- **Business Analyst (BA)**: Canonical profile schema, incremental patch/merge algebra, validation rules.
- **Project Manager (PM)**: Stateless enforcement, single-invoke container packaging, sprint tracking.
- **System Architect (SA)**: Protocol compliance (MCP stdio), loose coupling, export interoperability.
- **Tech Lead**: Rust idioms, schema type soundness, zero-warning clippy standard.
- **Developer**: Code implementation, patch logic, MCP tool handlers. Must run `cargo clippy --all-targets -- -D warnings` after every change.
- **Quality Control (QC)**: Black-box testing from an agent's conversational perspective, verifying edge-case patching, validation diagnostics, and cv-writer export compatibility.

Refer to [`.agents/rules/scrum_team.md`](file:///home/ahndunn/dev/jobs-finder/profile-curator/.agents/rules/scrum_team.md) and [`.agents/skills/profile-curator-scrum-team/SKILL.md`](file:///home/ahndunn/dev/jobs-finder/profile-curator/.agents/skills/profile-curator-scrum-team/SKILL.md) for full role definitions and lifecycle procedures.
