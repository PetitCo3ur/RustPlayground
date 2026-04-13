//! Tests d'intégration pour tous les modules
//! Combine plusieurs concepts

use my_lib::{basics, collections, control_flow, error_handling, traits_generics, text_processing};
use traits_generics::Shape;

#[test]
fn test_calculator_workflow() {
    // Test basique d'arithmétique
    let sum = basics::add(10, 5);
    let product = basics::multiply(sum, 2);
    assert_eq!(product, 30);

    // Utilisation des résultats
    assert!(basics::is_even(product));
}

#[test]
fn test_grade_workflow() {
    let scores = vec![85, 92, 78, 65];
    let letters: Vec<char> = scores.iter().map(|&s| control_flow::letter_grade(s)).collect();

    for &letter in &letters {
        let desc = control_flow::grade_description(letter);
        assert!(!desc.is_empty());
    }
}

#[test]
fn test_temperature_conversions() {
    let celsius = control_flow::Temperature::Celsius(25.0);
    let fahrenheit = celsius.to_fahrenheit();

    // Vérifier que la conversion inverse redonne la valeur originale
    let back_to_celsius = control_flow::Temperature::Fahrenheit(fahrenheit).to_celsius();
    assert!((back_to_celsius - 25.0).abs() < 0.001);
}

#[test]
fn test_vector_operations_pipeline() {
    let vec = vec![5, 2, 8, 2, 9, 2];

    // Obtenir statistiques
    let sum = collections::sum_vector(&vec);
    let average = collections::average(&vec);
    let (min, max) = collections::min_max(&vec).unwrap();

    assert_eq!(sum, 28);
    assert_eq!(average, 28.0 / 6.0);
    assert_eq!(min, 2);
    assert_eq!(max, 9);

    // Compter occurrences
    let occurrences = collections::count_occurrences(&vec);
    assert_eq!(occurrences.get(&2), Some(&3));
}

#[test]
fn test_word_analysis() {
    let text = "bonjour monde bonjour rust";
    let freq = collections::word_frequency(text);

    assert_eq!(freq.get("bonjour"), Some(&2));
    assert_eq!(freq.get("monde"), Some(&1));
    assert_eq!(freq.get("rust"), Some(&1));
}

#[test]
fn test_parsing_pipeline() {
    let input_str = "Alice, 30";

    // Parser la personne
    let result = error_handling::parse_person(input_str);
    assert!(result.is_ok());

    let (name, age) = result.unwrap();

    // Vérifier les données
    assert_eq!(name, "Alice");
    assert!(age > 0);

    // Transformer les données
    let capitalized = collections::capitalize(&name);
    assert_eq!(capitalized, "Alice");
}

#[test]
fn test_shape_analysis() {
    // Créer diverses formes
    let rect = traits_generics::Rectangle {
        width: 5.0,
        height: 3.0,
    };
    let circle = traits_generics::Circle { radius: 2.0 };

    // Utiliser le trait Shape
    let rect_area = rect.area();
    let circle_area = circle.area();

    // Comparer les aires
    assert!(rect_area < 20.0);
    assert!(circle_area > 10.0);
}

#[test]
fn test_text_analysis_complete() {
    let text = "Hello World\nRust is great\nLet's learn Rust";

    let analyzer = text_processing::TextAnalyzer::new(text);
    assert_eq!(analyzer.line_count(), 3);
    assert!(analyzer.word_count() > 6);
    assert!(analyzer.vowel_count() > 0);

    // Vérifier palindromes
    assert!(text_processing::is_palindrome("racecar"));
    assert!(!text_processing::is_palindrome("hello"));
}

#[test]
fn test_combined_error_handling() {
    // Parser plusieurs valeurs
    let inputs = vec!["10", "20", "30"];
    let results: Vec<Result<i32, _>> = inputs
        .iter()
        .map(|&s| error_handling::parse_i32(s))
        .collect();

    // Vérifier que tout est Ok
    let mut values = Vec::new();
    for result in results {
        if let Ok(val) = result {
            values.push(val);
        }
    }
    
    assert_eq!(values.len(), 3);
    let nums = values;
    let sum = collections::sum_vector(&nums);
    assert_eq!(sum, 60);
}

#[test]
fn test_generic_container() {
    use traits_generics::Container;

    let mut container: traits_generics::SimpleContainer<String> =
        traits_generics::SimpleContainer::new();

    container.push("hello".to_string());
    container.push("world".to_string());

    assert_eq!(container.len(), 2);
    assert!(!container.is_empty());

    let first = container.pop();
    assert_eq!(first, Some("world".to_string()));
}

#[test]
fn test_day_workflow() {
    let day = control_flow::DayOfWeek::Friday;

    // C'est un jour de travail ?
    assert!(day.is_working_day());

    // Quel est le nom ?
    assert_eq!(day.name(), "Vendredi");
}
