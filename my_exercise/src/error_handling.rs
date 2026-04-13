//! Exercices sur la gestion d'erreurs (Chapitre 9)
//! - Result, Option, gestion d'erreurs

use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidNumber,
    OutOfRange,
    InvalidFormat,
    EmptyInput,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::InvalidNumber => write!(f, "Nombre invalide"),
            ParseError::OutOfRange => write!(f, "Valeur hors limites"),
            ParseError::InvalidFormat => write!(f, "Format invalide"),
            ParseError::EmptyInput => write!(f, "Entrée vide"),
        }
    }
}

/// Parse une chaîne en i32
/// Retourne Err si chaîne vide ou non-numérique
/// # Exemple
/// assert_eq!(parse_i32("42"), Ok(42));
/// assert_eq!(parse_i32(""), Err(ParseError::EmptyInput));
pub fn parse_i32(s: &str) -> Result<i32, ParseError> {
    if s.trim().is_empty() {
        return Err(ParseError::EmptyInput);
    }
    // match s.trim().parse::<i32>() {
    //     Ok(num) => Ok(num),
    //     Err(_) => Err(ParseError::InvalidNumber),
    // }
    s.trim().parse::<i32>().map_err(|_| ParseError::InvalidNumber)
}

/// Parse une chaîne en nombre positif
/// Retourne Err si négatif ou non-numérique
/// # Exemple
/// assert_eq!(parse_positive("42"), Ok(42));
/// assert_eq!(parse_positive("-5"), Err(ParseError::OutOfRange));
pub fn parse_positive(s: &str) -> Result<i32, ParseError> {
    let s = s.trim();
    if s.is_empty() {
        return Err(ParseError::EmptyInput);
    }
    s.parse::<i32>().map_err(|_| ParseError::InvalidNumber).and_then(|n| {
        match n {
        n if n >= 0 => Ok(n),
        _ => Err(ParseError::OutOfRange),
    }
    })
}

/// Parse une chaîne en nombre dans une plage [min, max]
/// # Exemple
/// assert_eq!(parse_in_range("5", 0, 10), Ok(5));
/// assert_eq!(parse_in_range("15", 0, 10), Err(ParseError::OutOfRange));
pub fn parse_in_range(s: &str, min: i32, max: i32) -> Result<i32, ParseError> {
    if s.trim().is_empty() {
        return Err(ParseError::EmptyInput);
    }
    let num = s.trim().parse::<i32>().map_err(|_| ParseError::InvalidNumber)?;
    
    match num {
        n if n >= min && n <= max => Ok(n),
        _ => Err(ParseError::OutOfRange),
    }
}

/// Divise deux nombres. Retourne Err si diviseur est 0
/// # Exemple
/// assert_eq!(safe_divide(10, 2), Ok(5));
/// assert!(safe_divide(10, 0).is_err());
pub fn safe_divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Division par zéro".to_string())
    } else {
        Ok(a / b)
    }
}

/// Parse une ligne au format "nom,age"
/// # Exemple
/// assert_eq!(parse_person("Alice, 30"), Ok(("Alice".to_string(), 30)));
pub fn parse_person(line: &str) -> Result<(String, u32), ParseError> {
    // let parts: Vec<&str> = line.split(',').collect();
    // if parts.len() != 2 { return Err(ParseError::InvalidFormat); }

    // let age = parts[1].trim().parse::<u32>().map_err(|_| ParseError::InvalidNumber)?;

    // Ok((parts[0].to_string(), age))
    let (name, age) = line.split_once(',').ok_or(ParseError::InvalidFormat)?;
    let age = age.trim().parse::<u32>().map_err(|_| ParseError::InvalidNumber)?;
    Ok((name.trim().to_string(), age))
}

/// Retourne le premier élément du vecteur ou l'erreur spécifiée
/// # Exemple
/// assert_eq!(first_or_error(&[1, 2, 3], "vide".to_string()), Ok(1));
pub fn first_or_error<T: Clone>(vec: &[T], error: String) -> Result<T, String> {
    vec.first().cloned().ok_or(error)
}

/// Combine deux Results en les additionnant si tous deux sont Ok
/// # Exemple
/// assert_eq!(combine_results(Ok(5), Ok(3)), Ok(8));
pub fn combine_results(a: Result<i32, String>, b: Result<i32, String>) -> Result<i32, String> {
    // match (a, b) {
    //     (Err(error), _) => Err(error),
    //     (_, Err(error)) => Err(error),
    //     (Ok(x), Ok(y)) => Ok(x + y),
    // }
    Ok(a? + b?)
}

/// Retourne Ok si tous les nombres sont positifs, Err(msg) sinon
/// # Exemple
/// assert!(all_positive(&[1, 2, 3]).is_ok());
/// assert!(all_positive(&[1, -2, 3]).is_err());
pub fn all_positive(vec: &[i32]) -> Result<(), String> {
    for &item in vec {
        if item < 0 {
            return Err(format!("Nombre négatif trouvé: {}", item));
        }
    }
    Ok(())
}

/// Retry une opération 3 fois (implémentation fournie)
pub fn retry<F, T, E>(mut f: F) -> Result<T, E>
where
    F: FnMut() -> Result<T, E>,
{
    for _ in 0..3 {
        match f() {
            Ok(val) => return Ok(val),
            Err(_) => continue,
        }
    }
    f()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_i32() {
        assert_eq!(parse_i32("42"), Ok(42));
        assert_eq!(parse_i32(""), Err(ParseError::EmptyInput));
        assert_eq!(parse_i32("abc"), Err(ParseError::InvalidNumber));
    }

    #[test]
    fn test_parse_positive() {
        assert_eq!(parse_positive("42"), Ok(42));
        assert_eq!(parse_positive("0"), Ok(0));
        assert_eq!(parse_positive("-5"), Err(ParseError::OutOfRange));
    }

    #[test]
    fn test_parse_in_range() {
        assert_eq!(parse_in_range("5", 0, 10), Ok(5));
        assert_eq!(parse_in_range("15", 0, 10), Err(ParseError::OutOfRange));
        assert_eq!(parse_in_range("-1", 0, 10), Err(ParseError::OutOfRange));
    }

    #[test]
    fn test_safe_divide() {
        assert_eq!(safe_divide(10, 2), Ok(5));
        assert!(safe_divide(10, 0).is_err());
    }

    #[test]
    fn test_parse_person() {
        let result = parse_person("Alice, 30");
        assert_eq!(result, Ok(("Alice".to_string(), 30)));

        assert_eq!(
            parse_person("Alice, abc"),
            Err(ParseError::InvalidNumber)
        );
        assert_eq!(parse_person("Alice"), Err(ParseError::InvalidFormat));
    }

    #[test]
    fn test_first_or_error() {
        assert_eq!(first_or_error(&[1, 2, 3], "vide".to_string()), Ok(1));
        let empty: Vec<i32> = vec![];
        assert_eq!(
            first_or_error(&empty, "vide".to_string()),
            Err("vide".to_string())
        );
    }

    #[test]
    fn test_combine_results() {
        assert_eq!(combine_results(Ok(5), Ok(3)), Ok(8));
        assert!(combine_results(Ok(5), Err("erreur".to_string())).is_err());
    }

    #[test]
    fn test_all_positive() {
        assert!(all_positive(&[1, 2, 3]).is_ok());
        assert!(all_positive(&[1, -2, 3]).is_err());
    }
}
