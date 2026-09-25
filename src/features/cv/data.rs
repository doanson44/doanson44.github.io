use crate::i18n::Locale;
use sha2::Digest;

/// Static profile data displayed by the public CV page.
#[derive(Debug, Clone, PartialEq)]
pub struct Profile {
    pub name: &'static str,
    pub title: &'static str,
    pub location: &'static str,
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
pub fn profile(locale: Locale) -> Profile {
    Profile {
        name: "Thai Doan Son",
        title: "Senior Backend Developer",
        location: if locale == Locale::vi {
            "Việt Nam"
        } else {
            "Vietnam"
        },
        summary: if locale == Locale::vi {
            "Senior Backend Developer với khoảng 11 năm kinh nghiệm phát triển phần mềm chuyên nghiệp trong các lĩnh vực Tài chính, Bán lẻ, FMCG, Quản lý nhân sự, Quản lý dự án, Nền tảng khuyến mãi, Thương mại ảo và Phần mềm nhúng. Chuyên về thiết kế và phát triển các ứng dụng backend có khả năng mở rộng với C#, ASP.NET Core, .NET Core, Entity Framework Core, SQL Server, PostgreSQL, Azure và RESTful API. Có nền tảng vững về hiện đại hóa ứng dụng doanh nghiệp, tích hợp hệ thống, tối ưu cơ sở dữ liệu, kiến trúc phần mềm và hỗ trợ production."
        } else {
            "Senior Backend Developer with professional experience delivering enterprise software across Finance, Retail, FMCG, Employee Management, Project Management, Promotion Platforms, Virtual Commerce, and Embedded Software domains. Specialized in designing and developing scalable backend applications using C#, ASP.NET Core, .NET Core, Entity Framework Core, SQL Server, PostgreSQL, Azure, and RESTful APIs. Strong background in enterprise application modernization, system integration, database optimization, software architecture, and production support."
        },
    }
}

/// Contact data is stored as encrypted bytes so the plaintext is not embedded in the WASM source.
///
/// This is client-side obfuscation rather than a security boundary: the decryption key must ship
/// with the application because the browser needs it to reveal the contact data.
const CONTACT_KEY: &[u8] = b"cv-contact-v1:doanson44:reveal";
const ENCRYPTED_PHONE: &[u8] = &[67, 9, 237, 212, 124, 86, 14, 172, 26, 145];
const ENCRYPTED_EMAIL: &[u8] = &[
    16, 89, 177, 131, 33, 7, 89, 239, 71, 232, 177, 182, 0, 149, 203, 220, 174, 8, 27, 77, 153,
];

/// Decrypts a contact value only when the UI explicitly asks to reveal it.
fn decrypt_contact(ciphertext: &[u8]) -> String {
    let mut plaintext = Vec::with_capacity(ciphertext.len());

    for (index, &byte) in ciphertext.iter().enumerate() {
        let block_index = (index / 32) as u32;
        let mut input = Vec::with_capacity(CONTACT_KEY.len() + 4);
        input.extend_from_slice(CONTACT_KEY);
        input.extend_from_slice(&block_index.to_be_bytes());

        let digest = sha2::Sha256::digest(&input);
        plaintext.push(byte ^ digest[index % 32]);
    }

    String::from_utf8(plaintext).expect("encrypted CV contact data must be valid UTF-8")
}

/// Reveals the candidate's phone number.
pub fn reveal_phone() -> String {
    decrypt_contact(ENCRYPTED_PHONE)
}

/// Reveals the candidate's email address.
pub fn reveal_email() -> String {
    decrypt_contact(ENCRYPTED_EMAIL)
}

/// Returns the candidate's core professional competencies.
pub fn competencies(locale: Locale) -> Vec<Competency> {
    vec![
        Competency {
            name: "Backend Engineering",
        },
        Competency {
            name: if locale == Locale::vi {
                "Phát triển ứng dụng doanh nghiệp"
            } else {
                "Enterprise Application Development"
            },
        },
        Competency {
            name: if locale == Locale::vi {
                "Thiết kế RESTful API"
            } else {
                "RESTful API Design"
            },
        },
        Competency {
            name: if locale == Locale::vi {
                "Thiết kế & tối ưu cơ sở dữ liệu"
            } else {
                "Database Design & Optimization"
            },
        },
        Competency {
            name: if locale == Locale::vi {
                "Tích hợp hệ thống"
            } else {
                "System Integration"
            },
        },
        Competency {
            name: if locale == Locale::vi {
                "Tối ưu hiệu năng"
            } else {
                "Performance Optimization"
            },
        },
        Competency {
            name: if locale == Locale::vi {
                "Giải pháp nền tảng đám mây"
            } else {
                "Cloud-based Solutions"
            },
        },
        Competency {
            name: if locale == Locale::vi {
                "Kiến trúc phần mềm"
            } else {
                "Software Architecture"
            },
        },
        Competency {
            name: if locale == Locale::vi {
                "Clean Code & khả năng bảo trì"
            } else {
                "Clean Code & Maintainability"
            },
        },
        Competency {
            name: if locale == Locale::vi {
                "Hiện đại hóa hệ thống legacy"
            } else {
                "Legacy System Modernization"
            },
        },
        Competency {
            name: if locale == Locale::vi {
                "Phối hợp liên phòng ban"
            } else {
                "Cross-functional Collaboration"
            },
        },
        Competency {
            name: if locale == Locale::vi {
                "Hỗ trợ production & debug"
            } else {
                "Production Support & Debugging"
            },
        },
    ]
}

/// Returns the candidate's technical skills grouped by category.
pub fn skill_categories(locale: Locale) -> Vec<SkillCategory> {
    vec![
        SkillCategory {
            name: if locale == Locale::vi {
                "Ngôn ngữ"
            } else {
                "Languages"
            },
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
            name: if locale == Locale::vi {
                "Kiến trúc"
            } else {
                "Architecture"
            },
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
            name: if locale == Locale::vi {
                "Cơ sở dữ liệu"
            } else {
                "Database"
            },
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
            name: if locale == Locale::vi {
                "Công cụ"
            } else {
                "Tools"
            },
            skills: vec![
                "Visual Studio",
                "VS Code",
                "IntelliJ IDEA",
                "Eclipse",
                "Notepad++",
            ],
        },
        SkillCategory {
            name: if locale == Locale::vi {
                "Nền tảng"
            } else {
                "Platforms"
            },
            skills: vec!["Windows", "Linux (Ubuntu, Debian)"],
        },
        SkillCategory {
            name: if locale == Locale::vi {
                "Phương pháp"
            } else {
                "Practices"
            },
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
pub fn experiences(locale: Locale) -> Vec<Experience> {
    vec![
        Experience {
            company: "Titan Technology",
            role: "Senior Software Developer",
            period: "Jun 2020 — Mar 2022",
            description: if locale == Locale::vi {
                "Phát triển phần mềm tài chính doanh nghiệp và các tích hợp với hệ thống bên thứ ba."
            } else {
                "Developed enterprise financial software and third-party integrations."
            },
            projects: vec![
                if locale == Locale::vi {
                    "Tích hợp QuickBooks"
                } else {
                    "QuickBooks Integration"
                },
                if locale == Locale::vi {
                    "Thông báo"
                } else {
                    "Notification"
                },
                "Carry Forward",
                "Activation",
                "CU Indicator",
            ],
            contributions: vec![
                if locale == Locale::vi {
                    "Xây dựng backend services và các tính năng tự động hóa."
                } else {
                    "Built backend services and automation features."
                },
                if locale == Locale::vi {
                    "Tích hợp hệ thống doanh nghiệp với các dịch vụ bên ngoài."
                } else {
                    "Integrated enterprise systems with external services."
                },
                if locale == Locale::vi {
                    "Tối ưu truy vấn SQL Server và xử lý dữ liệu."
                } else {
                    "Optimized SQL Server queries and data processing."
                },
                if locale == Locale::vi {
                    "Bảo trì hệ thống production và xử lý các sự cố phức tạp."
                } else {
                    "Maintained production systems and resolved complex issues."
                },
            ],
            technologies: vec![".NET Core", "SQL Server"],
        },
        Experience {
            company: "Hybrid Technologies",
            role: "Software Developer",
            period: "Dec 2019 — Jun 2020",
            description: if locale == Locale::vi {
                "Phát triển các tính năng backend cho nền tảng Domestic Tour và triển khai các cải tiến, sửa lỗi production."
            } else {
                "Developed backend features for a Domestic Tour platform and delivered enhancements and production bug fixes."
            },
            projects: Vec::new(),
            contributions: vec![
                if locale == Locale::vi {
                    "Phát triển các tính năng backend cho nền tảng Domestic Tour."
                } else {
                    "Developed backend features for Domestic Tour platform."
                },
                if locale == Locale::vi {
                    "Triển khai các cải tiến và sửa lỗi production."
                } else {
                    "Implemented enhancements and production bug fixes."
                },
            ],
            technologies: Vec::new(),
        },
        Experience {
            company: "MTI Technology",
            role: "Software Developer",
            period: "Jul 2017 — Dec 2019",
            description: if locale == Locale::vi {
                "Phát triển và hiện đại hóa các ứng dụng doanh nghiệp và hệ thống quản lý."
            } else {
                "Developed and modernized enterprise applications and management systems."
            },
            projects: vec![
                "Device Management Tool",
                "Seasar Migration",
                "Music in the Ship",
                "Renovation 2",
            ],
            contributions: vec![
                if locale == Locale::vi {
                    "Migration các ứng dụng legacy sang .NET Core."
                } else {
                    "Migrated legacy applications to .NET Core."
                },
                if locale == Locale::vi {
                    "Phát triển các hệ thống quản lý doanh nghiệp."
                } else {
                    "Developed enterprise management systems."
                },
                if locale == Locale::vi {
                    "Tham gia làm rõ yêu cầu và nâng cấp hệ thống."
                } else {
                    "Participated in requirement clarification and system enhancement."
                },
                if locale == Locale::vi {
                    "Làm việc với Angular, Vue.js và Entity Framework Core."
                } else {
                    "Worked with Angular, Vue.js, and Entity Framework Core."
                },
            ],
            technologies: vec![".NET Core", "Entity Framework Core", "Angular", "Vue.js"],
        },
        Experience {
            company: "Renesas Design Vietnam",
            role: "Software Engineer",
            period: "Mar 2015 — Jul 2017",
            description: if locale == Locale::vi {
                "Thiết kế và phát triển các ứng dụng desktop phục vụ kỹ thuật, đồng thời hỗ trợ yêu cầu khách hàng và công việc bảo trì."
            } else {
                "Designed and developed engineering desktop applications and supported customer requirements and maintenance work."
            },
            projects: vec!["MISRA-C Checker", "Smart Manual", "Current Consumption"],
            contributions: vec![
                if locale == Locale::vi {
                    "Thiết kế và phát triển các ứng dụng desktop phục vụ kỹ thuật."
                } else {
                    "Designed and developed engineering desktop applications."
                },
                if locale == Locale::vi {
                    "Phân tích yêu cầu khách hàng."
                } else {
                    "Analyzed customer requirements."
                },
                if locale == Locale::vi {
                    "Thực hiện bảo trì, debug và phát triển tính năng."
                } else {
                    "Delivered maintenance, debugging, and feature implementation."
                },
            ],
            technologies: Vec::new(),
        },
    ]
}

/// Returns selected technical highlights from the CV.
pub fn highlights(locale: Locale) -> Vec<&'static str> {
    vec![
        if locale == Locale::vi {
            "Có nhiều kinh nghiệm phát triển phần mềm doanh nghiệp."
        } else {
            "Extensive enterprise software development experience."
        },
        if locale == Locale::vi {
            "Có nhiều kinh nghiệm backend với C# và các công nghệ .NET."
        } else {
            "Extensive experience with backend development using C# and .NET technologies."
        },
        if locale == Locale::vi {
            "Nắm vững cơ sở dữ liệu quan hệ và tối ưu SQL."
        } else {
            "Strong knowledge of relational databases and SQL optimization."
        },
        if locale == Locale::vi {
            "Có kinh nghiệm tích hợp hệ thống doanh nghiệp và các giải pháp cloud."
        } else {
            "Experienced in enterprise system integration and cloud-based solutions."
        },
        if locale == Locale::vi {
            "Thành thạo debug, hỗ trợ production và hiện đại hóa hệ thống legacy."
        } else {
            "Proficient in debugging, production support, and legacy modernization."
        },
        if locale == Locale::vi {
            "Có kinh nghiệm với thiết kế ứng dụng có khả năng mở rộng và các thực hành kỹ thuật phần mềm."
        } else {
            "Familiar with scalable application design and software engineering best practices."
        },
    ]
}

/// Returns the candidate's education history.
pub fn education(locale: Locale) -> Vec<Education> {
    vec![Education {
        institution: "University of Science — Vietnam National University Ho Chi Minh City",
        degree: if locale == Locale::vi {
            "Kỹ sư"
        } else {
            "Bachelor of Engineering"
        },
        major: if locale == Locale::vi {
            "Công nghệ thông tin"
        } else {
            "Information Technology"
        },
        classification: if locale == Locale::vi { "Khá" } else { "Good" },
    }]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profile_contains_expected_identity() {
        let value = profile(Locale::en);
        assert_eq!(value.name, "Thai Doan Son");
        assert_eq!(value.title, "Senior Backend Developer");
    }

    #[test]
    fn experiences_exclude_nda_sensitive_employment() {
        let value = experiences(Locale::en);
        assert_eq!(value.len(), 4);
        assert!(!value
            .iter()
            .any(|experience| experience.company.contains("Niteco")));
        assert_eq!(value[0].company, "Titan Technology");
        assert_eq!(value[3].company, "Renesas Design Vietnam");
    }

    #[test]
    fn skills_contain_backend_and_architecture_categories() {
        let value = skill_categories(Locale::en);
        assert!(value.iter().any(|category| category.name == "Backend"));
        assert!(value.iter().any(|category| category.name == "Architecture"));
    }

    #[test]
    fn education_contains_expected_degree() {
        let value = education(Locale::en);
        assert_eq!(value.len(), 1);
        assert_eq!(value[0].degree, "Bachelor of Engineering");
    }
}
