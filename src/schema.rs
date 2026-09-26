use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Canonical curated user profile combining professional background and future career orientation.
/// Statelessly passed in and returned by the MCP tools.
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct CuratedProfile {
    /// Personal contact details.
    pub contact: ContactInfo,
    /// Professional background history.
    pub background: ProfessionalBackground,
    /// Forward-looking career orientation and job hunt targets.
    pub career_orientation: CareerOrientation,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct ContactInfo {
    /// Full name of the candidate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Contact phone number.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// Primary email address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// GitHub profile username or handle.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub github: Option<String>,
    /// LinkedIn profile handle or URL slug.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linkedin: Option<String>,
    /// Personal portfolio, blog, or website URL.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    /// Current residence city / location (e.g. "Ho Chi Minh City, Vietnam").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_location: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct ProfessionalBackground {
    /// Brief executive summary or bio.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// Formal degrees, diplomas, or university education.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub education: Vec<EducationItem>,
    /// Past work experience grouped by company and positions held.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub experience: Vec<CompanyExperience>,
    /// Engineering, academic, or personal projects.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub projects: Vec<ProjectItem>,
    /// Professional skills categorized (e.g. Backend, Cloud, ML, Languages).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub skills: Vec<SkillCategory>,
    /// Professional certifications, licenses, or bootcamps.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub certifications: Vec<CertificationItem>,
    /// Spoken and written natural languages with proficiency.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub languages: Vec<LanguageItem>,
    /// Academic publications, research articles, or technical papers.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub publications: Vec<PublicationItem>,
    /// Honors, awards, hackathons, or formal recognitions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub awards: Vec<AwardItem>,
    /// Interview vault / story bank of rich behavioral & technical experiences (e.g. STAR format).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stories: Vec<StoryItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct EducationItem {
    /// Unique identifier or slug (e.g. "edu-stanford-bs").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Institution or university name.
    pub institution: String,
    /// Degree or major (e.g. "B.S. in Computer Science").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degree: Option<String>,
    /// Time period or graduation year (e.g. "2019 -- 2023").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dates: Option<String>,
    /// Bullet points for honors, GPA, relevant courses, or activities.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub highlights: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct CompanyExperience {
    /// Unique identifier or slug (e.g. "comp-google").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Organization or company name (e.g. "Google", "VNG").
    pub company: String,
    /// Location of employment (e.g. "Singapore", "Remote").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// Roles held at this company (hierarchical progression).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub roles: Vec<RoleItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct RoleItem {
    /// Unique identifier or slug (e.g. "role-sr-be").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Position or job title (e.g. "Senior Backend Engineer").
    pub title: String,
    /// Duration of this role (e.g. "Jan 2022 -- Present").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dates: Option<String>,
    /// Bullet points of achievements, metrics, and responsibilities.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub highlights: Vec<String>,
    /// First-class quantified impact metrics & KPIs (e.g. {"latency_reduction": "38%", "daily_volume": "$2B+"}).
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub impact_metrics: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct ProjectItem {
    /// Unique identifier or slug.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Project name.
    pub name: String,
    /// URL / repository link.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Date or duration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dates: Option<String>,
    /// Technologies used (e.g. ["Rust", "Docker", "LuaLaTeX"]).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub technologies: Vec<String>,
    /// Key outcomes, impact, and architectural descriptions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub highlights: Vec<String>,
    /// First-class quantified impact metrics & KPIs (e.g. {"throughput": "1.8M req/s", "core_utilization": "95%"}).
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub impact_metrics: std::collections::HashMap<String, String>,
}

/// Rich behavioral or technical story for interview preparation and battle-tested responses.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct StoryItem {
    /// Unique identifier or slug (e.g. "story-payout-outage-recovery").
    pub id: String,
    /// Short descriptive title of the experience or scenario.
    pub title: String,
    /// Situation: Context, company, scope, or initial challenge.
    pub situation: String,
    /// Task: Specific responsibility or objective expected.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task: Option<String>,
    /// Action: Technical and interpersonal steps taken, decisions, trade-offs.
    pub action: String,
    /// Result: Quantified impact, delivery outcome, business result.
    pub result: String,
    /// Lessons learned, retrospectives, or failure recovery takeaways.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub learnings: Option<String>,
    /// Categorization tags (e.g. ["conflict-resolution", "scalability", "failure-recovery", "rust"]).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    /// Optional foreign key to related company or project ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related_experience_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct SkillCategory {
    /// Category name (e.g. "Backend & Systems", "AI / ML", "Languages", "Cloud & DevOps").
    pub category: String,
    /// List of skills or technologies in this category.
    pub items: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct CertificationItem {
    /// Unique identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Name of the certification or credential.
    pub name: String,
    /// Issuing organization (e.g. "AWS", "Google Cloud", "Coursera").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    /// Issue date or validity period.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// Verification link or credential ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct LanguageItem {
    /// Natural language name (e.g. "English", "Vietnamese", "Japanese").
    pub language: String,
    /// Proficiency level (e.g. "Native", "Fluent / Professional", "Conversational", "C1").
    pub proficiency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct PublicationItem {
    /// Full citation or paper title.
    pub citation: String,
    /// URL to paper (DOI, arXiv, IEEE).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct AwardItem {
    /// Award or recognition title.
    pub title: String,
    /// Date or year received.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// Brief context, organizer, or competition ranking.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
}

/// Future Career Orientation defining what the candidate is actively searching for.
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct CareerOrientation {
    /// Target seniority levels (e.g. ["Senior", "Staff", "Lead", "Principal"]).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub target_seniorities: Vec<String>,
    /// Preferred work cities / locations (e.g. ["San Francisco, CA", "Singapore", "Tokyo", "Remote"]).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub preferred_locations: Vec<String>,
    /// Preferred target industries or domains (e.g. ["AI / LLM Infrastructure", "FinTech", "Autonomous Systems", "SaaS"]).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub target_domains: Vec<String>,
    /// Target job titles or functional roles (e.g. ["AI Engineer", "Backend Engineer", "MLOps Engineer"]).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub target_roles: Vec<String>,
    /// Work arrangement preferences (e.g. ["Remote", "Hybrid", "On-site"]).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub work_arrangements: Vec<String>,
    /// Target timeline for starting (e.g. "Immediate", "Within 1-2 months", "Q3 2026").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_timeline: Option<String>,
    /// Additional orientation notes or career aspirations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Sample curated profile for bootstrapping, testing, or demonstrations.
pub fn sample_curated_profile() -> CuratedProfile {
    CuratedProfile {
        contact: ContactInfo {
            name: Some("Alex Chen".to_string()),
            phone: Some("+1 (555) 234-5678".to_string()),
            email: Some("alex.chen@example.com".to_string()),
            github: Some("alexchen".to_string()),
            linkedin: Some("alex-chen-dev".to_string()),
            website: Some("https://alexchen.dev".to_string()),
            current_location: Some("San Francisco, CA".to_string()),
        },
        background: ProfessionalBackground {
            summary: Some("Senior Systems and Backend Engineer with 7+ years of experience designing high-throughput distributed services and high-reliability data pipelines in Rust and Go.".to_string()),
            education: vec![
                EducationItem {
                    id: Some("edu-ucb".to_string()),
                    institution: "University of California, Berkeley".to_string(),
                    degree: Some("B.S. in Electrical Engineering & Computer Sciences".to_string()),
                    dates: Some("2015 -- 2019".to_string()),
                    highlights: vec![
                        "Graduated with Honors".to_string(),
                        "Teaching Assistant for Operating Systems (CS 162)".to_string(),
                    ],
                }
            ],
            experience: vec![
                CompanyExperience {
                    id: Some("comp-stripe".to_string()),
                    company: "Stripe".to_string(),
                    location: Some("San Francisco, CA".to_string()),
                    roles: vec![
                        RoleItem {
                            id: Some("role-stripe-sr".to_string()),
                            title: "Senior Backend Engineer".to_string(),
                            dates: Some("Mar 2022 -- Present".to_string()),
                            highlights: vec![
                                "Architected global payout settlement engine handling $2B+ daily volume with 99.999% availability.".to_string(),
                                "Led Rust migration for high-performance edge transaction routing, reducing p99 latency by 38%.".to_string(),
                            ],
                            impact_metrics: [
                                ("daily_settlement_volume".to_string(), "$2B+".to_string()),
                                ("p99_latency_reduction".to_string(), "38%".to_string()),
                                ("uptime_sla".to_string(), "99.999%".to_string()),
                            ]
                            .into_iter()
                            .collect(),
                        }
                    ],
                }
            ],
            projects: vec![
                ProjectItem {
                    id: Some("proj-turborpc".to_string()),
                    name: "TurboRPC".to_string(),
                    url: Some("https://github.com/alexchen/turborpc".to_string()),
                    dates: Some("2024".to_string()),
                    technologies: vec!["Rust".to_string(), "Tokio".to_string(), "SIMD".to_string()],
                    highlights: vec![
                        "Zero-copy RPC protocol over io_uring achieving 1.8M req/s per core.".to_string(),
                    ],
                    impact_metrics: [
                        ("throughput_per_core".to_string(), "1.8M req/s".to_string()),
                    ]
                    .into_iter()
                    .collect(),
                }
            ],
            skills: vec![
                SkillCategory {
                    category: "Languages".to_string(),
                    items: vec!["Rust".to_string(), "Go".to_string(), "Python".to_string(), "SQL".to_string()],
                },
                SkillCategory {
                    category: "Distributed Systems & Cloud".to_string(),
                    items: vec!["Kubernetes".to_string(), "Kafka".to_string(), "AWS".to_string(), "PostgreSQL".to_string()],
                }
            ],
            certifications: vec![
                CertificationItem {
                    id: Some("cert-aws-sap".to_string()),
                    name: "AWS Certified Solutions Architect - Professional".to_string(),
                    issuer: Some("Amazon Web Services".to_string()),
                    date: Some("2023".to_string()),
                    url: None,
                }
            ],
            languages: vec![
                LanguageItem {
                    language: "English".to_string(),
                    proficiency: "Native / Bilingual".to_string(),
                },
                LanguageItem {
                    language: "Mandarin".to_string(),
                    proficiency: "Conversational".to_string(),
                }
            ],
            publications: vec![],
            awards: vec![
                AwardItem {
                    title: "First Place - Global FinTech Hackathon".to_string(),
                    date: Some("2023".to_string()),
                    summary: Some("Built a decentralized liquidity arbitration engine in Rust.".to_string()),
                }
            ],
            stories: vec![
                StoryItem {
                    id: "story-payout-settlement-migration".to_string(),
                    title: "Zero-Downtime Settlement Engine Migration".to_string(),
                    situation: "Legacy settlement engine faced frequent timeouts during Black Friday peak transaction surges, risking payment delays.".to_string(),
                    task: Some("Migrate transaction state reconciliation to an event-driven Rust service without downtime or double-settlement risk.".to_string()),
                    action: "Implemented a shadow-dual-write mechanism with idempotent deduplication keys in RocksDB, followed by canary traffic shifting over 3 weeks.".to_string(),
                    result: "Achieved zero double-settlements across $2B+ daily volume, reduced p99 reconciliation latency by 38%, and survived peak Black Friday load at 100% SLA.".to_string(),
                    learnings: Some("Shadow-running with real-time discrepancy alerting caught 3 edge-case currency rounding quirks before cutover.".to_string()),
                    tags: vec!["distributed-systems".to_string(), "high-throughput".to_string(), "zero-downtime".to_string(), "rust".to_string()],
                    related_experience_id: Some("comp-stripe".to_string()),
                }
            ],
        },
        career_orientation: CareerOrientation {
            target_seniorities: vec!["Senior".to_string(), "Staff Engineer".to_string()],
            preferred_locations: vec!["San Francisco, CA".to_string(), "Seattle, WA".to_string(), "Remote".to_string()],
            target_domains: vec!["AI Infrastructure".to_string(), "Distributed Systems".to_string(), "FinTech".to_string()],
            target_roles: vec!["AI Engineer".to_string(), "Backend Engineer".to_string(), "Infrastructure Engineer".to_string()],
            work_arrangements: vec!["Remote".to_string(), "Hybrid".to_string()],
            target_timeline: Some("Within 2 months".to_string()),
            notes: Some("Interested in teams building high-scale compute infrastructure for LLMs or modern financial rails.".to_string()),
        },
    }
}
