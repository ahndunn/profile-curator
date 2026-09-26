use std::path::PathBuf;
use clap::{Args, Parser, Subcommand, ValueEnum};
use schemars::schema_for;
use serde_json::json;

use crate::changelog::ChangelogManager;
use crate::export::{export_profile_for_cv_writer, CvWriterProfile};
use crate::io::{read_profile_state, write_profile_state};
use crate::patch::{apply_patch, ProfilePatch};
use crate::schema::{sample_curated_profile, CuratedProfile};
use crate::server::Server;
use crate::validation::{generate_next_questions, validate_profile};

/// CLI Engine for iterative interview preparation, career curation, and resume export.
///
/// Designed for both direct human execution and seamless orchestration by conversational AI agents.
/// AI agents can iteratively inspect, validate, patch, and guide candidate profile construction
/// without needing persistent database orchestration.
#[derive(Parser, Debug)]
#[command(
    name = "profile-curator",
    author = "ahndunn@gmail.com",
    version = env!("CARGO_PKG_VERSION"),
    about = "Conversational profile curation CLI engine with interview story vault and cv-writer export",
    after_help = "\
WORKFLOW FOR AI AGENTS (Iterative Curation):
  1. Initialize or inspect profile:
     $ profile-curator init --state profile.json
  2. Ask targeted questions to candidate:
     $ profile-curator guide --state profile.json
  3. Incrementally update profile with answers:
     $ profile-curator patch --state profile.json --patch-json '{\"summary\":\"...\"}' --changelog-dir ./history --message \"Added bio\"
  4. Review past session changelogs:
     $ profile-curator history --changelog-dir ./history
  5. Validate readiness for job hunting:
     $ profile-curator validate --state profile.json
  6. Export to resume compiler:
     $ profile-curator export --state profile.json --output cv_profile.json
"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize a new CuratedProfile scaffold (blank or sample).
    Init(InitArgs),

    /// Analyze a profile, calculate readiness, and suggest next conversational questions for the agent to ask.
    Guide(GuideArgs),

    /// Incrementally patch a profile with newly discovered information and log audit diffs.
    Patch(PatchArgs),

    /// Validate profile completeness, highlight critical gaps, and check section coverage.
    Validate(ValidateArgs),

    /// Export the curated profile into the exact input schema consumed by cv-writer (render_cv).
    Export(ExportArgs),

    /// Display past changelog entries and session narratives from the changelog directory.
    History(HistoryArgs),

    /// Print the JSON Schema for CuratedProfile, ProfilePatch, or CvWriterProfile.
    Schema(SchemaArgs),

    /// Run the legacy/stdio MCP server loop for JSON-RPC MCP clients (Claude Desktop, etc.).
    ServeMcp,
}

#[derive(Args, Debug)]
pub struct InitArgs {
    /// Destination state filepath to write initialized profile to (default: stdout).
    #[arg(short, long, default_value = "-")]
    pub output: PathBuf,

    /// Bootstrap with a realistic, comprehensive sample profile rather than an empty scaffold.
    #[arg(long)]
    pub sample: bool,
}

#[derive(Args, Debug)]
pub struct GuideArgs {
    /// Path to current profile state JSON file (use '-' for stdin).
    #[arg(short, long)]
    pub state: PathBuf,

    /// Output full guide diagnostics as JSON instead of human-friendly text.
    #[arg(long)]
    pub json: bool,
}

#[derive(Args, Debug)]
pub struct PatchArgs {
    /// Path to current profile state JSON file (use '-' for stdin).
    #[arg(short, long)]
    pub state: PathBuf,

    /// Inline JSON string containing the ProfilePatch object.
    #[arg(long, conflicts_with = "patch_file")]
    pub patch_json: Option<String>,

    /// Path to a JSON file containing the ProfilePatch object.
    #[arg(long, conflicts_with = "patch_json")]
    pub patch_file: Option<PathBuf>,

    /// Destination path to write updated profile (defaults to overwriting --state in-place, or stdout if state is '-').
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Directory to record timestamped .json and .md change entries for audit history.
    #[arg(long)]
    pub changelog_dir: Option<PathBuf>,

