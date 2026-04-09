//! Exercices sur le contrôle de flux et le pattern matching (Chapitres 3, 6)
//! - Match-expressions, énums, options

#[derive(Debug, PartialEq)]
pub enum DayOfWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl DayOfWeek {
    /// Retourne le nom du jour en français
    /// # Exemple
    /// assert_eq!(DayOfWeek::Monday.name(), "Lundi");
    pub fn name(&self) -> &str {
        match self {
            DayOfWeek::Monday => "Lundi",
            DayOfWeek::Tuesday => "Mardi",
            DayOfWeek::Wednesday => "Mercredi",
            DayOfWeek::Thursday => "Jeudi",
            DayOfWeek::Friday => "Vendredi",
            DayOfWeek::Saturday => "Samedi",
            DayOfWeek::Sunday => "Dimanche",
        }
    }

    /// Retourne true si c'est un jour de travail (Lundi-Vendredi)
    /// # Exemple
    /// assert!(DayOfWeek::Monday.is_working_day());
    /// assert!(!DayOfWeek::Saturday.is_working_day());
    pub fn is_working_day(&self) -> bool {
        !matches!(self, DayOfWeek::Saturday | DayOfWeek::Sunday)
    }
}

#[derive(Debug, PartialEq)]
pub enum Temperature {
    Celsius(f64),
    Fahrenheit(f64),
}

impl Temperature {
    /// Convertit la température en Celsius
    /// Formule: (F - 32) * 5/9 = C
    /// # Exemple
    /// assert_eq!(Temperature::Fahrenheit(32.0).to_celsius(), 0.0);
    pub fn to_celsius(&self) -> f64 {
        match self {
            Temperature::Celsius(temp) => *temp,
            Temperature::Fahrenheit(temp) => (*temp - 32.0) * 5.0/9.0,
        }
    }

    /// Convertit la température en Fahrenheit
    /// Formule: C * 9/5 + 32 = F
    /// # Exemple
    /// assert_eq!(Temperature::Celsius(0.0).to_fahrenheit(), 32.0);
    pub fn to_fahrenheit(&self) -> f64 {
        match self {
            Temperature::Celsius(temp) => *temp * 9.0/5.0 + 32.0,
            Temperature::Fahrenheit(temp) => *temp,
        }
    }

