/*
 * Chapter 4: Understanding Ownership
 * Section: References and Borrowing
 * 
 * This section demonstrates how to use references to allow functions to use
 * values without taking ownership of them.
 * 
 * KEY CONCEPTS:
 * - References allow you to refer to a value without taking ownership
 * - References are immutable by default
 * - Mutable references allow modification but have restrictions
 * - You can have either one mutable reference OR any number of immutable references
 * - References must always be valid (no dangling references)
 * 
 
 * THE RULES OF REFERENCES:
 * 1. At any given time, you can have either one mutable reference or any number
 *    of immutable references.
 * 2. References must always be valid.
 */

fn main() {
    println!("=== References and Borrowing Examples ===\n");

    // Example 1: Immutable references
    println!("1. Immutable references:");
    let s1 = String::from("hello");
    println!("   s1 = '{}'", s1);
    
    // Pass a reference to s1 (does not move ownership)
    let len = calculate_length(&s1);
    
    // s1 is still valid because only a reference was passed
    println!("   The length of '{}' is {}.", s1, len);
    println!("   s1 is still valid after the function call\n");

    // Example 2: Mutable references
    println!("2. Mutable references:");
    let mut s2 = String::from("hello");
    println!("   Before: s2 = '{}'", s2);
    
    // Pass a mutable reference to modify the string
    change(&mut s2);
    
    println!("   After: s2 = '{}'", s2);
    println!("   s2 is still valid and was modified\n");

    // Example 3: Only one mutable reference at a time
    println!("3. Only one mutable reference at a time:");
    let mut s = String::from("hello");
    
    let r1 = &mut s;
    // let r2 = &mut s; // ERROR: cannot borrow `s` as mutable more than once
    println!("   r1 = '{}'", r1);
    
    // r1 goes out of scope here, so we can create another mutable reference
    let r2 = &mut s;
    println!("   r2 = '{}' (created after r1 went out of scope)", r2);
    println!();

    // Example 4: Mixing mutable and immutable references
    println!("4. Cannot mix mutable and immutable references:");
    let mut s = String::from("hello");
    
    let r1 = &s; // Immutable reference
    let r2 = &s; // Another immutable reference - OK!
    println!("   r1 = '{}', r2 = '{}' (both immutable references)", r1, r2);
    
    // let r3 = &mut s; // ERROR: cannot borrow as mutable while immutable references exist
    // println!("{}", r1); // r1 and r2 are still in scope
    
    // r1 and r2 go out of scope here, so we can create a mutable reference
    let r3 = &mut s;
    println!("   r3 = '{}' (mutable reference after immutable ones dropped)", r3);
    println!();

    // Example 5: Dangling references (prevented by compiler)
    println!("5. Dangling references are prevented:");
    // This would be a dangling reference (commented out because it won't compile):
    // fn dangle() -> &String {
    //     let s = String::from("hello");
    //     &s // ERROR: returns a reference to data owned by the current function
    // }
    
    // Instead, return the value directly (transfer ownership):
    let s = no_dangle();
    println!("   s = '{}' (returned value, not reference)", s);
    println!();

    // Example 6: String slices as function parameters
    println!("6. Using string slices (&str) instead of &String:");
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("   First word of '{}' is '{}'", s, word);
    
    // String slices work with both String and &str
    let my_string_literal = "hello world";
    let word2 = first_word(my_string_literal);
    println!("   First word of '{}' is '{}'", my_string_literal, word2);
    println!("   Using &str is more flexible than &String\n");

    // Example 7: Multiple immutable references are fine
    println!("7. Multiple immutable references:");
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    let r3 = &s;
    println!("   r1 = '{}', r2 = '{}', r3 = '{}'", r1, r2, r3);
    println!("   All three immutable references are valid simultaneously");
}

// This function borrows a reference to a String (immutable)
// The reference does not take ownership, so the original value remains valid
fn calculate_length(s: &String) -> usize {
    s.len() // s goes out of scope, but because it's a reference, nothing is dropped
}

// This function borrows a mutable reference to a String
// It can modify the string without taking ownership
fn change(s: &mut String) {
    s.push_str(", world"); // We can modify through a mutable reference
}

// This function would create a dangling reference (won't compile)
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s // ERROR: s goes out of scope, but we're trying to return a reference to it
// }

// Instead, return the value directly (transfer ownership)
fn no_dangle() -> String {
    let s = String::from("hello");
    s // Ownership is moved out
}

// Better function signature: use &str instead of &String
// This allows the function to accept both String and string literals
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}