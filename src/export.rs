use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::schema::CuratedProfile;

/// Downstream CvProfile structure matching `../cv-writer`'s input schema.
/// This enables seamless single-call piping: `export_to_cv_writer` -> `cv-writer`'s `render_cv`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct CvWriterProfile {
    pub contact: CvWriterContact,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub education: Vec<CvWriterEducationItem>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub experience: Vec<CvWriterExperienceItem>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub projects: Vec<CvWriterProjectItem>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub skills: Vec<CvWriterSkillCategory>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub certifications: Vec<CvWriterCertificationItem>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub publications: Vec<CvWriterPublicationItem>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub awards: Vec<CvWriterAwardItem>,
    #[serde(default)]
    pub hide_page_numbers: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct CvWriterContact {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub github: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linkedin: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct CvWriterEducationItem {
    pub institution: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degree: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dates: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub highlights: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct CvWriterExperienceItem {
    pub company: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    pub roles: Vec<CvWriterRoleItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct CvWriterRoleItem {
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dates: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub highlights: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct CvWriterProjectItem {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dates: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub highlights: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct CvWriterSkillCategory {
    pub category: String,
    pub items: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct CvWriterCertificationItem {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct CvWriterPublicationItem {
    pub citation: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct CvWriterAwardItem {
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
}

/// Converts a curated profile into the exact CvWriterProfile expected by `../cv-writer`.
pub fn export_profile_for_cv_writer(profile: &CuratedProfile) -> CvWriterProfile {
    let name = profile.contact.name.clone().unwrap_or_else(|| "Anonymous Candidate".to_string());
    
    // Map education
    let education = profile.background.education.iter().map(|e| {
        CvWriterEducationItem {
            institution: e.institution.clone(),
            degree: e.degree.clone(),
            dates: e.dates.clone(),
            highlights: e.highlights.clone(),
        }
    }).collect();

    // Map experience
    let experience = profile.background.experience.iter().map(|c| {
        CvWriterExperienceItem {
            company: c.company.clone(),
            location: c.location.clone(),
            roles: c.roles.iter().map(|r| {
                CvWriterRoleItem {
                    title: r.title.clone(),
                    dates: r.dates.clone(),
                    highlights: r.highlights.clone(),
                }
            }).collect(),
        }
    }).collect();

    // Map projects (merging technologies into highlights if present)
    let projects = profile.background.projects.iter().map(|p| {
        let mut highlights = p.highlights.clone();
        if !p.technologies.is_empty() {
            highlights.insert(0, format!("Technologies: {}", p.technologies.join(", ")));
        }
        CvWriterProjectItem {
            name: p.name.clone(),
            url: p.url.clone(),
            dates: p.dates.clone(),
            highlights,
        }
    }).collect();

    // Map skills, plus append a category for Languages if spoken languages are defined
    let mut skills: Vec<CvWriterSkillCategory> = profile.background.skills.iter().map(|s| {
        CvWriterSkillCategory {
            category: s.category.clone(),
            items: s.items.clone(),
        }
    }).collect();

    if !profile.background.languages.is_empty() {
        let lang_items: Vec<String> = profile.background.languages.iter().map(|l| {
            format!("{} ({})", l.language, l.proficiency)
        }).collect();
        skills.push(CvWriterSkillCategory {
            category: "Languages".to_string(),
            items: lang_items,
        });
    }

    // Map certifications
    let certifications = profile.background.certifications.iter().map(|c| {
        CvWriterCertificationItem {
            name: c.name.clone(),
            issuer: c.issuer.clone(),
            date: c.date.clone(),
        }
    }).collect();

    // Map publications
    let publications = profile.background.publications.iter().map(|p| {
        CvWriterPublicationItem {
            citation: p.citation.clone(),
            url: p.url.clone(),
        }
    }).collect();

    // Map awards
    let awards = profile.background.awards.iter().map(|a| {
        CvWriterAwardItem {
            title: a.title.clone(),
            date: a.date.clone(),
            summary: a.summary.clone(),
        }
    }).collect();

    CvWriterProfile {
        contact: CvWriterContact {
            name,
            phone: profile.contact.phone.clone(),
            email: profile.contact.email.clone(),
            github: profile.contact.github.clone(),
            linkedin: profile.contact.linkedin.clone(),
            website: profile.contact.website.clone(),
            location: profile.contact.current_location.clone(),
        },
        summary: profile.background.summary.clone(),
        education,
        experience,
        projects,
        skills,
        certifications,
        publications,
        awards,
        hide_page_numbers: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::sample_curated_profile;

    #[test]
    fn test_export_sample_to_cv_writer() {
        let profile = sample_curated_profile();
        let cv = export_profile_for_cv_writer(&profile);
        assert_eq!(cv.contact.name, "Alex Chen");
        assert_eq!(cv.experience.len(), 1);
        assert_eq!(cv.education.len(), 1);
        assert_eq!(cv.projects.len(), 1);
        // Verify languages were converted into skills
        assert!(cv.skills.iter().any(|s| s.category == "Languages"));
    }
}