    /// Rationale or note describing what was updated in this turn (saved to changelog).
    #[arg(short, long, default_value = "Incremental profile update")]
    pub message: String,

    /// Suppress stderr next-step suggestions and changelog notifications.
    #[arg(short, long)]
    pub quiet: bool,
}

#[derive(Args, Debug)]
pub struct ValidateArgs {
    /// Path to profile state JSON file (use '-' for stdin).
    #[arg(short, long)]
    pub state: PathBuf,

    /// Output report in JSON format instead of human-readable text.
    #[arg(long)]
    pub json: bool,
}

#[derive(Args, Debug)]
pub struct ExportArgs {
    /// Path to curated profile state JSON file (use '-' for stdin).
    #[arg(short, long)]
    pub state: PathBuf,

    /// Destination path for exported cv-writer JSON (default: stdout).
    #[arg(short, long, default_value = "-")]
    pub output: PathBuf,
}

#[derive(Args, Debug)]
pub struct HistoryArgs {
    /// Directory containing changelog entries.
    #[arg(short, long, default_value = "./changelogs")]
    pub changelog_dir: PathBuf,
}

#[derive(Args, Debug)]
pub struct SchemaArgs {
    /// Which schema to output.
    #[arg(value_enum, default_value = "profile")]
    pub schema_type: SchemaType,
}

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum SchemaType {
    Profile,
    Patch,
    CvWriter,
    All,
}

