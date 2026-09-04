/// Static profile data displayed by the public CV page.
#[derive(Debug, Clone, PartialEq)]
pub struct Profile {
    pub name: &'static str,
    pub title: &'static str,
    pub location: &'static str,
    pub phone: &'static str,
    pub email: &'static str,
    pub summary: &'static str,
}

/// A professional competency displayed in the CV.
#[derive(Debug, Clone, PartialEq)]
pub struct Competency {
    pub name: &'static str,
}

/// A technical skill category and its technologies.
#[derive(Debug, Clone, PartialEq)]
pub struct SkillCategory {
    pub name: &'static str,
    pub skills: Vec<&'static str>,
}

/// A professional experience entry.
#[derive(Debug, Clone, PartialEq)]
pub struct Experience {
    pub company: &'static str,
    pub role: &'static str,
    pub period: &'static str,
    pub description: &'static str,
    pub projects: Vec<&'static str>,
    pub contributions: Vec<&'static str>,
    pub technologies: Vec<&'static str>,
}

/// A qualification in the candidate's education history.
#[derive(Debug, Clone, PartialEq)]
pub struct Education {
    pub institution: &'static str,
    pub degree: &'static str,
    pub major: &'static str,
    pub classification: &'static str,
}

/// Returns the candidate's profile information.
pub fn profile() -> Profile {
    Profile {
        name: "Thai Doan Son",
        title: "Senior Backend Developer",
        location: "Vietnam",
        phone: "0814466008",
        email: "doanson44@gmail.com",
        summary: "Senior Backend Developer with professional experience delivering enterprise software across Finance, Retail, FMCG, Employee Management, Project Management, Promotion Platforms, Virtual Commerce, and Embedded Software domains. Specialized in designing and developing scalable backend applications using C#, ASP.NET Core, .NET Core, Entity Framework Core, SQL Server, PostgreSQL, Azure, and RESTful APIs. Strong background in enterprise application modernization, system integration, database optimization, software architecture, and production support.",
    }
}

/// Returns the candidate's core professional competencies.
pub fn competencies() -> Vec<Competency> {
    vec![
        Competency {
            name: "Backend Engineering",
        },
        Competency {
            name: "Enterprise Application Development",
        },
        Competency {
            name: "RESTful API Design",
        },
        Competency {
            name: "Database Design & Optimization",
        },
        Competency {
            name: "System Integration",
        },
        Competency {
            name: "Performance Optimization",
        },
        Competency {
            name: "Cloud-based Solutions",
        },
        Competency {
            name: "Software Architecture",
        },
        Competency {
            name: "Clean Code & Maintainability",
        },
        Competency {
            name: "Legacy System Modernization",
        },
        Competency {
            name: "Cross-functional Collaboration",
        },
        Competency {
            name: "Production Support & Debugging",
        },
    ]
}

/// Returns the candidate's technical skills grouped by category.
pub fn skill_categories() -> Vec<SkillCategory> {
    vec![
        SkillCategory {
            name: "Languages",
            skills: vec!["C#", "SQL", "Java", "JavaScript"],
        },
        SkillCategory {
            name: "Backend",
            skills: vec![
                "ASP.NET Core",
                ".NET Core",
                "ASP.NET MVC",
                "Entity Framework Core",
                "REST API",
                "Web API",
                "Background Services",
                "LINQ",
                "Async Programming",
            ],
        },
        SkillCategory {
            name: "Architecture",
            skills: vec![
                "Clean Architecture",
                "SOLID",
                "CQRS",
                "Repository Pattern",
                "Unit of Work",
                "Dependency Injection",
                "Layered Architecture",
                "Design Patterns",
            ],
        },
        SkillCategory {
            name: "Database",
            skills: vec![
                "SQL Server",
                "PostgreSQL",
                "MySQL",
                "Query Optimization",
                "Indexing",
                "Transactions",
                "Stored Procedures",
                "EF Core Migrations",
            ],
        },
        SkillCategory {
            name: "Cloud & DevOps",
            skills: vec![
                "Microsoft Azure",
                "Azure Storage",
                "Azure Service Bus",
                "Azure VM",
                "ARM Templates",
                "Azure DevOps",
                "Docker",
                "CI/CD",
                "Git",
            ],
        },
        SkillCategory {
            name: "Frontend",
            skills: vec!["Angular", "HTML", "CSS", "JavaScript"],
        },
        SkillCategory {
            name: "Tools",
            skills: vec![
                "Visual Studio",
                "VS Code",
                "IntelliJ IDEA",
                "Eclipse",
                "Notepad++",
            ],
        },
        SkillCategory {
            name: "Platforms",
            skills: vec!["Windows", "Linux (Ubuntu, Debian)"],
        },
        SkillCategory {
            name: "Practices",
            skills: vec![
                "Agile",
                "Scrum",
                "Code Review",
                "Requirement Analysis",
                "Technical Documentation",
                "Debugging",
                "Performance Tuning",
            ],
        },
    ]
}

