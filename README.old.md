# RustPlayground
Learning Rust

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

# Basic compilation

```
rustc main.rs
./main
```

# Cargo

## Command Cargo

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
| `cargo init`         | créé un environment cargo depuis un dossier                 |
| `cargo build --release` | créé la version final de l éxécutable (optimisé)         |

## Configuration (Cargo.toml)

**[package]** : section heading that indicates that the following statements are configuring a package.

**[dependencies]** : section to list any of your project’s dependencies. Packages of code are referred to as crates.

**[bin]** : 
- name : indicate the name of the binary
- path : indicate the path of the source code

## Cargo.lock

Cargo.lock keeps track of the exact versions of dependencies in your project.
Allow to avoid regression while updating versions.
`cargo update` to bypass Cargo.lock versions.

# Rust Code

## Keyword

The following is a list of keywords currently in use, with their functionality described.

- as: Perform primitive casting, disambiguate the specific trait containing an item, or rename items in use statements.
- async: Return a Future instead of blocking the current thread.
- await: Suspend execution until the result of a Future is ready.
- break: Exit a loop immediately.
- const: Define constant items or constant raw pointers.
- continue: Continue to the next loop iteration.
- crate: In a module path, refers to the crate root.
- dyn: Dynamic dispatch to a trait object.
- else: Fallback for if and if let control flow constructs.
- enum: Define an enumeration.
- extern: Link an external function or variable.
- false: Boolean false literal.
- fn: Define a function or the function pointer type.
- for: Loop over items from an iterator, implement a trait, or specify a higher ranked lifetime.
- if: Branch based on the result of a conditional expression.
- impl: Implement inherent or trait functionality.
- in: Part of for loop syntax.
- let: Bind a variable.
- loop: Loop unconditionally.
- match: Match a value to patterns.
- mod: Define a module.
- move: Make a closure take ownership of all its captures.
- mut: Denote mutability in references, raw pointers, or pattern bindings.
- pub: Denote public visibility in struct fields, impl blocks, or modules.
- ref: Bind by reference.
- return: Return from function.
- Self: A type alias for the type we are defining or implementing.
- self: Method subject or current module.
- static: Global variable or lifetime lasting the entire program execution.
- struct: Define a structure.
- super: Parent module of the current module.
- trait: Define a trait.
- true: Boolean true literal.
- type: Define a type alias or associated type.
- union: Define a union; is a keyword only when used in a union declaration.
- unsafe: Denote unsafe code, functions, traits, or implementations.
- use: Bring symbols into scope.
- where: Denote clauses that constrain a type.
- while: Loop conditionally based on the result of an expression.

## struct update syntax

The syntax .. specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance.

```rust
fn main() {
    // --snip--

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}
```

Note that the struct update syntax uses = like an assignment; it moves the data

## Control Flow with if let and let...else

```rust
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }

    // can be replace by 
        let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
```

works also with else :

```rust
fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}
```

## Packages, Crates, and Modules

If a package contains src/main.rs and src/lib.rs, it has two crates: a binary and a library, both with the same name as the package. A package can have multiple binary crates by placing files in the src/bin directory: Each file will be a separate binary crate.

### Modules Cheat Sheet

- **Start from the crate root:** When compiling a crate, the compiler first looks in the crate root file (usually src/lib.rs for a library crate and src/main.rs for a binary crate) for code to compile.
- **Declaring modules:** In the crate root file, you can declare new modules; say you declare a “garden” module with mod garden;. The compiler will look for the module’s code in these places:
Inline, within curly brackets that replace the semicolon following mod garden
In the file src/garden.rs
In the file src/garden/mod.rs
- **Declaring submodules:** In any file other than the crate root, you can declare submodules. For example, you might declare mod vegetables; in src/garden.rs. The compiler will look for the submodule’s code within the directory named for the parent module in these places:
Inline, directly following mod vegetables, within curly brackets instead of the semicolon
In the file src/garden/vegetables.rs
In the file src/garden/vegetables/mod.rs
- **Paths to code in modules:** Once a module is part of your crate, you can refer to code in that module from anywhere else in that same crate, as long as the privacy rules allow, using the path to the code. For example, an Asparagus type in the garden vegetables module would be found at crate::garden::vegetables::Asparagus.
- **Private vs. public:** Code within a module is private from its parent modules by default. To make a module public, declare it with pub mod instead of mod. To make items within a public module public as well, use pub before their declarations.
- **The use keyword:** Within a scope, the use keyword creates shortcuts to items to reduce repetition of long paths. In any scope that can refer to crate::garden::vegetables::Asparagus, you can create a shortcut with use crate::garden::vegetables::Asparagus;, and from then on you only need to write Asparagus to make use of that type in the scope.

## Collections

*documentation* -> https://doc.rust-lang.org/std/collections/index.html

Rust’s collections can be grouped into four major categories:

- **Sequences:** Vec, VecDeque, LinkedList
- **Maps:** HashMap, BTreeMap
- **Sets:** HashSet, BTreeSet
- **Misc:** BinaryHeap

### What type of collection should be use?
These are fairly high-level and quick break-downs of when each collection should be considered. Detailed discussions of strengths and weaknesses of individual collections can be found on their own documentation pages.

Use a Vec when:
- You want to collect items up to be processed or sent elsewhere later, and don’t care about any properties of the actual values being stored.
- You want a sequence of elements in a particular order, and will only be appending to (or near) the end.
- You want a stack.
- You want a resizable array.
- You want a heap-allocated array.

Use a VecDeque when:
- You want a Vec that supports efficient insertion at both ends of the sequence.
- You want a queue.
- You want a double-ended queue (deque).

Use a LinkedList when:
- You want a Vec or VecDeque of unknown size, and can’t tolerate amortization.
- You want to efficiently split and append lists.
- You are absolutely certain you really, truly, want a doubly linked list.

Use a HashMap when:
- You want to associate arbitrary keys with an arbitrary value.
- You want a cache.
- You want a map, with no extra functionality.

Use a BTreeMap when:
- You want a map sorted by its keys.
- You want to be able to get a range of entries on-demand.
- You’re interested in what the smallest or largest key-value pair is.
- You want to find the largest or smallest key that is smaller or larger than something.

Use the Set variant of any of these Maps when:
- You just want to remember which keys you’ve seen.
- There is no meaningful value to associate with your keys.
- You just want a set.

Use a BinaryHeap when:
- You want to store a bunch of elements, but only ever want to process the “biggest” or “most important” one at any given time.
- You want a priority queue.

## The ? Operator Shortcut

The ? placed after a Result value is defined to work in almost the same way as the match expressions that we defined to handle the Result values

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
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

changes need to be commited before using this tool. Can be bypassed using `--allow-dirty`

```
cargo fix
```

## Quality / lints

The Clippy tool is a collection of lints to analyze your code so that you can catch common mistakes and improve your Rust code.

```
cargo clippy
```

# Bonus

**TOML** : Tom’s Obvious, Minimal Language