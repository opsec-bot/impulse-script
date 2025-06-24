Sure! Here’s a Rust-friendly rewrite of your C++ copilot instructions, adapted for idiomatic Rust development and tooling like `rustc`, `cargo`, and common Rust patterns:

---

# Rust Programming Copilot Instructions

## Basic Principles

* Use English for all code, documentation, and comments.
* Explicitly declare types for variables and functions where inference isn’t clear.
* Define structs, enums, traits, and modules as needed.
* Use Rust doc comments (`///`) for public APIs.
* Avoid unnecessary blank lines within functions.
* Follow the One Definition Rule (ODR) via modules and crates.

## Naming Conventions

* PascalCase for structs, enums, traits, and types.
* snake\_case for variables, functions, methods, modules, and files.
* SCREAMING\_SNAKE\_CASE for constants (`const`) and statics (`static`).
* Use descriptive, complete words rather than abbreviations, except for well-known acronyms (e.g., HTTP, URL, ID).
* Boolean variables and functions should be prefixed with `is_`, `has_`, `can_`, etc.

## Functions

* Write small, focused functions (ideally under 20 lines).
* Name functions with verbs and nouns, e.g., `calculate_sum`, `is_ready`.
* For boolean return functions, use `is_`, `has_`, `can_` prefixes.
* Use early returns to avoid deep nesting.
* Favor iterator adapters and combinators over explicit loops where possible.
* Use closures for short logic, named functions for complex logic.
* Use default parameter values with builder patterns or structs instead of optional parameters.
* Reduce parameter count by grouping related data in structs.
* Maintain a single abstraction level per function.

## Data

* Prefer structs and enums over primitive tuples or raw types.
* Use newtypes or wrapper structs with validation for data integrity.
* Favor immutability; use `let` (immutable bindings) by default and `mut` only when necessary.
* Use `Option<T>` for nullable or optional values.

## Traits and Types

* Follow SOLID-inspired principles adapted for Rust traits and composition.
* Favor trait-based polymorphism over inheritance.
* Define small, single-purpose traits.
* Implement `Drop` carefully to manage resources.
* Make struct fields private by default; expose public accessors if needed.
* Mark functions as `const fn` when possible.

## Error Handling

* Use `Result<T, E>` for recoverable errors.
* Use `panic!` only for unrecoverable errors.
* Propagate errors using `?` operator.
* Use crates like `anyhow` or `thiserror` for rich error contexts.
* Avoid global error handling unless in binary crates’ main function.

## Memory and Resource Management

* Use Rust’s ownership model to avoid manual memory management.
* Use smart pointers like `Box<T>`, `Rc<T>`, or `Arc<T>` as needed.
* Rely on RAII and `Drop` trait for cleanup.
* Prefer standard collections (`Vec`, `HashMap`, etc.) over raw arrays or pointers.

## Testing

* Use `#[cfg(test)]` modules and the built-in test framework.
* Follow Arrange-Act-Assert pattern in tests.
* Use descriptive names for test variables and cases.
* Write unit tests for public functions and methods.
* Use mock objects or traits for dependency injection in tests.
* Write integration tests for crate modules.
* Follow Given-When-Then pattern in complex scenarios.

## Project Structure

* Organize code into idiomatic Cargo project structure (`src/`, `tests/`, `benches/`, `examples/`).
* Use modules and crates to organize code logically.
* Use `Cargo.toml` for dependencies and build configuration.
* Separate public API (lib.rs) from private implementation (mod.rs or submodules).
* Use namespaces (modules) for logical grouping.

## Standard Library and Ecosystem

* Prefer Rust standard library (`std`) types like `String`, `Vec`, `HashMap`.
* Use `Option`, `Result`, `Cow`, and other idiomatic enums for flexibility.
* Use `std::fs` for filesystem operations.
* Use `std::time` for time and duration handling.
* Use `std::sync` primitives (`Mutex`, `RwLock`, `Arc`) for concurrency.

## Concurrency and Parallelism

* Use threads with `std::thread::spawn`.
* Use `std::sync::Mutex`, `RwLock`, and `Arc` for shared mutable state.
* Prefer message passing with channels (`std::sync::mpsc`) where appropriate.
* Use crates like `tokio` or `async-std` for async concurrency.
* Avoid data races by adhering to Rust’s ownership and borrowing rules.
* Use atomics (`AtomicBool`, `AtomicUsize`) for lock-free concurrency.

---

If you want, I can also generate a Rust linter config or sample module following these principles!
