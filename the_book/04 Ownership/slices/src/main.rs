/*
 * Chapter 4: Understanding Ownership
 * Section: The Slice Type
 * 
 * Slices let you reference a contiguous sequence of elements in a collection
 * rather than the whole collection. A slice is a kind of reference, so it does
 * not have ownership.
 * 
 * KEY CONCEPTS:
 * - String slices (&str) reference a portion of a String
 * - Array slices (&[T]) reference a portion of an array or vector
 * - Slices are references, so they don't take ownership
 * - String literals are string slices
 * - Slices store a pointer to the start and a length
 */

fn main() {
    println!("=== The Slice Type Examples ===\n");

    // Example 1: String slices
    println!("1. String slices (&str):");
    let s = String::from("hello world");
    
    // String slice from index 0 to 5 (exclusive)
    let hello = &s[0..5];
    println!("   s = '{}'", s);
    println!("   &s[0..5] = '{}'", hello);
    
    // String slice from index 6 to 11 (exclusive)
    let world = &s[6..11];
    println!("   &s[6..11] = '{}'", world);
    
    // String slice from start to index 5
    let hello2 = &s[..5];
    println!("   &s[..5] = '{}' (same as [0..5])", hello2);
    
    // String slice from index 6 to end
    let world2 = &s[6..];
    println!("   &s[6..] = '{}' (from index 6 to end)", world2);
    
    // String slice of entire string
    let whole = &s[..];
    println!("   &s[..] = '{}' (entire string)", whole);
    println!();

    // Example 2: String literals are string slices
    println!("2. String literals are string slices:");
    let s: &str = "Hello, world!"; // s is of type &str
    println!("   s = '{}' (type: &str)", s);
    println!("   String literals are stored in the binary and are immutable");
    println!();

    // Example 3: Using slices as function parameters
    println!("3. Using slices as function parameters:");
    let my_string = String::from("hello world");
    let my_string_literal = "hello world";
    
    // first_word works with both String and &str
    let word1 = first_word(&my_string);
    let word2 = first_word(my_string_literal);
    
    println!("   First word of '{}' is '{}'", my_string, word1);
    println!("   First word of '{}' is '{}'", my_string_literal, word2);
    println!("   Using &str is more flexible than &String\n");

    // Example 4: Array slices
    println!("4. Array slices:");
    let a = [1, 2, 3, 4, 5];
    
    // Slice of array from index 1 to 3 (exclusive)
    let slice = &a[1..3];
    println!("   Array: {:?}", a);
    println!("   Slice &a[1..3] = {:?}", slice);
    println!("   Slice type: &[i32]");
    println!();

    // Example 5: Vector slices
    println!("5. Vector slices:");
    let v = vec![1, 2, 3, 4, 5];
    
    // Slice of vector
    let slice = &v[1..3];
    println!("   Vector: {:?}", v);
    println!("   Slice &v[1..3] = {:?}", slice);
    println!();

    // Example 6: Slices prevent dangling references
    println!("6. Slices prevent dangling references:");
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("   s = '{}'", s);
    println!("   word = '{}'", word);
    println!("   word is a slice that references part of s");
    println!("   s must remain valid for word to be valid");
    println!();

    // Example 7: String slice ranges
    println!("7. String slice range syntax:");
    let s = String::from("rust programming");
    println!("   s = '{}'", s);
    
    // Different ways to create slices
    println!("   &s[0..4] = '{}'", &s[0..4]);
    println!("   &s[..4] = '{}' (start to 4)", &s[..4]);
    println!("   &s[5..] = '{}' (5 to end)", &s[5..]);
    println!("   &s[..] = '{}' (entire string)", &s[..]);
    println!();

    // Example 8: Slices and ownership
    println!("8. Slices don't take ownership:");
    let s = String::from("hello world");
    let word = first_word(&s); // word is a slice, doesn't own the data
    println!("   s = '{}' (still owns the string)", s);
    println!("   word = '{}' (just a reference)", word);
    println!("   s can still be used because word doesn't take ownership");
    println!();

    // Example 9: Finding multiple words
    println!("9. Finding multiple words:");
    let s = String::from("hello world rust");
    let first = first_word(&s);
    let second = second_word(&s);
    println!("   s = '{}'", s);
    println!("   First word: '{}'", first);
    println!("   Second word: '{}'", second);
}

// Function that takes a string slice and returns a string slice
// Using &str instead of &String makes this function more flexible
// It can accept both String and string literals
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    // Iterate through the bytes
    for (i, &item) in bytes.iter().enumerate() {
        // If we find a space, return the slice up to that point
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    // If no space is found, return the entire string
    &s[..]
}

// Example of a function that finds the second word
fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut start = 0;
    
    // Find the first space
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            start = i + 1;
            break;
        }
    }
    
    // Find the second space (or end of string)
    for (i, &item) in bytes.iter().enumerate().skip(start) {
        if item == b' ' {
            return &s[start..i];
        }
    }
    
    // Return from start to end if no second space found
    &s[start..]
}
