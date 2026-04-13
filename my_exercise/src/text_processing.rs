//! Exercices avancés sur la manipulation de texte (Chapitre 8)
//! - Strings, slices, itérateurs

/// Structure pour analyser un texte
#[derive(Debug)]
pub struct TextAnalyzer {
    pub text: String,
}

impl TextAnalyzer {
    /// Crée un nouveau TextAnalyzer
    pub fn new(text: &str) -> Self {
        Self { text: text.to_string() }
    }

    /// Compte le nombre de lignes
    pub fn line_count(&self) -> usize {
        self.text.lines().count()
    }

    /// Compte le nombre de mots
    pub fn word_count(&self) -> usize {
        self.text.split_whitespace().count()
    }

    /// Compte le nombre de caractères (sans espaces)
    pub fn char_count(&self) -> usize {
        // self.text.split_whitespace().collect::<Vec<&str>>().join("").chars().count()
        self.text.chars().filter(|c| !c.is_whitespace()).count()
    }

    /// Compte le nombre de voyelles
    pub fn vowel_count(&self) -> usize {
        self.text
            .chars()
            .filter(|&c| matches!(c.to_lowercase().next(), Some('a' | 'e' | 'i' | 'o' | 'u' | 'y')))
            .count()
    }

    /// Compte le nombre de consonnes
    pub fn consonant_count(&self) -> usize {
        self.text
            .chars()
            .filter(|&c| c.is_alphabetic() && !matches!(c.to_lowercase().next(), Some('a' | 'e' | 'i' | 'o' | 'u' | 'y')))
            .count()
    }

    /// Retourne les lignes
    pub fn lines(&self) -> Vec<&str> {
        self.text.lines().collect()
    }

    /// Retourne les paragraphes (séparés par une ligne vide)
    pub fn paragraphs(&self) -> Vec<Vec<&str>> {
        self.text.split("\n\n").map(|p| p.lines().collect()).collect()
    }
}

/// Vérifie si une chaîne est un palindrome
// pub fn is_palindrome(text: &str) -> bool {
//     let cleaned: String = text.chars().filter(|c| c.is_alphanumeric()).map(|c| c.to_ascii_lowercase()).collect();
//     cleaned == cleaned.chars().rev().collect::<String>()
// }
pub fn is_palindrome(text: &str) -> bool {
    let chars = text.chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase());

    chars.clone().eq(chars.rev())
}

/// Remplace les caractères spécifiques
// pub fn replace_multiple(text: &str, replacements: &[(&str, &str)]) -> String {
//     let mut res = text.to_string();
//     for &(from, to) in replacements {
//         res = res.replace(from, to);
//     }
//     res
// }
pub fn replace_multiple(text: &str, replacements: &[(&str, &str)]) -> String {
    replacements.iter().fold(text.to_string(), |acc, (from, to)| {
        acc.replace(from, to)
    })
}

/// Extrait les initiales
pub fn extract_initials(text: &str) -> String {
    text.split_whitespace()
        .filter_map(|word| word.chars().next())
        .map(|c| c.to_ascii_uppercase().to_string())
        .collect::<Vec<String>>()
        .join(".")
}

/// Compte les phrases (finissant par . ! ?)
pub fn sentence_count(text: &str) -> usize {
    text.matches(['.', '!', '?']).count()
}

/// Trouvé l'indice de la première occurrence (case-insensitive)
pub fn find_case_insensitive(text: &str, needle: &str) -> Option<usize> {
    text.to_lowercase().find(&needle.to_lowercase())
}

/// Compte les occurrences d'une sous-chaîne
pub fn count_substring(text: &str, substring: &str) -> usize {
    text.matches(substring).count()
}

/// Centre le texte dans une largeur donnée
pub fn center_text(text: &str, width: usize) -> String {
    let len = text.chars().count();
    if len >= width {
        return text.to_string();
    }
    let padding = width - len;
    let left_pad = padding / 2;
    let right_pad = padding - left_pad;
    format!("{}{}{}", " ".repeat(left_pad), text, " ".repeat(right_pad))
}

/// Formate un texte en colonnes
pub fn format_columns(items: &[&str], num_columns: usize) -> String {
    todo!()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_analyzer() {
        let text = "Hello\nWorld\nRust".trim();
        let analyzer = TextAnalyzer::new(text);
        assert_eq!(analyzer.line_count(), 3);
    }

    #[test]
    fn test_word_count() {
        let text = "The quick brown fox";
        let analyzer = TextAnalyzer::new(text);
        assert_eq!(analyzer.word_count(), 4);
    }

    #[test]
    fn test_char_count() {
        let text = "Hello World";
        let analyzer = TextAnalyzer::new(text);
        assert_eq!(analyzer.char_count(), 10);
    }

    #[test]
    fn test_vowel_count() {
        let text = "hello";
        let analyzer = TextAnalyzer::new(text);
        assert_eq!(analyzer.vowel_count(), 2);
    }

    #[test]
    fn test_consonant_count() {
        let text = "hello";
        let analyzer = TextAnalyzer::new(text);
        assert_eq!(analyzer.consonant_count(), 3);
    }

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome("Ava"));
        assert!(is_palindrome("A man a plan a canal Panama"));
        assert!(!is_palindrome("hello"));
    }

    #[test]
    fn test_replace_multiple() {
        let text = "hello world";
        let replacements = &[("hello", "hi"), ("world", "rust")];
        assert_eq!(replace_multiple(text, replacements), "hi rust");
    }

    #[test]
    fn test_extract_initials() {
        assert_eq!(extract_initials("John Doe Smith"), "J.D.S");
        assert_eq!(extract_initials("Jean Luc Picard"), "J.L.P");
    }

    #[test]
    fn test_sentence_count() {
        assert_eq!(sentence_count("Hello. World!"), 2);
        assert_eq!(sentence_count("What? Really? Yes!"), 3);
    }

    #[test]
    fn test_find_case_insensitive() {
        assert_eq!(find_case_insensitive("Hello World", "world"), Some(6));
        assert_eq!(find_case_insensitive("Hello World", "xyz"), None);
    }

    #[test]
    fn test_count_substring() {
        assert_eq!(count_substring("banana", "an"), 2);
        assert_eq!(count_substring("hello", "o"), 1);
    }

    #[test]
    fn test_center_text() {
        let centered = center_text("hi", 10);
        assert_eq!(centered.len(), 10);
        assert!(centered.contains("hi"));
    }
}
