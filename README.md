# Profile Curator MCP Server

A stateless, decoupled Model Context Protocol (MCP) server written in Rust that enables conversational AI agents to iteratively interview candidates, incrementally curate their professional background and future career orientation, and export directly to `cv-writer`.

## Key Capabilities

1. **Iterative Conversational Curation**:
   - The AI agent chats naturally with a candidate and invokes `patch_profile` incrementally as new details are learned.
2. **Comprehensive Profile Model**:
   - **Professional Background**: Education, work experiences/companies, personal/open-source projects, skills (categorized), certificates, spoken/written languages, awards, publications, and contact info.
   - **Future Career Orientation**: Target roles (e.g. AI Engineer, Backend Engineer, Platform Engineer), preferred work locations/cities, target domains/industries, target seniority levels, work arrangements, and timelines.
3. **Stateless & Decoupled Architecture**:
   - Zero database or Redis dependencies. State is passed by the caller and returned in tool responses, ensuring complete isolation and ephemeral horizontal scaling.
4. **Seamless Downstream Interoperability with `cv-writer`**:
   - Provides an `export_to_cv_writer` tool that transforms a `CuratedProfile` directly into the exact `CvProfile` JSON schema accepted by `../cv-writer`'s `render_cv` tool.

## Exposed MCP Tools

| Tool Name | Purpose |
| :--- | :--- |
| `get_profile_schema` | Returns full JSON schema for `CuratedProfile`, `ProfilePatch`, and `CvWriterProfile`. |
| `create_empty_profile` | Scaffolds a clean empty profile object to start an interview session. |
| `get_sample_profile` | Returns a complete sample profile with both background and career orientation. |
| `patch_profile` | Incrementally adds, updates, or removes sections/fields, returning updated profile + readable change diff. |
| `validate_profile` | Analyzes profile completeness, critical gaps, and section coverage. |
| `recommend_next_questions` | Returns targeted conversational questions for the agent to ask the user next. |
| `export_to_cv_writer` | Transforms a `CuratedProfile` into `cv-writer`'s exact input format for PDF compilation. |

## Quickstart

### Build and Run with Cargo
```bash
cargo build --release
./target/release/profile-curator-mcp
```

### Run with Docker
```bash
docker build -t profile-curator-mcp .
docker run -i --rm profile-curator-mcp
```

### Quality Assurance
```bash
cargo test
cargo clippy --all-targets -- -D warnings
```
