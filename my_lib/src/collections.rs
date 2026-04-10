//! Exercices sur les collections (Chapitre 8)
//! - Vec, String, HashMap

use std::{char, collections::HashMap};

/// Retourne la somme de tous les éléments d'un vecteur
/// # Exemple
/// assert_eq!(sum_vector(&[1, 2, 3, 4, 5]), 15);
pub fn sum_vector(vec: &[i32]) -> i32 {
    // let mut result= 0;
    // for item in vec {
    //     result += item;
    // }
    // result
    vec.iter().sum()
}

/// Retourne la moyenne des éléments en f64. Retourne 0.0 si vecteur vide
/// # Exemple
/// assert_eq!(average(&[2, 4, 6]), 4.0);
pub fn average(vec: &[i32]) -> f64 {
    if vec.len() == 0 { return 0.0; }

    // let mut result:f64 = 0.0;
    // for item in vec {
    //     result += *item as f64;
    // }
    // result / vec.len() as f64

    let sum: i32 = vec.iter().sum();
    sum as f64 / vec.len() as f64
}

/// Retourne un tuple (min, max) ou None si vecteur vide
/// # Exemple
/// assert_eq!(min_max(&[3, 1, 4, 1, 5]), Some((1, 5)));
pub fn min_max(vec: &[i32]) -> Option<(i32, i32)> {
    if vec.is_empty() { return None; }

    let (mut min, mut max) = (vec[0], vec[0]);
    for &item in vec {
        min = min.min(item);
        max = max.max(item);
    }
    Some((min, max))
}

/// Duplique chaque élément du vecteur une fois
/// [1, 2, 3] -> [1, 1, 2, 2, 3, 3]
/// # Exemple
/// assert_eq!(duplicate_elements(&[1, 2, 3]), vec![1, 1, 2, 2, 3, 3]);
pub fn duplicate_elements(vec: &[i32]) -> Vec<i32> {
    // let mut res = Vec::new();
    // for &item in vec {
    //     res.push(item);
    //     res.push(item);
    // }
    // res
    vec.iter().flat_map(|&x| [x, x]).collect()
}

/// Retourne un nouveau vecteur avec seulement les nombres pairs
/// # Exemple
/// assert_eq!(filter_even(&[1, 2, 3, 4, 5]), vec![2, 4]);
pub fn filter_even(vec: &[i32]) -> Vec<i32> {
    vec.iter().copied().filter(|&x| x % 2 == 0).collect()
}

/// Retourne un nouveau vecteur avec seulement les nombres impairs
/// # Exemple
/// assert_eq!(filter_odd(&[1, 2, 3, 4, 5]), vec![1, 3, 5]);
pub fn filter_odd(vec: &[i32]) -> Vec<i32> {
    // let mut res = Vec::new();

    // for &item in vec {
    //     if item % 2 == 1 { res.push(item)}
    // }
    // res
    vec.iter().copied().filter(|&x| x % 2 == 1).collect()
}

/// Compte les occurrences de chaque élément dans une HashMap
/// # Exemple
/// let vec = vec![1, 2, 2, 3, 3, 3];
/// let result = count_occurrences(&vec);
/// assert_eq!(result.get(&1), Some(&1));
/// assert_eq!(result.get(&3), Some(&3));
pub fn count_occurrences(vec: &[i32]) -> HashMap<i32, u32> {
    let mut res: HashMap<i32, u32> = HashMap::new();
    
    for &item in vec {
        *res.entry(item).or_insert(0) += 1;
    }
    res
}

/// Retourne les éléments uniques, triés
/// # Exemple
/// assert_eq!(unique_elements(&[1, 2, 2, 3, 3, 3]), vec![1, 2, 3]);
pub fn unique_elements(vec: &[i32]) -> Vec<i32> {
    // let mut res = Vec::new();

    // for &item in vec {
    //     if !res.contains(&item) {
    //         res.push(item);
    //     }
    // }

    // let mut set: HashSet<i32> = vec.iter().copied().collect();
    // let mut res: Vec<i32> = set.drain().collect();

    let mut res: Vec<i32> = vec.iter().copied().collect();
    res.sort();
    res.dedup();
    res
}

/// Inverse l'ordre des éléments
/// # Exemple
/// assert_eq!(reverse_vector(&[1, 2, 3]), vec![3, 2, 1]);
pub fn reverse_vector(vec: &[i32]) -> Vec<i32> {
    // let mut res = Vec::new();
    // for &item in vec.iter().rev() {
    //     res.push(item);
    // }
    // res
    vec.iter().copied().rev().collect()
}

/// Compte la fréquence de chaque mot (HashMap<&str, u32>)
/// Les mots sont séparés par les espaces
/// # Exemple
/// let text = "bonjour monde bonjour";
/// let result = word_frequency(text);
/// assert_eq!(result.get("bonjour"), Some(&2));
pub fn word_frequency(text: &str) -> HashMap<&str, u32> {
    let mut res: HashMap<&str, u32> = HashMap::new();
    for word in text.split_whitespace() {
        *res.entry(word).or_insert(0) += 1;
    }
    res
}

