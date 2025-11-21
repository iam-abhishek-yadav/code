## Guessing Game

A simple command-line guessing game written in Rust, based on the example from [The Rust Programming Language](https://doc.rust-lang.org/book/).

#### Imports

```rust
use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;
```

*   `colored::*`: Brings the `colored` crate's traits into scope, allowing strings to be colored (e.g., `"You win!".green()`).
*   `rand::Rng`: The `Rng` trait defines methods that random number generators implement.
*   `std::cmp::Ordering`: An enum with variants `Less`, `Greater`, and `Equal`, used for comparing values.
*   `std::io`: The standard input/output library.

#### Main Loop

The game runs inside an infinite `loop`:

```rust
loop {
    // ... game logic ...
}
```

This continues until the user guesses correctly, at which point `break` is called to exit the loop.

#### Generating a Secret Number

```rust
let secret_number = rand::thread_rng().gen_range(1, 101);
```

*   `rand::thread_rng()`: Gives us a random number generator local to the current thread of execution.
*   `gen_range(1, 101)`: Generates a number between 1 and 100 (inclusive of lower bound, exclusive of upper bound).

#### Handling User Input

```rust
let mut guess = String::new();

io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
```

*   `String::new()`: Creates a new, empty mutable string instance.
*   `read_line(&mut guess)`: Appends user input to the `guess` string. The `&mut` indicates a mutable reference.
*   `expect(...)`: Handles potential errors. If `read_line` fails, the program crashes with the specified message.

#### Type Conversion and Error Handling

```rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
```

*   Shadowing: We create a new variable `guess` (u32) that shadows the previous `guess` (String).
*   `trim()`: Removes whitespace (like the newline from pressing Enter).
*   `parse()`: Parses the string into a number.
*   `match`: Handles the `Result` returned by `parse`.
    *   `Ok(num)`: If successful, returns the number.
    *   `Err(_)`: If parsing fails (e.g., user entered text), `continue` skips the rest of the loop iteration and prompts for input again.

#### Comparison

```rust
match guess.cmp(&secret_number) {
    Ordering::Equal => {
        println!("{}", "You win!".green());
        break;
    }
    Ordering::Greater => println!("{}", "Too big!".red()),
    Ordering::Less => println!("{}", "Too small!".red()),
}
```

*   `cmp`: Compares two values and returns an `Ordering`.
*   `match`: Matches the result against the `Ordering` variants to determine the outcome.

### Prerequisites

-   Rust and Cargo installed.

### How to Run

1.  Clone the repository or navigate to the project directory.
2.  Run the game using Cargo:

    ```bash
    cargo run
    ```

### Dependencies

-   [rand](https://crates.io/crates/rand) - Random number generation.
-   [colored](https://crates.io/crates/colored) - Terminal color output.
