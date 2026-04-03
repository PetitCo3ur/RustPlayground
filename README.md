# RustPlayground

Notes personnelles pour apprendre Rust (book officiel), jusqu'a la fin du chapitre 8.

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

## Points Rust importants (chapitres 1 -> 8)

### 1. Variables et mutabilite

- Par defaut, une variable est immuable: `let x = 5;`.
- `let mut x = 5;` pour autoriser la mutation.
- Le *shadowing* permet de redefinir un nom: `let x = x + 1;`.

### 2. Ownership et Borrowing

- Chaque valeur a un proprietaire unique.
- Quand le proprietaire sort de portee, la valeur est liberee.
- Un move transfere la propriete.
- References:
  - `&T` emprunt immutable (plusieurs autorises)
  - `&mut T` emprunt mutable (un seul a la fois)
- Pas de reference pendante: securite memoire a la compilation.

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

Attention: `..user1` peut deplacer des champs (move).

- `enum` modelise des variantes exclusives (ex: `Option<T>`, `Result<T, E>`).
- `match` force a traiter tous les cas.
- `if let` est pratique pour un seul motif.

### 5. Collections (chapitre 8)

### Vec<T>

- Tableau dynamique en memoire contigue.
- Operations typiques: `push`, indexation, `get`, iteration.
- Eviter `v[i]` si risque d'index invalide; preferer `v.get(i)`.

```rust
let mut v = vec![1, 2, 3];
v.push(4);
if let Some(x) = v.get(2) {
    println!("{x}");
}
```

### String

- `String` est UTF-8, donc indexation directe interdite (`s[0]` impossible).
- Trois vues utiles:
  - bytes (`.as_bytes()`)
  - scalar values (`.chars()`)
  - grapheme clusters (via crate externe, ex: `unicode-segmentation`)
- Concatener: `push_str`, `push`, `format!`.

```rust
let mut s = String::from("Bon");
s.push_str("jour");
s.push('!');
let msg = format!("{} {}", s, "Rust");
```

### HashMap<K, V>

- Associe une cle a une valeur.
- Utiliser `entry(...).or_insert(...)` pour initialiser si absent.
- Ownership important: inserer peut deplacer les valeurs possedees.

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Bleu"), 10);
scores.entry(String::from("Jaune")).or_insert(50);
scores.entry(String::from("Bleu")).or_insert(50);
```

### 6. Outils qualite

```bash
cargo fmt
cargo clippy -- -D warnings
cargo test
```

`cargo fix` est utile, mais relire le diff avant de valider.

### 7. Bonnes pratiques personnelles

- Preferer `cargo check` pendant le dev (plus rapide).
- Ajouter des types explicites quand un code devient ambigu.
- Lire les messages du compilateur jusqu'au bout: ils sont souvent tres pedagogiques.
- Garder des exemples minimalistes dans des petits crates, comme dans ce repo.

## Lexique express

- **crate**: unite de compilation Rust.
- **package**: projet Cargo (avec `Cargo.toml`).
- **module**: organisation du code a l'interieur d'une crate.
- **ownership**: regles de possession memoire sans GC.
- **borrow**: emprunt via reference.
- **TOML**: Tom's Obvious, Minimal Language.