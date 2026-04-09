# RustPlayground

Notes personnelles pour apprendre Rust (book officiel), jusqu'a la fin du chapitre 11.

Documentation de reference:
- Rust Book: https://doc.rust-lang.org/book/
- Chapitre Strings (8.2): https://doc.rust-lang.org/book/ch08-02-strings.html
- std::collections: https://doc.rust-lang.org/std/collections/index.html

## Installation rapide (Codespaces/Linux)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
rustc --version
cargo --version
```

## Compiler sans Cargo (rare)

```bash
rustc main.rs
./main
```

En pratique, on utilise presque toujours Cargo.

## Cargo: commandes utiles

| Commande | A quoi ca sert |
| --- | --- |
| `cargo new mon_projet` | Cree un nouveau projet binaire |
| `cargo new mon_projet --lib` | Cree une librairie |
| `cargo init` | Initialise un projet Cargo dans un dossier existant |
| `cargo build` | Compile en debug |
| `cargo build --release` | Compile en release (optimise) |
| `cargo run` | Compile puis execute |
| `cargo check` | Verifie rapidement sans produire de binaire |
| `cargo test` | Lance les tests |
| `cargo fmt` | Formate le code |
| `cargo clippy` | Lance les lints Clippy |
| `cargo fix` | Applique certains correctifs automatiques |
| `cargo doc --open` | Genere et ouvre la doc locale |
| `cargo update` | Met a jour les versions dans `Cargo.lock` |
| `cargo clean` | Supprime `target/` |

### A retenir sur l'environnement Cargo

- `Cargo.toml` decrit le package (nom, version, edition, dependances).
- `Cargo.lock` fige les versions exactes pour des builds reproductibles.
- Le dossier `target/` contient les artefacts de compilation.
- Profil par defaut: debug (plus rapide a compiler). Profil release: plus rapide a executer.

Exemple minimal de `Cargo.toml`:

```toml
[package]
name = "mon_projet"
version = "0.1.0"
edition = "2021"

