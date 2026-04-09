//! Exercices sur les traits et génériques (Chapitre 10)
//! - Traits, types génériques, implémentations

use std::fmt;

/// Trait pour les formes géométriques
pub trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn describe(&self) -> String {
        format!(
            "Aire: {:.2}, Périmètre: {:.2}",
            self.area(),
            self.perimeter()
        )
    }
}

/// Rectangle
#[derive(Debug, Clone)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        todo!()
    }

    fn perimeter(&self) -> f64 {
        todo!()
    }
}

/// Cercle
#[derive(Debug, Clone)]
pub struct Circle {
    pub radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        todo!()
    }

    fn perimeter(&self) -> f64 {
        todo!()
    }
}

/// Trait pour les conteneurs
pub trait Container<T> {
    fn push(&mut self, item: T);
    fn pop(&mut self) -> Option<T>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Conteneur générique simple
#[derive(Debug, Clone)]
pub struct SimpleContainer<T> {
    items: Vec<T>,
}

impl<T> SimpleContainer<T> {
    /// Crée un nouveau conteneur vide
    pub fn new() -> Self {
        todo!()
    }
}

impl<T> Default for SimpleContainer<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Container<T> for SimpleContainer<T> {
    fn push(&mut self, item: T) {
        self.items.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    fn len(&self) -> usize {
        self.items.len()
    }
}

/// Trait pour les éléments comparables
pub trait Comparable {
    fn is_greater_than(&self, other: &Self) -> bool;
    fn is_less_than(&self, other: &Self) -> bool;
    fn is_equal_to(&self, other: &Self) -> bool;
}

impl Comparable for i32 {
    fn is_greater_than(&self, other: &Self) -> bool {
        self > other
    }

    fn is_less_than(&self, other: &Self) -> bool {
        self < other
    }

    fn is_equal_to(&self, other: &Self) -> bool {
        self == other
    }
}

impl Comparable for String {
    fn is_greater_than(&self, other: &Self) -> bool {
        self > other
    }

    fn is_less_than(&self, other: &Self) -> bool {
        self < other
    }

    fn is_equal_to(&self, other: &Self) -> bool {
        self == other
    }
}

/// Traite les nombres itérables
pub trait Summable {
    fn sum_all(&self) -> i64;
    fn average(&self) -> f64 {
        todo!()
    }
    fn count_items(&self) -> usize;
}

impl Summable for Vec<i32> {
    fn sum_all(&self) -> i64 {
        todo!()
    }

    fn count_items(&self) -> usize {
        self.len()
    }
}

/// Retourne le maximum générique avec un trait bound
pub fn find_max<T: Comparable + Clone>(items: &[T]) -> Option<T> {
    todo!()
}

/// Pair générique
#[derive(Debug, Clone, PartialEq)]
pub struct Pair<T> {
    pub first: T,
    pub second: T,
}

impl<T> Pair<T> {
    /// Crée une nouvelle Pair
    /// # Exemple
    /// let pair = Pair::new(1, 2);
    /// assert_eq!(pair.first, 1);
    pub fn new(first: T, second: T) -> Self {
        todo!()
    }

    /// Échange les deux valeurs et retourne la Pair échangée
    /// # Exemple
    /// let pair = Pair::new(1, 2).swap();
    /// assert_eq!(pair.first, 2);
    pub fn swap(self) -> Self {
        todo!()
    }
}

impl<T: fmt::Display> fmt::Display for Pair<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.first, self.second)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangle() {
        let rect = Rectangle {
            width: 4.0,
            height: 3.0,
        };
        assert_eq!(rect.area(), 12.0);
        assert_eq!(rect.perimeter(), 14.0);
    }

    #[test]
    fn test_circle() {
        let circle = Circle { radius: 1.0 };
        assert!((circle.area() - std::f64::consts::PI).abs() < 0.001);
        assert!((circle.perimeter() - 2.0 * std::f64::consts::PI).abs() < 0.001);
    }

    #[test]
    fn test_simple_container() {
        let mut container = SimpleContainer::new();
        assert!(container.is_empty());

        container.push(1);
        container.push(2);
        container.push(3);
        assert_eq!(container.len(), 3);

        assert_eq!(container.pop(), Some(3));
        assert_eq!(container.pop(), Some(2));
    }

    #[test]
    fn test_comparable() {
        let a = 5;
        let b = 10;
        assert!(b.is_greater_than(&a));
        assert!(a.is_less_than(&b));
        assert!(a.is_equal_to(&5));
    }

    #[test]
    fn test_find_max() {
        let numbers = vec![3, 1, 4, 1, 5, 9];
        assert_eq!(find_max(&numbers), Some(9));

        let strings = vec!["apple", "zebra", "banana"];
        assert_eq!(find_max(&strings.iter().map(|s| s.to_string()).collect::<Vec<_>>()), Some("zebra".to_string()));
    }

    #[test]
    fn test_pair() {
        let pair = Pair::new(1, 2);
        assert_eq!(pair.first, 1);
        assert_eq!(pair.second, 2);

        let swapped = pair.swap();
        assert_eq!(swapped.first, 2);
        assert_eq!(swapped.second, 1);
    }

    #[test]
    fn test_summable() {
        let numbers = vec![1, 2, 3, 4, 5];
        assert_eq!(numbers.sum_all(), 15);
        assert_eq!(numbers.average(), 3.0);
    }
}
