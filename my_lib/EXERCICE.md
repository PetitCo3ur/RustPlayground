# 🦀 Exercices Rust - Chapitres 1-11

Suite complète d'exercices pour consolider les connaissances du **Rust Book jusqu'au chapitre 11**.

## 🎯 État actuel

Les exercices sont en **mode pratique**! Toutes les implémentations ont été supprimées et remplacées par `todo!()`. À toi d'implémenter les 57 fonctions manquantes.

---

## 📊 Statistiques

| Métrique | Valeur |
|----------|--------|
| **Tests unitaires** | 63 ❌ |
| **Tests d'intégration** | 12 ❌ |
| **Total tests** | 75 ❌ |
| **Fonctions à implémenter** | 57 |
| **Modules** | 6 📦 |
| **Chapitres couverts** | 1-11 📚 |

---

## 🚀 Démarrage rapide

```bash
cd my_lib

# Voir les tests échouer
cargo test

# Vérifier un module spécifique
cargo test basics::tests
```

---

## 📚 Structure des modules

### 1. **basics.rs** - Fondamentaux (Chapitres 1-5)
**Concepts:** Variables, fonctions, types, structures de contrôle  
**11 fonctions:** `add`, `subtract`, `multiply`, `divide`, `modulo`, `absolute`, `check_number_sign`, `factorial`, `is_even`, `is_odd`, `is_in_range`

```rust
/// Calcule la somme de deux nombres
pub fn add(a: i32, b: i32) -> i32 {
    todo!()
}

#[test]
fn test_add() {
    assert_eq!(add(2, 3), 5);
    assert_eq!(add(-1, 1), 0);
}
```

---

### 2. **control_flow.rs** - Pattern Matching et Énums (Chapitres 3, 6)
**Concepts:** Énums, pattern matching, match-expressions, Options  
**8 fonctions/méthodes:** 
- `DayOfWeek::name()` et `is_working_day()`
- `Temperature::to_celsius()`, `to_fahrenheit()`, `feeling()`
- `Color::hex_code()`
- `letter_grade()`, `grade_description()`, `unwrap_or_default()`, `max_if_both_some()`

```rust
#[derive(Debug, PartialEq)]
pub enum DayOfWeek { Monday, Tuesday, ... }

impl DayOfWeek {
    pub fn name(&self) -> &str {
        todo!()  // Retourne "Lundi", "Mardi", etc.
    }
}
```

---

### 3. **collections.rs** - Collections (Chapitre 8)
**Concepts:** Vec, String, HashMap, itérateurs, closures  
**17 fonctions:** 
- Vecteurs: `sum_vector()`, `average()`, `min_max()`, `duplicate_elements()`, `filter_even()`, `filter_odd()`, `count_occurrences()`, `unique_elements()`, `reverse_vector()`
- Strings: `word_frequency()`, `to_uppercase()`, `to_lowercase()`, `char_count()`, `extract_words()`, `reverse_words()`, `capitalize()`

```rust
/// Retourne la somme de tous les éléments
pub fn sum_vector(vec: &[i32]) -> i32 {
    todo!()
}

#[test]
fn test_sum_vector() {
    assert_eq!(sum_vector(&[1, 2, 3, 4, 5]), 15);
}
```

---

### 4. **error_handling.rs** - Gestion d'erreurs (Chapitre 9)
**Concepts:** Result, Option, gestion d'erreurs, opérateur `?`  
**9 fonctions:** 
- `parse_i32()`, `parse_positive()`, `parse_in_range()` - Parsing avec validation
- `safe_divide()` - Division sécurisée
- `parse_person()` - Parser format "nom,age"
- `first_or_error()`, `combine_results()`, `all_positive()` - Combinaisons de Result/Option
- `retry()` - Retry avec closure (implémentation fournie)

```rust
pub enum ParseError {
    InvalidNumber,
    OutOfRange,
    InvalidFormat,
    EmptyInput,
}

/// Parse une chaîne en i32
pub fn parse_i32(s: &str) -> Result<i32, ParseError> {
    todo!()
}

#[test]
fn test_parse_i32() {
    assert_eq!(parse_i32("42"), Ok(42));
    assert_eq!(parse_i32(""), Err(ParseError::EmptyInput));
}
```

---

### 5. **traits_generics.rs** - Traits et Génériques (Chapitre 10)
**Concepts:** Traits, implémentations, génériques, trait bounds  
**Traits à implémenter:**
- `Shape` impl pour `Rectangle` et `Circle`
- `Container<T>` impl pour `SimpleContainer<T>`
- `Comparable` impl pour `i32` et `String`
- `Summable` impl pour `Vec<i32>`

**Fonctions/méthodes:**
- `SimpleContainer::new()`, `Pair::new()`, `Pair::swap()`
- `find_max<T>()`

```rust
pub trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

pub struct Rectangle { pub width: f64, pub height: f64 }

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        todo!()
    }
}
```

---

### 6. **text_processing.rs** - Analyse de texte (Chapitre 8+)
**Concepts:** String avancée, slices, itérateurs, analyse linguistique  
**12 fonctions:**
- `TextAnalyzer::new()`, `line_count()`, `word_count()`, `char_count()`, `vowel_count()`, `consonant_count()`, `lines()`, `paragraphs()`
- `is_palindrome()`, `replace_multiple()`, `extract_initials()`, `sentence_count()`, `find_case_insensitive()`, `count_substring()`, `center_text()`, `format_columns()`

