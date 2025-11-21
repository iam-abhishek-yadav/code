## Getting Started with Rust

### Installation

*   Install `rust-analyzer` (for IDE support).
*   Install Visual Studio with `Desktop development with C++` enabled (required for some Rust crates on Windows).
*   Install `cargo` (Rust's package manager and build system).

### Project Setup

1.  Create a new Rust project:
    ```bash
    cargo new hello-cargo
    ```
    This command will create a new directory named `hello-cargo` with a basic Rust project structure.

### Basic Commands

*   **Build the project:**
    ```bash
    cargo build
    ```
*   **Run the project:**
    ```bash
    cargo run
    ```
*   **Check for errors (without building):**
    ```bash
    cargo check
    ```
*   **View other available commands:**
    ```bash
    cargo --help
    ```
*   **Build an executable directly (without Cargo):**
    ```bash
    rustc main.rs
    ```
*   **Run the compiled executable:**
    ```bash
    ./main.exe # On Windows
    ./main     # On Linux/macOS
    ```