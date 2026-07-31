# 🦀 Rust Lab

Welcome to **Rust Lab** — a hands-on learning repository and playground for mastering the [Rust Programming Language](https://www.rust-lang.org/).

---

## 🚀 Getting Started

### 1. Prerequisites & Installation

To get started with Rust, install `rustup` (the official Rust toolchain installer and version manager):

- **Windows**: Download and run [rustup-init.exe](https://rustup.rs/) (Ensure you also have the [Visual Studio C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) installed).
- **macOS / Linux**: Run in your terminal:
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

Verify your installation:
```bash
rustc --version
cargo --version
```

---

## 🛠️ Essential Cargo Commands

[Cargo](https://doc.rust-lang.org/cargo/) is Rust's official package manager and build system.

| Command | Description |
|---|---|
| `cargo new <project_name>` | Create a new binary Rust application |
| `cargo new --lib <lib_name>` | Create a new Rust library |
| `cargo check` | Quickly verify that your code compiles without building binaries |
| `cargo build` | Compile the project (Debug build in `./target/debug`) |
| `cargo build --release` | Compile with optimizations (Release build in `./target/release`) |
| `cargo run` | Compile and run the project |
| `cargo test` | Run tests across the project |
| `cargo fmt` | Format your code according to Rust conventions |
| `cargo clippy` | Run Rust linter for catch common mistakes & idioms |

---

## 🗺️ Learning Roadmap

A step-by-step checklist of core Rust concepts to explore:

- [ ] **1. Foundations**
  - [ ] Variables & Mutability (`let`, `let mut`, `const`)
  - [ ] Data Types (Scalar: `i32`, `f64`, `bool`, `char` / Compound: Tuples, Arrays)
  - [ ] Functions, Expressions vs Statements
  - [ ] Control Flow (`if/else`, `loop`, `while`, `for`)
- [ ] **2. Core Rust Concepts**
  - [ ] **Ownership Rules** (Stack vs Heap, Move semantics, Copy trait)
  - [ ] **Borrowing & References** (`&T` vs `&mut T`, Aliasing XOR Mutability)
  - [ ] **Slices** (String slices `&str`, Array slices)
- [ ] **3. Data Structures & Types**
  - [ ] Structs (Classic, Tuple Structs, Unit Structs) & `impl` blocks
  - [ ] Enums & `Option<T>` / `Result<T, E>`
  - [ ] Pattern Matching (`match`, `if let`, `while let`)
- [ ] **4. Collections & Error Handling**
  - [ ] Common Collections (`Vec<T>`, `HashMap<K, V>`, `String`)
  - [ ] Unrecoverable Errors (`panic!`) vs Recoverable Errors (`Result`, `?` operator)
- [ ] **5. Advanced Fundamentals**
  - [ ] Generics & Traits (`impl Trait`, Trait bounds, Derive macros)
  - [ ] Lifetimes (`'a`, Lifetime annotations)
  - [ ] Closures & Iterators (`map`, `filter`, `fold`)
- [ ] **6. Concurrency & Systems**
  - [ ] Threads & Message Passing (`std::sync::mpsc`)
  - [ ] Shared State (`Arc<Mutex<T>>`)
  - [ ] Smart Pointers (`Box<T>`, `Rc<T>`, `RefCell<T>`)

---

## 📂 Recommended Repository Structure

As you create experiments and sub-projects, you can organize them as a Cargo workspace or standalone sub-crates:

```text
rust-lab/
├── 01-basics/             # Basic syntax, variables, control flow
├── 02-ownership/          # Ownership, borrowing, and slices
├── 03-structs-enums/      # Custom data types & pattern matching
├── 04-error-handling/     # Result, Option, custom error types
├── 05-collections/        # Vec, HashMap, String manipulation
├── 06-traits-generics/    # Traits, generic types, trait bounds
├── projects/              # Mini-projects (CLI tools, games, servers)
└── README.md
```

---

## 📚 Recommended Resources

- 📖 **[The Rust Programming Language (The Book)](https://doc.rust-lang.org/book/)** - The official, most comprehensive guide.
- 🦀 **[Rust by Example](https://doc.rust-lang.org/rust-by-example/)** - Learn by exploring annotated code snippets.
- 🏋️ **[Rustlings](https://github.com/rust-lang/rustlings)** - Small interactive exercises to get used to reading and writing Rust.
- 📦 **[Crates.io](https://crates.io/)** - Rust community package registry.
- 📑 **[Rust Standard Library Docs](https://doc.rust-lang.org/std/)** - Official standard library documentation.