    /// Retourne "chaud" (> 25°C), "tiède" (15-25°C) ou "froid" (< 15°C)
    /// # Exemple
    /// assert_eq!(Temperature::Celsius(30.0).feeling(), "chaud");
    pub fn feeling(&self) -> &str {
        let celsius = self.to_celsius();
        match celsius {
            temp if temp > 25.0 => "chaud",
            temp if temp >= 15.0 => "tiède",
            _ => "froid",
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Color {
    Red,
    Green,
    Blue,
    Custom(String),
}

impl Color {
    /// Retourne le code hexadécimal de la couleur
    /// Red -> "#FF0000", Green -> "#00FF00", Blue -> "#0000FF"
    /// Custom(code) -> le code fourni
    /// # Exemple
    /// assert_eq!(Color::Red.hex_code(), "#FF0000");
    pub fn hex_code(&self) -> String {
        match self {
            Color::Red => String::from("#FF0000"),
            Color::Green => String::from("#00FF00"),
            Color::Blue => String::from("#0000FF"),
            Color::Custom(code) => code.clone(),
        }
    }
}

/// Retourne la note lettre basée sur le score (0-100)
/// 90-100: A, 80-89: B, 70-79: C, 60-69: D, <60: F
/// # Exemple
/// assert_eq!(letter_grade(95), 'A');
/// assert_eq!(letter_grade(85), 'B');
pub fn letter_grade(score: u32) -> char {
    match score {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        _ => 'F',
    }
}

/// Retourne la description en français de la note lettre
/// A -> "Excellent", B -> "Très bien", C -> "Bien", D -> "Acceptable", F -> "Échoué"
/// Autre -> "Invalide"
/// # Exemple
/// assert_eq!(grade_description('A'), "Excellent");
pub fn grade_description(grade: char) -> &'static str {
    match grade {
        'A' => "Excellent",
        'B' => "Très bien",
        'C' => "Bien",
        'D' => "Acceptable",
        'F' => "Échoué",
        _ => "Invalide",
    }
}
       

/// Retourne la valeur de l'Option, ou la valeur par défaut si None
/// # Exemple
/// assert_eq!(unwrap_or_default(Some(5), 10), 5);
/// assert_eq!(unwrap_or_default(None, 10), 10);
pub fn unwrap_or_default<T: Clone>(option: Option<T>, default: T) -> T {
    option.unwrap_or(default)
}

/// Retourne le maximum si les deux Options contiennent Some, sinon None
/// # Exemple
/// assert_eq!(max_if_both_some(Some(5), Some(10)), Some(10));
/// assert_eq!(max_if_both_some(None, Some(10)), None);
pub fn max_if_both_some(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    match (a, b) {
        (Some(x), Some(y)) => Some(x.max(y)),
        _ => None
    }
    // a.and_then(|x| b.map(|y| x.max(y)))
    // a.zip(b).map(|(x, y)| x.max(y))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_of_week_name() {
        assert_eq!(DayOfWeek::Monday.name(), "Lundi");
        assert_eq!(DayOfWeek::Friday.name(), "Vendredi");
        assert_eq!(DayOfWeek::Sunday.name(), "Dimanche");
    }

    #[test]
    fn test_is_working_day() {
        assert!(DayOfWeek::Monday.is_working_day());
        assert!(DayOfWeek::Friday.is_working_day());
        assert!(!DayOfWeek::Saturday.is_working_day());
        assert!(!DayOfWeek::Sunday.is_working_day());
    }

    #[test]
    fn test_temperature_conversion() {
        let celsius = Temperature::Celsius(0.0);
        assert_eq!(celsius.to_fahrenheit(), 32.0);

        let fahrenheit = Temperature::Fahrenheit(32.0);
        assert_eq!(fahrenheit.to_celsius(), 0.0);
    }

    #[test]
    fn test_temperature_feeling() {
        assert_eq!(Temperature::Celsius(30.0).feeling(), "chaud");
        assert_eq!(Temperature::Celsius(20.0).feeling(), "tiède");
        assert_eq!(Temperature::Celsius(10.0).feeling(), "froid");
    }

    #[test]
    fn test_color_hex_code() {
        assert_eq!(Color::Red.hex_code(), "#FF0000");
        assert_eq!(Color::Green.hex_code(), "#00FF00");
        assert_eq!(
            Color::Custom("#ABC123".to_string()).hex_code(),
            "#ABC123"
        );
    }

    #[test]
    fn test_letter_grade() {
        assert_eq!(letter_grade(95), 'A');
        assert_eq!(letter_grade(85), 'B');
        assert_eq!(letter_grade(75), 'C');
        assert_eq!(letter_grade(65), 'D');
        assert_eq!(letter_grade(50), 'F');
    }

    #[test]
    fn test_grade_description() {
        assert_eq!(grade_description('A'), "Excellent");
        assert_eq!(grade_description('B'), "Très bien");
        assert_eq!(grade_description('F'), "Échoué");
    }

    #[test]
    fn test_unwrap_or_default() {
        assert_eq!(unwrap_or_default(Some(5), 10), 5);
        assert_eq!(unwrap_or_default(None, 10), 10);
    }

    #[test]
    fn test_max_if_both_some() {
        assert_eq!(max_if_both_some(Some(5), Some(10)), Some(10));
        assert_eq!(max_if_both_some(Some(15), Some(10)), Some(15));
        assert_eq!(max_if_both_some(None, Some(10)), None);
        assert_eq!(max_if_both_some(Some(5), None), None);
    }
}
