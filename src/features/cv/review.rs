use crate::i18n::Locale;

const ENGLISH_RESOURCE: &str = include_str!("resources/en.json");
const VIETNAMESE_RESOURCE: &str = include_str!("resources/vi.json");

/// A single flashcard from the interview study resource.
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct ReviewCard {
    pub section: String,
    pub title: String,
    pub content: String,
}

/// Returns flashcards for the active application locale.
pub fn review_cards(locale: Locale) -> Vec<ReviewCard> {
    let resource = match locale {
        Locale::vi => VIETNAMESE_RESOURCE,
        Locale::en => ENGLISH_RESOURCE,
    };

    serde_json::from_str(resource).expect("CV review resource JSON must be valid")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn english_resource_contains_expected_review_cards() {
        let cards = review_cards(Locale::en);
        assert_eq!(cards.len(), 67);
        assert_eq!(cards[0].title, "OOP");
        assert_eq!(cards[0].section, "1 — Foundation");
        assert!(cards[0].content.contains("Abstraction"));
    }

    #[test]
    fn vietnamese_resource_contains_expected_review_cards() {
        let cards = review_cards(Locale::vi);
        assert_eq!(cards.len(), 67);
        assert_eq!(cards[0].title, "OOP");
        assert_eq!(cards[0].section, "1 — Nền tảng");
        assert!(cards[0].content.contains("Trừu tượng"));
    }
}