[dependencies]
rand = "0.8"
ma_lib = { path = "../ma_lib" } 
```

## Package, crate, modules

- Un **package** = projet Cargo (defini par `Cargo.toml`).
- Une **crate** = unite de compilation Rust.
- `src/main.rs` -> crate binaire.
- `src/lib.rs` -> crate librairie.
- Un package peut contenir plusieurs binaires via `src/bin/*.rs`.

Regles modules (memo):
- `mod foo;` cherche `foo.rs` ou `foo/mod.rs`.
- `pub` rend un module/item visible hors du module parent.
- `use` permet de raccourcir les chemins.
- Chemins typiques: `crate::...`, `self::...`, `super::...`.

## Points Rust importants (chapitres 1 -> 11)

### 1. Variables et mutabilite

- Par defaut, une variable est immuable: `let x = 5;`.
- `let mut x = 5;` pour autoriser la mutation.
- Le shadowing permet de redefinir un nom: `let x = x + 1;`.

### 2. Ownership et borrowing

- Chaque valeur a un proprietaire unique.
- Quand le proprietaire sort de portee, la valeur est liberee.
- Un move transfere la propriete (plus utilisable via l'ancien nom).
- References:
  - `&T` emprunt immutable (plusieurs autorises)
  - `&mut T` emprunt mutable (un seul a la fois)
- Pas de reference pendante: securite memoire garantie a la compilation.

### 3. Slices

- `&str` est un slice UTF-8, souvent pour emprunter une chaine.
- `String` est possedee et extensible.
- Exemple: `let s: &str = &mon_string[0..4];`.

### 4. Structs et enums

- `struct` regroupe des champs nommes.
- Syntaxe de mise a jour:

```rust
let user2 = User {
  email: String::from("another@example.com"),
  ..user1
};
```

Attention: `..user1` peut deplacer des champs.

- `enum` modelise des variantes exclusives (ex: `Option<T>`, `Result<T, E>`).
- `match` oblige a traiter tous les cas.
- `if let` est pratique pour un seul motif.

### 5. Collections (chapitre 8)

#### Vec<T>

- Tableau dynamique en memoire contigue.
- Operations frequentes: `push`, indexation, `get`, iteration.
- Eviter `v[i]` si risque d'index invalide; preferer `v.get(i)`.

```rust
let mut v = vec![1, 2, 3];
v.push(4);
if let Some(x) = v.get(2) {
  println!("{x}");
}
```

#### String

- `String` est UTF-8, donc pas d'indexation directe (`s[0]` interdit).
- Vues utiles: bytes (`as_bytes`), caracteres Unicode (`chars`), graphemes (crate externe).
- Concatener: `push_str`, `push`, `format!`.

```rust
let mut s = String::from("Bon");
s.push_str("jour");
s.push('!');
let msg = format!("{} {}", s, "Rust");
```

#### HashMap<K, V>

- Associe une cle a une valeur.
- Utiliser `entry(...).or_insert(...)` pour initialiser si absent.
- Pattern courant pour compter: `entry(cle).or_insert(0)` puis incrementation.
- Si tu as besoin d'ordre par cle, preferer `BTreeMap`.

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Bleu"), 10);
scores.entry(String::from("Jaune")).or_insert(50);
scores.entry(String::from("Bleu")).or_insert(50);
```

### 6. Gestion d'erreurs (chapitre 9)

- Erreurs non recuperables: `panic!` (arret du programme).
- Erreurs recuperables: `Result<T, E>`.
- `unwrap()` est pratique en prototype, mais `expect("contexte")` est plus clair.
- L'operateur `?` propage les erreurs proprement.

```rust
use std::fs::File;
use std::io::{self, Read};

fn lire_username() -> Result<String, io::Error> {
  let mut f = File::open("hello.txt")?;
  let mut s = String::new();
  f.read_to_string(&mut s)?;
  Ok(s)
}
```

### 7. Generic types, traits, lifetimes (chapitre 10)

#### Generics

- Ecrire du code reutilisable sans dupliquer la logique.
- Exemples standards: `Option<T>`, `Vec<T>`, `Result<T, E>`.

```rust
fn plus_grand<T: PartialOrd>(liste: &[T]) -> &T {
  let mut plus_grand = &liste[0];
  for item in liste {
    if item > plus_grand {
      plus_grand = item;
    }
  }
  plus_grand
}
```

#### Traits

- Un trait decrit un comportement partage.
- Tu peux definir des methodes par defaut.
- `impl Trait` simplifie certaines signatures.

```rust
trait Summary {
  fn summarize(&self) -> String;
}

fn notifier(item: &impl Summary) {
  println!("News: {}", item.summarize());
}
```

#### Lifetimes

- Les lifetimes relient la validite de references entre elles.
- Elles n'allongent pas la vie des donnees, elles la decrivent.

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() { x } else { y }
}
```

### 8. Tests (chapitre 11)

- Unit tests: dans le meme fichier/module (`mod tests`).
- Integration tests: dans `tests/`, via l'API publique de la crate.
- Macros cle: `assert!`, `assert_eq!`, `assert_ne!`.
- Cas de panic: `#[should_panic]`.
- Un test peut retourner `Result<(), E>`.

```rust
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn additionne() {
    assert_eq!(2 + 2, 4);
  }
}
```

Commandes utiles:
- `cargo test`
- `cargo test nom_test`
- `cargo test -- --show-output`
- `cargo test -- --ignored`

### 9. Outils qualite

```bash
cargo fmt
cargo clippy -- -D warnings
cargo test
```

`cargo fix` est utile, mais relire le diff avant validation.

### 10. Bonnes pratiques personnelles

- Preferer `cargo check` pendant le dev (plus rapide).
- Ajouter des types explicites quand un code devient ambigu.
- Lire les messages du compilateur jusqu'au bout (souvent tres pedagogiques).
- Garder des exemples minimalistes dans des petits crates, comme dans ce repo.
- Tester les cas limites, pas seulement le chemin nominal.

## Lexique express

- **crate**: unite de compilation Rust.
- **package**: projet Cargo (avec `Cargo.toml`).
- **module**: organisation du code a l'interieur d'une crate.
- **ownership**: regles de possession memoire sans GC.
- **borrow**: emprunt via reference.
- **TOML**: Tom's Obvious, Minimal Language.