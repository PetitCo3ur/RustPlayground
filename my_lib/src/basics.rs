//! Exercices sur les fondamentaux (Chapitres 1-5)
//! - Variables, fonctions, types, structures de contrôle

/// Calcule la somme de deux nombres
/// # Exemple
/// assert_eq!(add(2, 3), 5);
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Calcule la différence de deux nombres
/// # Exemple
/// assert_eq!(subtract(10, 3), 7);
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

/// Calcule le produit de deux nombres
/// # Exemple
/// assert_eq!(multiply(4, 5), 20);
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

/// Divise deux nombres. Retourne 0 si diviseur est 0
/// # Exemple
/// assert_eq!(divide(10, 2), 5);
/// assert_eq!(divide(10, 0), 0);
pub fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        0
    } else {
        a / b
    }
}

/// Retourne le reste de la division. Retourne 0 si diviseur est 0
/// # Exemple
/// assert_eq!(modulo(10, 3), 1);
pub fn modulo(a: i32, b: i32) -> i32 {
    if b == 0 {
        0
    } else {
        a % b
    }
}

/// Retourne la valeur absolue d'un nombre
/// # Exemple
/// assert_eq!(absolute(-5), 5);
pub fn absolute(n: i32) -> i32 {
    if n < 0 {
        -n
    } else {
        n
    }
}

/// Vérifie si un nombre est positif, négatif ou zéro
/// Retourne: "zéro", "positif" ou "négatif"
/// # Exemple
/// assert_eq!(check_number_sign(5), "positif");
pub fn check_number_sign(n: i32) -> &'static str {
    match n {
        n if n > 0 => "positif",
        n if n < 0 => "négatif",
        _ => "zéro",
    }
}

/// Calcule la factorielle d'un nombre (n!)
/// 0! = 1, 1! = 1, 5! = 120, etc.
/// # Exemple
/// assert_eq!(factorial(5), 120);
pub fn factorial(n: u32) -> u64 {
    let mut result: u64 = 1;
    for i in 1..=n {
        result *= i as u64;
    }
    result
}

/// Vérifie si un nombre est pair (divisible par 2)
/// # Exemple
/// assert!(is_even(4));
/// assert!(!is_even(5));
pub fn is_even(n: i32) -> bool {
    n % 2 == 0
}

/// Vérifie si un nombre est impair (non divisible par 2)
/// # Exemple
/// assert!(is_odd(5));
/// assert!(!is_odd(4));
pub fn is_odd(n: i32) -> bool {
    n % 2 != 0
}

/// Vérifie si un nombre est dans une plage [min, max] inclus
/// # Exemple
/// assert!(is_in_range(5, 0, 10));
/// assert!(!is_in_range(11, 0, 10));
pub fn is_in_range(n: i32, min: i32, max: i32) -> bool {
    n >= min && n <= max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(-5, -3), -8);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(10, 3), 7);
        assert_eq!(subtract(3, 10), -7);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(4, 5), 20);
        assert_eq!(multiply(-2, 3), -6);
        assert_eq!(multiply(0, 100), 0);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(10, 2), 5);
        assert_eq!(divide(10, 0), 0);
    }

    #[test]
    fn test_modulo() {
        assert_eq!(modulo(10, 3), 1);
        assert_eq!(modulo(10, 0), 0);
    }

    #[test]
    fn test_absolute() {
        assert_eq!(absolute(5), 5);
        assert_eq!(absolute(-5), 5);
        assert_eq!(absolute(0), 0);
    }

    #[test]
    fn test_check_number_sign() {
        assert_eq!(check_number_sign(5), "positif");
        assert_eq!(check_number_sign(-5), "négatif");
        assert_eq!(check_number_sign(0), "zéro");
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(10), 3628800);
    }

    #[test]
    fn test_is_even() {
        assert!(is_even(4));
        assert!(!is_even(5));
        assert!(is_even(0));
    }

    #[test]
    fn test_is_odd() {
        assert!(!is_odd(4));
        assert!(is_odd(5));
        assert!(!is_odd(0));
    }

    #[test]
    fn test_is_in_range() {
        assert!(is_in_range(5, 0, 10));
        assert!(is_in_range(0, 0, 10));
        assert!(is_in_range(10, 0, 10));
        assert!(!is_in_range(11, 0, 10));
        assert!(!is_in_range(-1, 0, 10));
    }
}
