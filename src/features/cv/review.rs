use crate::i18n::Locale;

const ENGLISH_RESOURCE: &str = include_str!("resources/ENGLISH.md");
const VIETNAMESE_RESOURCE: &str = include_str!("resources/TIENG-VIET.md");

#[derive(Debug, Clone, PartialEq)]
pub struct ReviewCard {
    pub section: String,
    pub title: String,
    pub content: String,
}

pub fn review_cards(locale: Locale) -> Vec<ReviewCard> {
    parse_review_resource(match locale {
        Locale::vi => VIETNAMESE_RESOURCE,
        Locale::en => ENGLISH_RESOURCE,
    })
}

fn parse_review_resource(resource: &str) -> Vec<ReviewCard> {
    let mut cards = Vec::new();
    let mut section = String::new();
    let mut title: Option<String> = None;
    let mut content = Vec::new();

    for line in resource.lines() {
        if let Some(part) = line.strip_prefix("# Part ") {
            section = part.trim().to_string();
            continue;
        }

        if let Some(part) = line.strip_prefix("# Phần ") {
            section = part.trim().to_string();
            continue;
        }

        if let Some(new_title) = line.strip_prefix("## ") {
            push_card(&mut cards, &section, title.take(), &mut content);
            content.clear();
            title = Some(new_title.trim().to_string());
            continue;
        }

        if title.is_some() {
            content.push(line.to_string());
        }
    }

    push_card(&mut cards, &section, title, &mut content);
    cards
}

fn push_card(
    cards: &mut Vec<ReviewCard>,
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

    cards.push(ReviewCard {
        section: section.to_string(),
        title,
        content: body,
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn english_resource_produces_review_cards() {
        let cards = review_cards(Locale::en);
        assert_eq!(cards.len(), 35);
        assert_eq!(cards[0].title, "OOP");
        assert_eq!(cards[0].section, "1 — Foundation");
        assert!(cards[0].content.contains("Abstraction"));
    }

    #[test]
    fn vietnamese_resource_produces_review_cards() {
        let cards = review_cards(Locale::vi);
        assert_eq!(cards.len(), 35);
        assert_eq!(cards[0].title, "OOP");
        assert_eq!(cards[0].section, "1 — Nền tảng");
        assert!(cards[0].content.contains("Trừu tượng"));
    }
}