impl Cli {
    pub async fn execute(self) -> Result<(), Box<dyn std::error::Error>> {
        match self.command {
            None => {
                // Default behavior when invoked without arguments: launch MCP server stdio loop
                let server = Server::new();
                server.run_stdio().await?;
            }
            Some(Commands::ServeMcp) => {
                let server = Server::new();
                server.run_stdio().await?;
            }
            Some(Commands::Init(args)) => {
                let profile = if args.sample {
                    sample_curated_profile()
                } else {
                    CuratedProfile::default()
                };
                write_profile_state(&args.output, &profile)?;
                if args.output != std::path::Path::new("-") {
                    eprintln!("✅ Initialized profile scaffold at '{}'", args.output.display());
                    eprintln!("💡 Next step for AI agent: Run `profile-curator guide --state {}` to determine what questions to ask.", args.output.display());
                }
            }
            Some(Commands::Guide(args)) => {
                let profile = read_profile_state(&args.state)?;
                let report = validate_profile(&profile);
                let questions = generate_next_questions(&profile);

                if args.json {
                    let output = json!({
                        "readiness_score": report.completeness_score,
                        "critical_gaps": report.critical_gaps,
                        "recommendations": report.recommendations,
                        "suggested_questions": questions,
                        "section_coverage": report.section_coverage,
                    });
                    println!("{}", serde_json::to_string_pretty(&output)?);
                } else {
                    println!("═══════════════════════════════════════════════════════════");
                    println!(" 🎯 CANDIDATE PROFILE READINESS & INTERVIEW GUIDANCE");
                    println!("═══════════════════════════════════════════════════════════");
                    println!("• Completeness Score: {} / 100", report.completeness_score);
                    println!();

                    if !report.critical_gaps.is_empty() {
                        println!("⚠️ Critical Missing Elements:");
                        for gap in &report.critical_gaps {
                            println!("  - {}", gap);
                        }
                        println!();
                    }

                    if !report.recommendations.is_empty() {
                        println!("💡 Recommended Improvements:");
                        for rec in &report.recommendations {
                            println!("  - {}", rec);
                        }
                        println!();
                    }

                    println!("💬 Suggested Next Questions for Interview Agent:");
                    for (i, q) in questions.iter().enumerate() {
                        println!("  {}. {}", i + 1, q);
                    }
                    println!("═══════════════════════════════════════════════════════════");
                }
            }
            Some(Commands::Patch(args)) => {
                let profile = read_profile_state(&args.state)?;

                let patch_raw = if let Some(ref json_str) = args.patch_json {
                    json_str.clone()
                } else if let Some(ref path) = args.patch_file {
                    std::fs::read_to_string(path)?
                } else {
                    return Err("Error: Must provide either --patch-json or --patch-file".into());
                };

                let patch: ProfilePatch = serde_json::from_str(&patch_raw)
                    .map_err(|e| format!("Failed to parse patch JSON: {}", e))?;

                let (updated_profile, changes) = apply_patch(profile, patch.clone());

                // Save to changelog if requested
                if let Some(ref cl_dir) = args.changelog_dir {
                    let cl = ChangelogManager::new(cl_dir);
                    let (json_log, md_log) = cl.record_change(&args.message, &changes, &patch)?;
                    if !args.quiet {
                        eprintln!("📝 Changelog recorded: {} and {}", json_log.display(), md_log.display());
                    }
                }

                // Determine destination
                let out_path = args.output.unwrap_or_else(|| args.state.clone());
                write_profile_state(&out_path, &updated_profile)?;

                if !args.quiet && out_path != std::path::Path::new("-") {
                    eprintln!("✅ Profile updated successfully at '{}'", out_path.display());
                    eprintln!("📋 Applied changes ({}):", changes.len());
                    for ch in &changes {
                        eprintln!("  • {}", ch);
                    }
                    eprintln!();
                    let next_q = generate_next_questions(&updated_profile);
                    if let Some(first) = next_q.first() {
                        eprintln!("💡 Suggested next question: \"{}\"", first);
                    }
                }
            }
            Some(Commands::Validate(args)) => {
                let profile = read_profile_state(&args.state)?;
                let report = validate_profile(&profile);

                if args.json {
                    println!("{}", serde_json::to_string_pretty(&report)?);
                } else {
                    println!("=== Profile Validation Report ===");
                    println!("Completeness Score: {} / 100", report.completeness_score);
                    if report.critical_gaps.is_empty() {
                        println!("Critical Gaps: None ✅");
                    } else {
                        println!("Critical Gaps:");
                        for gap in &report.critical_gaps {
                            println!("  - [CRITICAL] {}", gap);
                        }
                    }
                    if !report.recommendations.is_empty() {
                        println!("Recommendations:");
                        for rec in &report.recommendations {
                            println!("  - [INFO] {}", rec);
                        }
                    }
                }
            }
            Some(Commands::Export(args)) => {
                let profile = read_profile_state(&args.state)?;
                let cv_profile = export_profile_for_cv_writer(&profile);
                let serialized = serde_json::to_string_pretty(&json!({
                    "cv_profile": cv_profile
                }))?;

                if args.output == std::path::Path::new("-") {
                    println!("{}", serialized);
                } else {
                    std::fs::write(&args.output, serialized)?;
                    eprintln!("✅ Exported CV profile for cv-writer at '{}'", args.output.display());
                }
            }
            Some(Commands::History(args)) => {
                let cl = ChangelogManager::new(&args.changelog_dir);
                let history = cl.list_history()?;
                if history.is_empty() {
                    println!("No changelog records found in '{}'", args.changelog_dir.display());
                } else {
                    println!("=== Profile Change History ({} entries) ===", history.len());
                    for (file, content) in history {
                        println!("--- {} ---", file);
                        println!("{}", content);
                    }
                }
            }
            Some(Commands::Schema(args)) => match args.schema_type {
                SchemaType::Profile => {
                    let schema = schema_for!(CuratedProfile);
                    println!("{}", serde_json::to_string_pretty(&schema)?);
                }
                SchemaType::Patch => {
                    let schema = schema_for!(ProfilePatch);
                    println!("{}", serde_json::to_string_pretty(&schema)?);
                }
                SchemaType::CvWriter => {
                    let schema = schema_for!(CvWriterProfile);
                    println!("{}", serde_json::to_string_pretty(&schema)?);
                }
                SchemaType::All => {
                    let schema = json!({
                        "curated_profile": schema_for!(CuratedProfile),
                        "profile_patch": schema_for!(ProfilePatch),
                        "cv_writer_profile": schema_for!(CvWriterProfile),
                    });
                    println!("{}", serde_json::to_string_pretty(&schema)?);
                }
            },
        }

        Ok(())
    }
}