/// Returns the candidate's professional experience.
pub fn experiences() -> Vec<Experience> {
    vec![
        Experience {
            company: "Titan Technology",
            role: "Senior Software Developer",
            period: "Jun 2020 — Mar 2022",
            description: "Developed enterprise financial software and third-party integrations.",
            projects: vec!["QuickBooks Integration", "Notification", "Carry Forward", "Activation", "CU Indicator"],
            contributions: vec![
                "Built backend services and automation features.",
                "Integrated enterprise systems with external services.",
                "Optimized SQL Server queries and data processing.",
                "Maintained production systems and resolved complex issues.",
            ],
            technologies: vec![".NET Core", "SQL Server"],
        },
        Experience {
            company: "Hybrid Technologies",
            role: "Software Developer",
            period: "Dec 2019 — Jun 2020",
            description: "Developed backend features for a Domestic Tour platform and delivered enhancements and production bug fixes.",
            projects: Vec::new(),
            contributions: vec![
                "Developed backend features for Domestic Tour platform.",
                "Implemented enhancements and production bug fixes.",
            ],
            technologies: Vec::new(),
        },
        Experience {
            company: "MTI Technology",
            role: "Software Developer",
            period: "Jul 2017 — Dec 2019",
            description: "Developed and modernized enterprise applications and management systems.",
            projects: vec!["Device Management Tool", "Seasar Migration", "Music in the Ship", "Renovation 2"],
            contributions: vec![
                "Migrated legacy applications to .NET Core.",
                "Developed enterprise management systems.",
                "Participated in requirement clarification and system enhancement.",
                "Worked with Angular, Vue.js, and Entity Framework Core.",
            ],
            technologies: vec![".NET Core", "Entity Framework Core", "Angular", "Vue.js"],
        },
        Experience {
            company: "Renesas Design Vietnam",
            role: "Software Engineer",
            period: "Mar 2015 — Jul 2017",
            description: "Designed and developed engineering desktop applications and supported customer requirements and maintenance work.",
            projects: vec!["MISRA-C Checker", "Smart Manual", "Current Consumption"],
            contributions: vec![
                "Designed and developed engineering desktop applications.",
                "Analyzed customer requirements.",
                "Delivered maintenance, debugging, and feature implementation.",
            ],
            technologies: Vec::new(),
        },
    ]
}

/// Returns selected technical highlights from the CV.
pub fn highlights() -> Vec<&'static str> {
    vec![
        "Extensive enterprise software development experience.",
        "Extensive experience with backend development using C# and .NET technologies.",
        "Strong knowledge of relational databases and SQL optimization.",
        "Experienced in enterprise system integration and cloud-based solutions.",
        "Proficient in debugging, production support, and legacy modernization.",
        "Familiar with scalable application design and software engineering best practices.",
    ]
}

/// Returns the candidate's education history.
pub fn education() -> Vec<Education> {
    vec![Education {
        institution: "University of Science — Vietnam National University Ho Chi Minh City",
        degree: "Bachelor of Engineering",
        major: "Information Technology",
        classification: "Good",
    }]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profile_contains_expected_identity() {
        let value = profile();
        assert_eq!(value.name, "Thai Doan Son");
        assert_eq!(value.title, "Senior Backend Developer");
        assert_eq!(value.phone, "0814466008");
        assert_eq!(value.email, "doanson44@gmail.com");
    }

    #[test]
    fn experiences_exclude_nda_sensitive_employment() {
        let value = experiences();
        assert_eq!(value.len(), 4);
        assert!(!value
            .iter()
            .any(|experience| experience.company.contains("Niteco")));
        assert_eq!(value[0].company, "Titan Technology");
        assert_eq!(value[3].company, "Renesas Design Vietnam");
    }

    #[test]
    fn skills_contain_backend_and_architecture_categories() {
        let value = skill_categories();
        assert!(value.iter().any(|category| category.name == "Backend"));
        assert!(value.iter().any(|category| category.name == "Architecture"));
    }

    #[test]
    fn education_contains_expected_degree() {
        let value = education();
        assert_eq!(value.len(), 1);
        assert_eq!(value[0].degree, "Bachelor of Engineering");
    }
}
