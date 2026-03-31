# RustPlayground
Learning Rust

# Command Cargo

| Commande Cargo       | Fonction                                                    |
| -------------------- | ----------------------------------------------------------- |
| `cargo new mon_proj` | Crée un nouveau projet avec structure standard              |
| `cargo build`        | Compile le projet (debug ou release)                        |
| `cargo run`          | Compile **et** exécute le projet                            |
| `cargo test`         | Lance les tests unitaires                                   |
| `cargo check`        | Vérifie la compilation **sans générer** de binaire (rapide) |
| `cargo add crate`    | Ajoute une dépendance dans `Cargo.toml`                     |
| `cargo update`       | Met à jour les dépendances                                  |
| `cargo clean`        | Supprime les fichiers compilés (`target/`)                  |

# github Codespaces setup

installation :
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
reload le terminal 
```
source $HOME/.cargo/env
rustc --version
cargo --version
```

# Development Tools

## format

file formating :
```
rustfmt "file"
```
cargo project formating :
```
cargo fmt
```

## fix

The rustfix tool can automatically fix compiler warnings that have a clear way to correct the problem that’s likely what you want.