```rust
pub struct TextAnalyzer { pub text: String }

impl TextAnalyzer {
    pub fn new(text: &str) -> Self {
        todo!()
    }
    
    pub fn line_count(&self) -> usize {
        todo!()
    }
}
```

---

## 🎓 Plan de progression

### Jour 1: Fondamentaux
```bash
cargo test basics::tests              # 11 tests
# Opérations arithmétiques, conditions simples
```

### Jour 2: Énums et Pattern Matching
```bash
cargo test control_flow::tests        # 8 tests
# Match-expressions, énums, Options
```

### Jour 3: Collections
```bash
cargo test collections::tests         # 17 tests
# Vec, String, HashMap, itérateurs, closures
```

### Jour 4: Gestion d'erreurs
```bash
cargo test error_handling::tests      # 9 tests
# Result, Option, parsing, custom errors
```

### Jour 5: Traits et Génériques
```bash
cargo test traits_generics::tests     # 9 tests
# Traits, implémentations, types génériques
```

### Jour 6: Analyse de texte
```bash
cargo test text_processing::tests     # 12 tests
# String avancée, itérateurs, analyse
```

### Jour 7: Intégration
```bash
cargo test --test integration_test_complete  # 12 tests
# Combiner plusieurs modules
```

---

## 🔧 Comment démarrer une fonction

**Étape 1:** Ouvre le module
```bash
code src/basics.rs
```

**Étape 2:** Trouve la fonction avec `todo!()`
```rust
pub fn add(a: i32, b: i32) -> i32 {
    todo!()  // ← À implémenter
}
```

**Étape 3:** Lis la doc et les tests
```rust
/// Calcule la somme de deux nombres
#[test]
fn test_add() {
    assert_eq!(add(2, 3), 5);  // ← Specs exactes
}
```

**Étape 4:** Implémente
```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b  // ✅ Implémentation
}
```

**Étape 5:** Teste
```bash
cargo test basics::tests::test_add -- --exact
```

---

## 🏃 Commandes utiles

### Exécuter tests d'un module
```bash
cargo test basics::tests           # Tous les tests de basics
cargo test collections::tests      # Tous les tests de collections
```

### Tester une fonction spécifique
```bash
cargo test test_add -- --exact     # Juste test_add
```

### Voir les détails
```bash
cargo test -- --nocapture         # Affiche les println!
cargo test -- --test-threads=1    # Test séquentiel
RUST_BACKTRACE=1 cargo test       # Backtrace complet
```

### Juste compiler sans tester
```bash
cargo check                        # Vérifier la syntaxe
```

---

## 💡 Conseils pour réussir

### 1. Les tests sont des specs
```rust
// Le test te dit EXACTEMENT ce que tu dois implémenter
#[test]
fn test_factorial() {
    assert_eq!(factorial(0), 1);   // 0! = 1
    assert_eq!(factorial(5), 120); // 5! = 120
}
```

### 2. Commence simple
```rust
// ✅ Bon
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// ❌ Évite les solutions compliquées
```

### 3. Lis les commentaires
```rust
/// Divise deux nombres. Retourne 0 si diviseur est 0
/// → Le commentaire EST ta spécification!
pub fn divide(a: i32, b: i32) -> i32 {
    todo!()
}
```

### 4. Une fonction à la fois
```bash
# Implémenter + Tester une fonction
# Puis passer à la suivante
cargo test test_divide -- --exact
```

### 5. Observe les erreurs
```bash
# Rust te dit EXACTEMENT où est le problème
error: not yet implemented
thread 'test_add' panicked at 'not yet implemented'
```

---

## 🎯 Concepts clés par module

| Module | Concepts clés |
|--------|---------------|
| **basics** | Variables, types, fonctions, if/else, boucles |
| **control_flow** | Énums, match, patterns, Option |
| **collections** | Vec, String, HashMap, itérateurs, closures |
| **error_handling** | Result, Option, custom errors, ? operator |
| **traits_generics** | Traits, implémentations, génériques, bounds |
| **text_processing** | String avancée, chars, lines, parsing |

---

## ✨ Points d'excellence

Chaque fonction que tu implémentes t'apprend un concept Rust:

- **add()** → Fonctions, arithmétique, retour
- **check_number_sign()** → Match-expressions, guards
- **filter_even()** → Closures, itérateurs, filter
- **parse_i32()** → Result, map_err, gestion d'erreurs
- **Shape trait** → Traits, implémentations, polymorphisme
- **is_palindrome()** → String, chars, reverse

---

## 📖 Ressources

- [Livre Rust officiel](https://doc.rust-lang.org/book/) - Ch. 1-11
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Standard Library](https://doc.rust-lang.org/std/)
- [Rustlings](https://github.com/rust-lang/rustlings)

---

## 🚀 C'est parti!

```bash
cd my_lib
cargo test basics::tests

# Vois les 11 tests échouer
# Commence par le premier: add()
# Une par une, fais-les passer!
```

**57 fonctions t'attendent! 🦀**

Bonne chance! You got this! 💪

---

**Créé:** Avril 2026  
**État:** Mode PRATIQUE ✅  
**À faire:** 57 fonctions  
**Temps estimé:** 7 jours
