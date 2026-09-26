use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::schema::{
    AwardItem, CareerOrientation, CertificationItem, CompanyExperience,
    CuratedProfile, EducationItem, LanguageItem, ProjectItem, PublicationItem, SkillCategory, StoryItem,
};

/// High-level patch payload for updating a CuratedProfile.
/// Allows conversational AI agents to update fields incrementally without re-sending the whole profile.
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct ProfilePatch {
    /// Update or replace contact information fields.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact: Option<ContactInfoPatch>,
    /// Update professional background summary.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// Append new education items.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub add_education: Vec<EducationItem>,
    /// Update existing education item by ID or institution name.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub update_education: Vec<EducationItem>,
    /// Remove education items by ID or institution name.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub remove_education: Vec<String>,
    /// Append new company experiences.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub add_experience: Vec<CompanyExperience>,
    /// Update existing company experience by ID or company name.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub update_experience: Vec<CompanyExperience>,
    /// Remove company experience by ID or company name.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub remove_experience: Vec<String>,
    /// Append new projects.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub add_projects: Vec<ProjectItem>,
    /// Update existing projects by ID or project name.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub update_projects: Vec<ProjectItem>,
    /// Remove projects by ID or project name.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub remove_projects: Vec<String>,
    /// Upsert skills: merges skill items into existing categories or appends new categories.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub upsert_skills: Vec<SkillCategory>,
    /// Remove skills from a category, or remove entire category if items is empty.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub remove_skills: Vec<SkillCategory>,
    /// Append new certifications.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub add_certifications: Vec<CertificationItem>,
    /// Remove certifications by ID or certification name.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub remove_certifications: Vec<String>,
    /// Upsert spoken/written languages.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub upsert_languages: Vec<LanguageItem>,
    /// Remove languages by name.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub remove_languages: Vec<String>,
    /// Append new publications.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub add_publications: Vec<PublicationItem>,
    /// Append new awards.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub add_awards: Vec<AwardItem>,
    /// Append new interview stories / STAR experiences to the story vault.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub add_stories: Vec<StoryItem>,
    /// Update existing interview story by ID or title.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub update_stories: Vec<StoryItem>,
    /// Remove interview stories by ID or title.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub remove_stories: Vec<String>,
    /// Update future career orientation fields.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub career_orientation: Option<CareerOrientationPatch>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct ContactInfoPatch {
    pub name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub github: Option<String>,
    pub linkedin: Option<String>,
    pub website: Option<String>,
    pub current_location: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct CareerOrientationPatch {
    /// Add or merge target seniority levels (e.g. ["Staff Engineer"]).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub add_target_seniorities: Vec<String>,
    /// Add or merge preferred work locations by city (e.g. ["Tokyo", "Singapore"]).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub add_preferred_locations: Vec<String>,
    /// Add or merge target domains (e.g. ["Autonomous Driving", "FinTech"]).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub add_target_domains: Vec<String>,
    /// Add or merge target job titles (e.g. ["AI Engineer", "BE Engineer"]).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub add_target_roles: Vec<String>,
    /// Add or merge preferred work arrangements (e.g. ["Remote", "Hybrid"]).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub add_work_arrangements: Vec<String>,
    /// Set or update the target start timeline.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set_target_timeline: Option<String>,
    /// Set or update additional orientation notes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set_notes: Option<String>,
    /// Complete replacement of career orientation (if conversational agent wants clean reset).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replace_all: Option<CareerOrientation>,
}

/// Applies a patch to a profile, returning the modified profile and a readable diff summary.
pub fn apply_patch(mut profile: CuratedProfile, patch: ProfilePatch) -> (CuratedProfile, Vec<String>) {
    let mut changes = Vec::new();

    // 1. Contact Info
    if let Some(cp) = patch.contact {
        if let Some(name) = cp.name {
            profile.contact.name = Some(name.clone());
            changes.push(format!("Updated contact name to '{name}'"));
        }
        if let Some(phone) = cp.phone {
            profile.contact.phone = Some(phone.clone());
            changes.push(format!("Updated contact phone to '{phone}'"));
        }
        if let Some(email) = cp.email {
            profile.contact.email = Some(email.clone());
            changes.push(format!("Updated contact email to '{email}'"));
        }
        if let Some(github) = cp.github {
            profile.contact.github = Some(github.clone());
            changes.push(format!("Updated GitHub handle to '{github}'"));
        }
        if let Some(linkedin) = cp.linkedin {
            profile.contact.linkedin = Some(linkedin.clone());
            changes.push(format!("Updated LinkedIn handle to '{linkedin}'"));
        }
        if let Some(website) = cp.website {
            profile.contact.website = Some(website.clone());
            changes.push(format!("Updated website to '{website}'"));
        }
        if let Some(loc) = cp.current_location {
            profile.contact.current_location = Some(loc.clone());
            changes.push(format!("Updated current location to '{loc}'"));
        }
    }

    // 2. Summary
    if let Some(summary) = patch.summary {
        profile.background.summary = Some(summary);
        changes.push("Updated professional summary".to_string());
    }

    // 3. Education
    for edu in patch.add_education {
        let name = edu.institution.clone();
        profile.background.education.push(edu);
        changes.push(format!("Added education: {name}"));
    }
    for update in patch.update_education {
        let name = update.institution.clone();
        if let Some(existing) = profile.background.education.iter_mut().find(|e| {
            (update.id.is_some() && e.id == update.id)
                || e.institution.eq_ignore_ascii_case(&update.institution)
        }) {
            *existing = update;
            changes.push(format!("Updated education: {name}"));
        } else {
            profile.background.education.push(update);
            changes.push(format!("Added new education (not found for update): {name}"));
        }
    }
    for rem in patch.remove_education {
        let initial_len = profile.background.education.len();
        profile.background.education.retain(|e| {
            e.id.as_deref() != Some(&rem) && !e.institution.eq_ignore_ascii_case(&rem)
        });
        if profile.background.education.len() < initial_len {
            changes.push(format!("Removed education: {rem}"));
        }
    }

    // 4. Experience
    for exp in patch.add_experience {
        let comp = exp.company.clone();
        profile.background.experience.push(exp);
        changes.push(format!("Added experience: {comp}"));
    }
    for update in patch.update_experience {
        let comp = update.company.clone();
        if let Some(existing) = profile.background.experience.iter_mut().find(|e| {
            (update.id.is_some() && e.id == update.id)
                || e.company.eq_ignore_ascii_case(&update.company)
        }) {
            *existing = update;
            changes.push(format!("Updated experience: {comp}"));
        } else {
            profile.background.experience.push(update);
            changes.push(format!("Added new experience (not found for update): {comp}"));
        }
    }
    for rem in patch.remove_experience {
        let initial_len = profile.background.experience.len();
        profile.background.experience.retain(|e| {
            e.id.as_deref() != Some(&rem) && !e.company.eq_ignore_ascii_case(&rem)
        });
        if profile.background.experience.len() < initial_len {
            changes.push(format!("Removed experience: {rem}"));
        }
    }

    // 5. Projects
    for proj in patch.add_projects {
        let name = proj.name.clone();
        profile.background.projects.push(proj);
        changes.push(format!("Added project: {name}"));
    }
    for update in patch.update_projects {
        let name = update.name.clone();
        if let Some(existing) = profile.background.projects.iter_mut().find(|p| {
            (update.id.is_some() && p.id == update.id)
                || p.name.eq_ignore_ascii_case(&update.name)
        }) {
            *existing = update;
            changes.push(format!("Updated project: {name}"));
        } else {
            profile.background.projects.push(update);
            changes.push(format!("Added new project (not found for update): {name}"));
        }
    }
    for rem in patch.remove_projects {
        let initial_len = profile.background.projects.len();
        profile.background.projects.retain(|p| {
            p.id.as_deref() != Some(&rem) && !p.name.eq_ignore_ascii_case(&rem)
        });
        if profile.background.projects.len() < initial_len {
            changes.push(format!("Removed project: {rem}"));
        }
    }

    // 6. Skills
    for cat in patch.upsert_skills {
        let category_name = cat.category.clone();
        if let Some(existing_cat) = profile.background.skills.iter_mut().find(|s| {
            s.category.eq_ignore_ascii_case(&category_name)
        }) {
            for item in cat.items {
                if !existing_cat.items.iter().any(|i| i.eq_ignore_ascii_case(&item)) {
                    existing_cat.items.push(item);
                }
            }
            changes.push(format!("Updated skill category: {category_name}"));
        } else {
            profile.background.skills.push(cat);
            changes.push(format!("Added new skill category: {category_name}"));
        }
    }
    for rem in patch.remove_skills {
        let cat_name = rem.category.clone();
        if rem.items.is_empty() {
            profile.background.skills.retain(|s| !s.category.eq_ignore_ascii_case(&cat_name));
            changes.push(format!("Removed skill category: {cat_name}"));
        } else if let Some(existing) = profile.background.skills.iter_mut().find(|s| {
            s.category.eq_ignore_ascii_case(&cat_name)
        }) {
            existing.items.retain(|i| !rem.items.iter().any(|r| r.eq_ignore_ascii_case(i)));
            changes.push(format!("Removed skills from category {cat_name}: {:?}", rem.items));
        }
    }

    // 7. Certifications
    for cert in patch.add_certifications {
        let name = cert.name.clone();
        profile.background.certifications.push(cert);
        changes.push(format!("Added certification: {name}"));
    }
    for rem in patch.remove_certifications {
        let initial_len = profile.background.certifications.len();
        profile.background.certifications.retain(|c| {
            c.id.as_deref() != Some(&rem) && !c.name.eq_ignore_ascii_case(&rem)
        });
        if profile.background.certifications.len() < initial_len {
            changes.push(format!("Removed certification: {rem}"));
        }
    }

    // 8. Languages
    for lang in patch.upsert_languages {
        let name = lang.language.clone();
        if let Some(existing) = profile.background.languages.iter_mut().find(|l| {
            l.language.eq_ignore_ascii_case(&lang.language)
        }) {
            *existing = lang;
            changes.push(format!("Updated language proficiency for: {name}"));
        } else {
            profile.background.languages.push(lang);
            changes.push(format!("Added language: {name}"));
        }
    }
    for rem in patch.remove_languages {
        let initial_len = profile.background.languages.len();
        profile.background.languages.retain(|l| !l.language.eq_ignore_ascii_case(&rem));
        if profile.background.languages.len() < initial_len {
            changes.push(format!("Removed language: {rem}"));
        }
    }

    // 9. Publications & Awards
    for publ in patch.add_publications {
        changes.push(format!("Added publication: {}", publ.citation));
        profile.background.publications.push(publ);
    }
    for aw in patch.add_awards {
        let title = aw.title.clone();
        profile.background.awards.push(aw);
        changes.push(format!("Added award: {title}"));
    }

    // 10. Interview Stories Vault
    for story in patch.add_stories {
        let title = story.title.clone();
        profile.background.stories.push(story);
        changes.push(format!("Added interview story: {title}"));
    }
    for update in patch.update_stories {
        let title = update.title.clone();
        if let Some(existing) = profile.background.stories.iter_mut().find(|s| {
            s.id == update.id || s.title.eq_ignore_ascii_case(&update.title)
        }) {
            *existing = update;
            changes.push(format!("Updated interview story: {title}"));
        } else {
            profile.background.stories.push(update);
            changes.push(format!("Added new interview story (not found for update): {title}"));
        }
    }
    for rem in patch.remove_stories {
        let initial_len = profile.background.stories.len();
        profile.background.stories.retain(|s| {
            s.id != rem && !s.title.eq_ignore_ascii_case(&rem)
        });
        if profile.background.stories.len() < initial_len {
            changes.push(format!("Removed interview story: {rem}"));
        }
    }

    // 10. Future Career Orientation
    if let Some(co_patch) = patch.career_orientation {
        if let Some(full_replacement) = co_patch.replace_all {
            profile.career_orientation = full_replacement;
            changes.push("Completely reset career orientation".to_string());
        } else {
            for sen in co_patch.add_target_seniorities {
                if !profile.career_orientation.target_seniorities.iter().any(|s| s.eq_ignore_ascii_case(&sen)) {
                    profile.career_orientation.target_seniorities.push(sen.clone());
                    changes.push(format!("Added target seniority: {sen}"));
                }
            }
            for loc in co_patch.add_preferred_locations {
                if !profile.career_orientation.preferred_locations.iter().any(|l| l.eq_ignore_ascii_case(&loc)) {
                    profile.career_orientation.preferred_locations.push(loc.clone());
                    changes.push(format!("Added preferred location: {loc}"));
                }
            }
            for dom in co_patch.add_target_domains {
                if !profile.career_orientation.target_domains.iter().any(|d| d.eq_ignore_ascii_case(&dom)) {
                    profile.career_orientation.target_domains.push(dom.clone());
                    changes.push(format!("Added target domain: {dom}"));
                }
            }
            for role in co_patch.add_target_roles {
                if !profile.career_orientation.target_roles.iter().any(|r| r.eq_ignore_ascii_case(&role)) {
                    profile.career_orientation.target_roles.push(role.clone());
                    changes.push(format!("Added target role: {role}"));
                }
            }
            for arr in co_patch.add_work_arrangements {
                if !profile.career_orientation.work_arrangements.iter().any(|a| a.eq_ignore_ascii_case(&arr)) {
                    profile.career_orientation.work_arrangements.push(arr.clone());
                    changes.push(format!("Added work arrangement: {arr}"));
                }
            }
            if let Some(timeline) = co_patch.set_target_timeline {
                profile.career_orientation.target_timeline = Some(timeline.clone());
                changes.push(format!("Updated target start timeline to '{timeline}'"));
            }
            if let Some(notes) = co_patch.set_notes {
                profile.career_orientation.notes = Some(notes);
                changes.push("Updated career orientation notes".to_string());
            }
        }
    }

    (profile, changes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_patch_incremental_additions() {
        let initial = CuratedProfile::default();
        let patch = ProfilePatch {
            contact: Some(ContactInfoPatch {
                name: Some("Jane Doe".to_string()),
                email: Some("jane@example.com".to_string()),
                ..Default::default()
            }),
            career_orientation: Some(CareerOrientationPatch {
                add_target_roles: vec!["AI Engineer".to_string(), "BE Engineer".to_string()],
                add_preferred_locations: vec!["Ho Chi Minh City".to_string(), "Remote".to_string()],
                add_target_seniorities: vec!["Senior".to_string()],
                ..Default::default()
            }),
            ..Default::default()
        };

        let (updated, changes) = apply_patch(initial, patch);
        assert_eq!(updated.contact.name.as_deref(), Some("Jane Doe"));
        assert_eq!(updated.contact.email.as_deref(), Some("jane@example.com"));
        assert_eq!(updated.career_orientation.target_roles.len(), 2);
        assert_eq!(updated.career_orientation.preferred_locations.len(), 2);
        assert_eq!(updated.career_orientation.target_seniorities.len(), 1);
        assert!(!changes.is_empty());
    }

    #[test]
    fn test_upsert_skills() {
        let mut initial = CuratedProfile::default();
        initial.background.skills.push(SkillCategory {
            category: "Languages".to_string(),
            items: vec!["Rust".to_string()],
        });

        let patch = ProfilePatch {
            upsert_skills: vec![
                SkillCategory {
                    category: "Languages".to_string(),
                    items: vec!["Rust".to_string(), "Go".to_string()],
                },
                SkillCategory {
                    category: "Cloud".to_string(),
                    items: vec!["AWS".to_string()],
                }
            ],
            ..Default::default()
        };

        let (updated, _) = apply_patch(initial, patch);
        let langs = updated.background.skills.iter().find(|s| s.category == "Languages").unwrap();
        assert_eq!(langs.items, vec!["Rust", "Go"]);
        assert_eq!(updated.background.skills.len(), 2);
    }

    #[test]
    fn test_patch_story_vault() {
        let initial = CuratedProfile::default();
        let story = StoryItem {
            id: "story-incident-1".to_string(),
            title: "Database Outage Resolution".to_string(),
            situation: "Primary DB failed during peak load".to_string(),
            task: Some("Restore data safely within 15 min".to_string()),
            action: "Promoted read replica and switched DNS".to_string(),
            result: "Zero data loss and service restored in 8 min".to_string(),
            learnings: Some("Automate replica promotion".to_string()),
            tags: vec!["reliability".to_string(), "incident-management".to_string()],
            related_experience_id: None,
        };

        let patch = ProfilePatch {
            add_stories: vec![story],
            ..Default::default()
        };

        let (updated, changes) = apply_patch(initial, patch);
        assert_eq!(updated.background.stories.len(), 1);
        assert_eq!(updated.background.stories[0].title, "Database Outage Resolution");
        assert!(changes.iter().any(|c| c.contains("Added interview story")));

        // Update story
        let mut updated_story = updated.background.stories[0].clone();
        updated_story.result = "Zero data loss, 5 min recovery".to_string();
        let patch_update = ProfilePatch {
            update_stories: vec![updated_story],
            ..Default::default()
        };
        let (updated_2, changes_2) = apply_patch(updated, patch_update);
        assert_eq!(updated_2.background.stories[0].result, "Zero data loss, 5 min recovery");
        assert!(changes_2.iter().any(|c| c.contains("Updated interview story")));

        // Remove story
        let patch_remove = ProfilePatch {
            remove_stories: vec!["story-incident-1".to_string()],
            ..Default::default()
        };
        let (updated_3, changes_3) = apply_patch(updated_2, patch_remove);
        assert_eq!(updated_3.background.stories.len(), 0);
        assert!(changes_3.iter().any(|c| c.contains("Removed interview story")));
    }
}