/// Convertit une chaîne en majuscules
/// # Exemple
/// assert_eq!(to_uppercase("hello"), "HELLO");
pub fn to_uppercase(text: &str) -> String {
    text.to_uppercase()
}

/// Convertit une chaîne en minuscules
/// # Exemple
/// assert_eq!(to_lowercase("HELLO"), "hello");
pub fn to_lowercase(text: &str) -> String {
    text.to_lowercase()
}

/// Compte le nombre de caractères (pas en bytes), excluant les espaces
/// # Exemple
/// assert_eq!(char_count("hello"), 5);
/// assert_eq!(char_count("café"), 4);
pub fn char_count(text: &str) -> usize {
    text.chars().count()
}

/// Retourne les mots séparés par les espaces
/// # Exemple
/// assert_eq!(extract_words("hello world rust"), vec!["hello", "world", "rust"]);
pub fn extract_words(text: &str) -> Vec<&str> {
    // let mut res = Vec::new();
    // for word in text.split_whitespace() {
    //     res.push(word);
    // }
    // res
    text.split_whitespace().collect()
}

/// Inverse l'ordre des mots
/// # Exemple
/// assert_eq!(reverse_words("hello world rust"), "rust world hello");
pub fn reverse_words(text: &str) -> String {
    // let mut res = String::new();
    // for word in text.split_whitespace().rev() {
    //     res.push_str(&(word.to_owned() + " "));
    // }
    
    // res.trim().to_string()
    text.split_whitespace().rev().collect::<Vec<&str>>().join(" ")
}

/// Retourne la première lettre en majuscule, le reste inchangé
/// # Exemple
/// assert_eq!(capitalize("hello"), "Hello");
/// assert_eq!(capitalize(""), "");
pub fn capitalize(text: &str) -> String {
    // if text.is_empty() {
    //     return String::new();
    // }
    // let mut chars = text.chars();
    // let up = chars.next().unwrap().to_ascii_uppercase().to_string();
    // let rest: String = chars.collect();
    // up + &rest
    let mut chars = text.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().to_string() + chars.as_str(),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_vector() {
        assert_eq!(sum_vector(&[1, 2, 3, 4, 5]), 15);
        assert_eq!(sum_vector(&[]), 0);
    }

    #[test]
    fn test_average() {
        assert_eq!(average(&[2, 4, 6]), 4.0);
        assert_eq!(average(&[]), 0.0);
    }

    #[test]
    fn test_min_max() {
        assert_eq!(min_max(&[3, 1, 4, 1, 5]), Some((1, 5)));
        assert_eq!(min_max(&[]), None);
    }

    #[test]
    fn test_duplicate_elements() {
        assert_eq!(duplicate_elements(&[1, 2, 3]), vec![1, 1, 2, 2, 3, 3]);
    }

    #[test]
    fn test_filter_even() {
        assert_eq!(filter_even(&[1, 2, 3, 4, 5]), vec![2, 4]);
    }

    #[test]
    fn test_filter_odd() {
        assert_eq!(filter_odd(&[1, 2, 3, 4, 5]), vec![1, 3, 5]);
    }

    #[test]
    fn test_count_occurrences() {
        let vec = vec![1, 2, 2, 3, 3, 3];
        let result = count_occurrences(&vec);
        assert_eq!(result.get(&1), Some(&1));
        // assert_eq!(result.get(&2), Some(&2));
        // assert_eq!(result.get(&3), Some(&3));
    }

    #[test]
    fn test_unique_elements() {
        assert_eq!(unique_elements(&[1, 2, 2, 3, 3, 3]), vec![1, 2, 3]);
    }

    #[test]
    fn test_reverse_vector() {
        assert_eq!(reverse_vector(&[1, 2, 3]), vec![3, 2, 1]);
    }

    #[test]
    fn test_word_frequency() {
        let text = "bonjour monde bonjour";
        let result = word_frequency(text);
        assert_eq!(result.get("bonjour"), Some(&2));
        assert_eq!(result.get("monde"), Some(&1));
    }

    #[test]
    fn test_to_uppercase() {
        assert_eq!(to_uppercase("hello"), "HELLO");
    }

    #[test]
    fn test_to_lowercase() {
        assert_eq!(to_lowercase("HELLO"), "hello");
    }

    #[test]
    fn test_char_count() {
        assert_eq!(char_count("hello"), 5);
        assert_eq!(char_count("café"), 4);
    }

    #[test]
    fn test_extract_words() {
        assert_eq!(
            extract_words("hello world rust"),
            vec!["hello", "world", "rust"]
        );
    }

    #[test]
    fn test_reverse_words() {
        assert_eq!(reverse_words("hello world rust"), "rust world hello");
    }

    #[test]
    fn test_capitalize() {
        assert_eq!(capitalize("hello"), "Hello");
        assert_eq!(capitalize(""), "");
    }
}
