use crate::i18n::Locale;

const ENGLISH_RESOURCE: &str = include_str!("resources/ENGLISH.md");
const VIETNAMESE_RESOURCE: &str = include_str!("resources/TIENG-VIET.md");

/// A single flashcard extracted from an interview-study resource.
#[derive(Debug, Clone, PartialEq)]
pub struct ReviewCard {
    pub section: String,
    pub title: String,
    pub content: String,
}

/// Returns flashcards for the active application locale.
pub fn review_cards(locale: Locale) -> Vec<ReviewCard> {
    parse_review_resource(match locale {
        Locale::vi => VIETNAMESE_RESOURCE,
        Locale::en => ENGLISH_RESOURCE,
    })
}

fn parse_review_resource(resource: &str) -> Vec<ReviewCard> {
    let mut cards = Vec::new();
    let mut part = String::new();
    let mut section = String::new();
    let mut title: Option<String> = None;
    let mut content = Vec::new();

    for line in resource.lines() {
        if let Some(value) = line.strip_prefix("# Part ") {
            flush_card(&mut cards, &part, &section, title.take(), &mut content);
            part = value.trim().to_string();
            section.clear();
            continue;
        }

        if let Some(value) = line.strip_prefix("# Phần ") {
            flush_card(&mut cards, &part, &section, title.take(), &mut content);
            part = value.trim().to_string();
            section.clear();
            continue;
        }

        if let Some(value) = line.strip_prefix("## ") {
            flush_card(&mut cards, &part, &section, title.take(), &mut content);
            section = value.trim().to_string();
            continue;
        }

        if let Some(value) = line.strip_prefix("### ") {
            flush_card(&mut cards, &part, &section, title.take(), &mut content);
            title = Some(value.trim().to_string());
            continue;
        }

        if section.is_empty() {
            continue;
        }

        if title.is_none() {
            title = Some(section.clone());
        }

        content.push(line.to_string());
    }

    flush_card(&mut cards, &part, &section, title, &mut content);
    cards
}

fn flush_card(
    cards: &mut Vec<ReviewCard>,
    part: &str,
    section: &str,
    title: Option<String>,
    content: &mut Vec<String>,
) {
    let Some(title) = title else {
        return;
    };

    let body = content.join("\n").trim().to_string();
    if body.is_empty() {
        return;
    }

    let label = if title == section {
        part.to_string()
    } else {
        format!("{part} / {section}")
    };

    cards.push(ReviewCard {
        section: label,
        title,
        content: body,
    });
    content.clear();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn english_resource_produces_granular_review_cards() {
        let cards = review_cards(Locale::en);
        assert_eq!(cards.len(), 99);
        assert_eq!(cards[0].title, "Virtual vs abstract vs interface");
        assert_eq!(cards[0].section, "1 — Foundation / OOP");
        assert!(cards[0].content.contains("Interface"));
    }

    #[test]
    fn vietnamese_resource_produces_granular_review_cards() {
        let cards = review_cards(Locale::vi);
        assert_eq!(cards.len(), 99);
        assert_eq!(cards[0].title, "Virtual vs abstract vs interface");
        assert_eq!(cards[0].section, "1 — Nền tảng / OOP");
        assert!(cards[0].content.contains("Interface"));
    }
}
