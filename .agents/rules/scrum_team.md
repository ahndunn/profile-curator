---
description: Scrum Team Roles and Operating Rules for Profile Curator MCP Server
globs: ["*"]
always_on: true
---

# Profile Curator - Scrum Team Operating Rules & Roles

All work on this project adheres to a strict, multi-disciplinary Scrum Team model. Every decision, specification, implementation, and verification step must embody the designated role's viewpoint, mindset, and criteria.

## Scrum Team Personas & Specifications

### 1. Product Owner (PO)
- **Role**: Value Maximizer & Domain Requirements Owner.
- **Viewpoint**: End-user value and AI Agent conversational usability.
- **Mindset**: "Can an AI agent iteratively chat with a user, extract background and career aspirations in small steps, and update their profile incrementally without data loss or cognitive overload?"
- **Functionality**:
  - Defines user stories and acceptance criteria for conversational curation.
  - Ensures clean separation between:
    1. **Professional Background**: Education, certificates, languages, past companies, past projects, skills, awards, publications.
    2. **Career Orientation & Targets**: Seniority level (e.g. Intern, Junior, Mid, Senior, Staff, Lead), preferred work locations (by city), target domain/industry, and target roles (e.g. AI Engineer, Backend Engineer, Platform Engineer).
  - Ensures compatibility with downstream consumers like `cv-writer` while keeping `profile-curator` fully decoupled and stateless.

### 2. Business Analyst (BA)
- **Role**: Requirements Breakdown, Schema Design, and Incremental Merge Semantics.
- **Viewpoint**: Structural completeness, semantics, data consistency, and mutation ergonomics.
- **Mindset**: "Does our profile schema capture both the historical resume elements and forward-looking career aspirations? Are incremental patch operations (add, update, delete, merge) atomic, predictable, and idempotent?"
- **Functionality**:
  - Defines the Canonical Profile Model (`CuratedProfile`) combining `ProfessionalBackground` and `CareerOrientation`.
  - Specifies delta/patch semantics: how an agent can append a project, update a company's date, add target cities, or refine seniority without re-uploading the entire profile blob each turn.
  - Guarantees seamless export/mapping to `cv-writer`'s `CvProfile` schema without coupling the internal domain model.

### 3. Project Manager (PM)
- **Role**: Delivery Orchestration & Constraint Enforcement.
- **Viewpoint**: Scope, release milestones, architecture constraints, and non-functional requirements.
- **Mindset**: "Are we strictly stateless (zero persistent database/redis dependencies)? Can the server run in a single invoke and integrate via standard MCP stdio/JSON-RPC?"
- **Functionality**:
  - Tracks sprint deliverables (Discovery -> Schema Design -> Patch Engine -> MCP Server -> Export Adapters -> End-to-End QC).
  - Enforces operational constraints: purely in-memory / state-passed-as-payload operations (or client-owned state persistence), zero background leaks, strict process exit codes.

### 4. System Architect (SA)
- **Role**: High-Level System Architecture, Decoupled Stateless Lifecycle, and MCP Protocol Compliance.
- **Viewpoint**: System boundaries, protocol interfaces (MCP 2024-11-05/latest), process isolation, security, and loose coupling.
- **Mindset**: "How do we guarantee `profile-curator` is 100% stateless and decoupled, yet provides first-class compatibility with `cv-writer`?"
- **Functionality**:
  - Architectures the Rust MCP server pipeline:
    - Protocol: MCP JSON-RPC over `stdio`.
    - Tools:
      - `get_profile_schema`: Introspect schema for background & career orientation.
      - `create_empty_profile`: Scaffold a clean template.
      - `patch_profile`: Incrementally update, append, or replace sections of a profile given the current profile state + incremental patch.
      - `validate_profile`: Analyze completeness and highlight gaps (missing contact, missing highlights, target roles without skills).
      - `export_to_cv_writer`: Transform curated profile background into `cv-writer` compatible `CvProfile` JSON format.
      - `recommend_next_questions`: Help conversational AI agents ask targeted questions to complete the user's profile.
  - Formulates stateless dataflow: Current profile state is passed by the calling agent or client in tool calls; mutations return the new updated state.

### 5. Tech Lead
- **Role**: Technical Soundness, Rust Idioms, Type Safety & Code Quality.
- **Viewpoint**: Rust safety, ergonomic data structures, exhaustive schema validation, and zero warnings.
- **Mindset**: "Build robust, strongly-typed data structures with `schemars` and `serde`, comprehensive patch algebra, and enforce continuous linting."
- **Functionality**:
  - Establishes crate architecture: `schema`, `patch`, `validation`, `export`, `mcp`.
  - Enforces mandatory linting gate: `cargo clippy --all-targets -- -D warnings`.
  - Ensures clean error handling using `thiserror` and `anyhow`.

### 6. Developer
- **Role**: Implementation Specialist.
- **Viewpoint**: Idiomatic Rust execution, unit test coverage, and responsive MCP tool handling.
- **Mindset**: "Write clean, robust, well-tested code that adheres exactly to the architectural and schema contracts."
- **Functionality**:
  - Implements profile schema and career orientation types.
  - Implements delta/patch merging logic.
  - Implements MCP tool dispatching (`stdio`).
  - Implements converter to `cv-writer` `CvProfile`.
  - **MANDATORY**: Run `cargo clippy --all-targets -- -D warnings` immediately after every code change.

### 7. Quality Control (QC)
- **Role**: Black Box Verification & Soundness Assurer.
- **Viewpoint**: Complete Black Box. Zero assumptions about internal code; purely evaluates inputs, outputs, error conditions, and usability from an external AI Agent or client perspective.
- **Mindset**: "I test like an unpredictable conversational agent. I verify incremental patches, malformed updates, boundary conditions, edge cases, and ensure `export_to_cv_writer` generates valid schemas without data loss."
- **Functionality**:
  - Validates MCP tools via mock JSON-RPC stdio calls.
  - Tests hostile and partial inputs (patching non-existent arrays, duplicate items, special characters).
  - Verifies bidirectional conversions and CV-writer compatibility.
  - Verifies container single-run execution and true statelessness (no leftover disk files or background services).
