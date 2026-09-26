use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::schema::CuratedProfile;

/// Diagnostic summary of profile completeness, missing elements, and readiness.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct ValidationReport {
    /// Overall score from 0 to 100 based on completeness.
    pub completeness_score: u32,
    /// Critical issues preventing high-quality export or representation.
    pub critical_gaps: Vec<String>,
    /// Helpful recommendations for things to improve.
    pub recommendations: Vec<String>,
    /// Flags indicating section coverage.
    pub section_coverage: SectionCoverage,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct SectionCoverage {
    pub has_contact_name: bool,
    pub has_contact_email_or_phone: bool,
    pub has_summary: bool,
    pub has_education: bool,
    pub has_experience: bool,
    pub has_projects: bool,
    pub has_skills: bool,
    pub has_stories: bool,
    pub has_impact_metrics: bool,
    pub has_target_roles: bool,
    pub has_target_locations: bool,
    pub has_target_seniorities: bool,
}

/// Evaluates a curated profile and returns actionable diagnostics.
pub fn validate_profile(profile: &CuratedProfile) -> ValidationReport {
    let mut score = 0u32;
    let mut critical_gaps = Vec::new();
    let mut recommendations = Vec::new();

    // 1. Contact Info (15 pts)
    let has_contact_name = profile.contact.name.as_ref().map(|s| !s.trim().is_empty()).unwrap_or(false);
    let has_contact_email = profile.contact.email.as_ref().map(|s| !s.trim().is_empty()).unwrap_or(false);
    let has_contact_phone = profile.contact.phone.as_ref().map(|s| !s.trim().is_empty()).unwrap_or(false);
    let has_contact_email_or_phone = has_contact_email || has_contact_phone;

    if has_contact_name {
        score += 8;
    } else {
        critical_gaps.push("Candidate name is missing.".to_string());
    }

    if has_contact_email_or_phone {
        score += 7;
    } else {
        critical_gaps.push("No contact channel (email or phone) provided.".to_string());
    }

    if profile.contact.github.is_none() && profile.contact.linkedin.is_none() {
        recommendations.push("Consider asking for a GitHub, LinkedIn, or personal website link.".to_string());
    }

    // 2. Summary (5 pts)
    let has_summary = profile.background.summary.as_ref().map(|s| !s.trim().is_empty()).unwrap_or(false);
    if has_summary {
        score += 5;
    } else {
        recommendations.push("Professional summary or objective statement is empty.".to_string());
    }

    // 3. Experience & Impact Metrics (25 pts)
    let has_experience = !profile.background.experience.is_empty();
    let mut total_metrics = 0;
    if has_experience {
        score += 15;
        let mut missing_highlights = 0;
        for exp in &profile.background.experience {
            for role in &exp.roles {
                if role.highlights.is_empty() {
                    missing_highlights += 1;
                }
                total_metrics += role.impact_metrics.len();
            }
        }
        if missing_highlights > 0 {
            recommendations.push(format!(
                "{missing_highlights} work position(s) lack achievement bullet points. Prompt candidate for quantifiable impact."
            ));
        }
    } else {
        critical_gaps.push("No work experience / company history recorded.".to_string());
    }

    for proj in &profile.background.projects {
        total_metrics += proj.impact_metrics.len();
    }
    let has_impact_metrics = total_metrics > 0;
    if has_impact_metrics {
        score += 10;
    } else {
        recommendations.push("No structured impact metrics / KPIs recorded. Quantifying results (e.g. latency reduction, revenue, throughput) strengthens CVs and interviews.".to_string());
    }

    // 4. Education & Projects (15 pts)
    let has_education = !profile.background.education.is_empty();
    let has_projects = !profile.background.projects.is_empty();

    if has_education {
        score += 8;
    } else {
        recommendations.push("Education history is missing.".to_string());
    }

    if has_projects {
        score += 7;
    } else {
        recommendations.push("No notable projects recorded. Personal or open-source projects strengthen technical profiles.".to_string());
    }

    // 5. Skills (10 pts)
    let total_skill_items: usize = profile.background.skills.iter().map(|s| s.items.len()).sum();
    let has_skills = total_skill_items > 0;
    if has_skills {
        score += 10;
    } else {
        critical_gaps.push("No technical or domain skills listed.".to_string());
    }

    // 6. Story Vault (15 pts)
    let has_stories = !profile.background.stories.is_empty();
    if has_stories {
        score += 15;
    } else {
        recommendations.push("Interview story vault is empty. Capturing STAR answers (challenges, actions, trade-offs) will prepare the candidate for behavioral and system design interviews.".to_string());
    }

    // 7. Career Orientation (15 pts)
    let has_target_roles = !profile.career_orientation.target_roles.is_empty();
    let has_target_locations = !profile.career_orientation.preferred_locations.is_empty();
    let has_target_seniorities = !profile.career_orientation.target_seniorities.is_empty();

    if has_target_roles {
        score += 7;
    } else {
        critical_gaps.push("Target roles (e.g. AI Engineer, Backend Engineer) are not defined.".to_string());
    }

    if has_target_locations {
        score += 4;
    } else {
        recommendations.push("Preferred work locations / cities are not set.".to_string());
    }

    if has_target_seniorities {
        score += 4;
    } else {
        recommendations.push("Target seniority level (e.g. Junior, Mid, Senior, Staff) is not set.".to_string());
    }

    ValidationReport {
        completeness_score: score.min(100),
        critical_gaps,
        recommendations,
        section_coverage: SectionCoverage {
            has_contact_name,
            has_contact_email_or_phone,
            has_summary,
            has_education,
            has_experience,
            has_projects,
            has_skills,
            has_stories,
            has_impact_metrics,
            has_target_roles,
            has_target_locations,
            has_target_seniorities,
        },
    }
}

