---
name: profile-curator-scrum-team
description: >-
  Orchestrates the Scrum Team workflow (PO, BA, PM, SA, Tech Lead, Developer, QC)
  for building and maintaining the stateless Rust MCP Profile Curator server.
---

# Profile Curator Scrum Team Skill

This skill defines the operational workflow, role checklists, and review gates for the Profile Curator MCP project.

## Operational Lifecycle

1. **Sprint Planning & Vision (PO + PM)**:
   - Establish sprint goals tailored for conversational, iterative profile building.
   - Guard statelessness: the server maintains zero state internally (no database, no redis, no file store); callers pass the current profile and receive the updated profile.
   - Enforce single-invoke Docker portability and CLI stdio integration.

2. **Analysis & Schema Definition (BA + PO)**:
   - Define canonical `CuratedProfile` schema:
     - **Professional Background**:
       - Personal / Contact Info (name, email, phone, location, github, linkedin, website).
       - Summary / Bio.
       - Education (institution, degree, dates, highlights).
       - Certificates (name, issuer, date, url).
       - Languages (language, proficiency/fluency).
       - Past Companies / Experience (company, location, roles: [title, dates, highlights]).
       - Past Projects (name, url, dates, highlights, technologies).
       - Skills (categorized: Technical, Languages, Frameworks, Platforms, etc.).
       - Awards & Publications.
     - **Career Orientation & Future Goals**:
       - Seniority Level (e.g. Intern, Junior, Mid-Level, Senior, Staff, Principal, Lead).
       - Target Work Locations (list of preferred cities/regions, e.g. ["San Francisco, CA", "Remote", "Tokyo, Japan"]).
       - Target Domains / Industries (e.g. ["FinTech", "HealthTech", "AI/ML Infrastructure", "Robotics"]).
       - Target Roles (e.g. ["AI Engineer", "Backend Engineer", "MLOps Engineer", "Engineering Manager"]).
       - Availability & Work Preference (e.g. full-time, contract, remote/hybrid/on-site).
   - Define incremental patch operations:
     - Append items to lists (education, experience, projects, skills, certificates, target locations, roles).
     - Update existing items by ID/index or matching keys.
     - Replace or remove items.
     - Upsert fields in contact or career orientation.

3. **Architectural & Interoperability Review (SA + Tech Lead)**:
   - Rust architecture: MCP stdio server handling JSON-RPC requests.
   - Decoupled Stateless Model:
     - Tools take `(current_profile, patch)` -> return `(updated_profile, diff_summary)`.
     - Tool `export_to_cv_writer`: takes `CuratedProfile` -> returns `CvProfile` matching `cv-writer` schema.
   - Conversational Assistance Tools:
     - `validate_profile`: detects gaps (e.g., missing contact details, missing accomplishments in past roles, vague career targets).
     - `recommend_next_questions`: returns prompt guidance and targeted follow-up questions for the AI agent to ask the user.

4. **Implementation (Developer)**:
   - Rust crates: `serde`, `serde_json`, `schemars`, `tokio`, `thiserror`, `tracing`.
   - MCP Tools:
     - `get_profile_schema`: returns full JSON schema of `CuratedProfile`.
     - `create_empty_profile`: returns blank scaffolded profile.
     - `patch_profile`: applies incremental mutations to profile.
     - `validate_profile`: returns completeness analysis and warning flags.
     - `export_to_cv_writer`: outputs JSON compliant with `../cv-writer`'s `render_cv` input format.
     - `get_sample_profile`: returns realistic curated profile.
     - `recommend_next_questions`: suggests conversational inquiries for missing data.
   - **Quality Gate**: Execute `cargo clippy --all-targets -- -D warnings` after every code change to guarantee zero warnings.

5. **Black Box Quality Control (QC)**:
   - Test MCP tool discovery (`tools/list`).
   - Test JSON Schema retrieval (`tools/call` `get_profile_schema`).
   - Test incremental patching flow: starting empty, iteratively adding education, experience, target roles, and cities.
   - Test export compatibility: verify exported output conforms exactly to `cv-writer`'s expected `CvProfile` structure.
   - Test robustness against hostile/malformed patches.
   - Verify zero persistent file leaks or background daemons.
