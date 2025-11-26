/*
 * Chapter 4: Understanding Ownership
 * Section: What is Ownership - Ownership Rules
 * 
 * Ownership is Rust's most unique feature. It enables memory safety without
 * a garbage collector. Understanding ownership is crucial for writing Rust code.
 * 
 * THE THREE RULES OF OWNERSHIP:
 * 1. Each value in Rust has a variable that is its owner.
 * 2. There can only be one owner at a time.
 * 3. When the owner goes out of scope, the value will be dropped.
 */

fn main() {
    println!("=== Ownership Rules Examples ===\n");

    // Example 1: Variable Scope and Ownership
    println!("1. Variable Scope and Ownership:");
    {
        // s is valid from this point forward
        let s = String::from("hello");
        println!("   s = '{}' (valid in this scope)", s);
        // do stuff with s
    } // this scope is now over, and s is no longer valid
      // Rust automatically calls drop() here to free memory
    println!("   s is no longer valid here\n");

    // Example 2: String vs String Literal
    println!("2. String Type vs String Literal:");
    // String literals are immutable and stored in the binary
    let s_literal = "hello"; // &str type, stored in binary
    println!("   String literal: '{}' (immutable, in binary)", s_literal);
    
    // String type is mutable and heap-allocated
    let mut s = String::from("hello"); // String type, heap-allocated
    s.push_str(", world!"); // Can modify because it's mutable
    println!("   String type: '{}' (mutable, heap-allocated)", s);
    println!("   When s goes out of scope, memory is automatically freed\n");

    // Example 3: Multiple Owners (This violates rule 2)
    println!("3. Only One Owner at a Time:");
    let s1 = String::from("hello");
    let s2 = s1; // Ownership moved from s1 to s2
    // println!("s1: {}", s1); // ERROR: s1 is no longer valid!
    println!("   s1 moved to s2, s1 is no longer valid");
    println!("   s2 = '{}' (now owns the string)", s2);
    println!("   This prevents double-free errors\n");

    // Example 4: Copy vs Move
    println!("4. Copy Trait vs Move:");
    // Types that implement Copy are copied, not moved
    let x = 5;
    let y = x; // x is copied (because i32 implements Copy)
    println!("   x = {}, y = {} (both valid - Copy trait)", x, y);
    
    // Types that don't implement Copy are moved
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2
    // println!("s1: {}", s1); // ERROR: value moved
    println!("   s1 moved to s2 (String doesn't implement Copy)");
    println!("   s2 = '{}'", s2);
}