/// Generates guided conversational prompts for the AI agent to ask the candidate next.
pub fn generate_next_questions(profile: &CuratedProfile) -> Vec<String> {
    let mut questions = Vec::new();

    if profile.contact.name.is_none() {
        questions.push("What is your full name?".to_string());
    }

    if profile.career_orientation.target_roles.is_empty() {
        questions.push("What target roles are you aiming for in your next career step (for example, AI Engineer, Backend Engineer, Platform Engineer)?".to_string());
    }

    if profile.career_orientation.target_seniorities.is_empty() {
        questions.push("What seniority level are you targeting (e.g., Mid-level, Senior, Staff, Lead)?".to_string());
    }

    if profile.career_orientation.preferred_locations.is_empty() {
        questions.push("Which cities or locations would you prefer to work in (e.g. San Francisco, Tokyo, Singapore, or Remote)?".to_string());
    }

    if profile.background.experience.is_empty() {
        questions.push("Could you tell me about your most recent company, your job title, and a few key achievements you had there?".to_string());
    }

    if profile.background.skills.is_empty() {
        questions.push("What are your primary programming languages, frameworks, or core technical skills?".to_string());
    }

    if profile.background.stories.is_empty() {
        questions.push("Can you share a challenging engineering challenge or outage recovery you led (situation, what you did, and quantifiable result)? We can save it into your interview story vault.".to_string());
    }

    if profile.background.education.is_empty() {
        questions.push("Where did you study, and what degree or major did you graduate with?".to_string());
    }

    if profile.career_orientation.target_domains.is_empty() {
        questions.push("Are there particular industry domains that excite you (such as FinTech, AI Infrastructure, Healthcare, or Autonomous Robotics)?".to_string());
    }

    if questions.is_empty() {
        questions.push("Your profile is comprehensively filled out! Would you like to review your interview stories, refine impact metrics, or export to a resume via cv-writer?".to_string());
    }

    questions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_profile_validation() {
        let empty = CuratedProfile::default();
        let report = validate_profile(&empty);
        assert!(report.completeness_score < 20);
        assert!(!report.critical_gaps.is_empty());
        let questions = generate_next_questions(&empty);
        assert!(!questions.is_empty());
    }
}
