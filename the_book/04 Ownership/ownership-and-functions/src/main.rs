/*
 * Chapter 4: Understanding Ownership
 * Section: What is Ownership - Ownership and Functions
 * 
 * This section demonstrates how ownership works when passing values to functions
 * and returning values from functions.
 * 
 * KEY CONCEPTS:
 * - Passing a variable to a function moves or copies it, just as assignment does
 * - Returning values can also transfer ownership
 * - Copy types are copied, non-Copy types are moved
 */

fn main() {
    println!("=== Ownership and Functions Examples ===\n");

    // Example 1: Taking ownership (Move)
    println!("1. Function takes ownership (Move):");
    let s = String::from("hello");
    println!("   Before function call: s = '{}'", s);
    
    // Ownership of `s` is moved to the function
    takes_ownership(s);
    
    // s is no longer valid here
    // println!("s: {}", s); // ERROR: value borrowed after move
    println!("   After function call: s is no longer valid\n");

    // Example 2: Copy types are copied
    println!("2. Copy types are copied (not moved):");
    let x = 5;
    println!("   Before function call: x = {}", x);
    
    // `x` is copied into the function (not moved)
    makes_copy(x);
    
    // x is still valid because it was copied, not moved
    println!("   After function call: x = {} (still valid)\n", x);

    // Example 3: Returning ownership
    println!("3. Returning ownership from functions:");
    // Function returns a String, giving ownership to `s1`
    let s1 = gives_ownership();
    println!("   s1 = '{}' (received ownership from function)", s1);
    println!();

    // Example 4: Taking and giving back ownership
    println!("4. Taking and giving back ownership:");
    let s2 = String::from("hello");
    println!("   Before: s2 = '{}'", s2);
    
    // Ownership of s2 is moved to takes_and_gives_back
    // Ownership is then returned to s3
    let s3 = takes_and_gives_back(s2);
    
    // s2 is no longer valid
    // println!("s2: {}", s2); // ERROR: value moved
    println!("   After: s2 is no longer valid");
    println!("   s3 = '{}' (received ownership back)", s3);
    println!();

    // Example 5: Multiple parameters
    println!("5. Multiple parameters:");
    let s1 = String::from("hello");
    let s2 = String::from("world");
    println!("   Before: s1 = '{}', s2 = '{}'", s1, s2);
    
    // Both s1 and s2 are moved
    takes_two_strings(s1, s2);
    
    // Both s1 and s2 are no longer valid
    // println!("s1: {}", s1); // ERROR
    // println!("s2: {}", s2); // ERROR
    println!("   After: both s1 and s2 are no longer valid\n");

    // Example 6: Returning multiple values
    println!("6. Returning multiple values:");
    let s1 = String::from("hello");
    let (s2, len) = calculate_length_and_return(s1);
    println!("   The length of '{}' is {}.", s2, len);
    println!("   s2 still owns the string after the function call");
}

// This function takes ownership of a String
// When the function ends, `s` goes out of scope and is dropped
fn takes_ownership(s: String) {
    println!("   Inside function: s = '{}'", s);
    // s is dropped here when function ends
}

// This function receives a copy of an i32
// The original variable is unaffected because i32 implements Copy
fn makes_copy(x: i32) {
    println!("   Inside function: x = {}", x);
    // x is a copy, original is unaffected
}

// This function creates a new String and returns it
// Ownership is transferred to the caller
fn gives_ownership() -> String {
    let some_string = String::from("yours");
    println!("   Function creates: '{}'", some_string);
    some_string // Ownership is moved out of the function
}

// This function takes ownership and then returns it
// Ownership is transferred back to the caller
fn takes_and_gives_back(a_string: String) -> String {
    println!("   Function received: '{}'", a_string);
    a_string // Ownership is moved back to the caller
}

// Function that takes two String parameters
fn takes_two_strings(s1: String, s2: String) {
    println!("   Inside function: s1 = '{}', s2 = '{}'", s1, s2);
    // Both s1 and s2 are dropped here
}

// Function that takes ownership, calculates length, and returns both
fn calculate_length_and_return(s: String) -> (String, usize) {
    let length = s.len();
    (s, length) // Return both the string and its length
}