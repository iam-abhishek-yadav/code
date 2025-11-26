/*
 * Chapter 4: Understanding Ownership
 * Section: What is Ownership - Memory and Allocation
 * 
 * This section demonstrates how Rust handles memory allocation and deallocation,
 * the difference between stack and heap, and how ownership prevents memory issues.
 * 
 * KEY CONCEPTS:
 * - Stack: Fast, fixed-size data (Copy types)
 * - Heap: Slower, dynamic-size data (String, Vec, etc.)
 * - Move: Ownership transfer (prevents double-free)
 * - Clone: Deep copy (expensive but allows multiple owners)
 */

fn main() {
    println!("=== Memory and Allocation Examples ===\n");

    // Example 1: Stack-allocated data (Copy types)
    println!("1. Stack-allocated data (Copy trait):");
    let x: i32 = 5;
    let y: i32 = x; // x is COPIED (not moved) because i32 implements Copy
    println!("   x = {}, y = {} (both valid)", x, y);
    println!("   i32 is stored on the stack, copying is cheap\n");

    // Example 2: Heap-allocated data (Move semantics)
    println!("2. Heap-allocated data (Move semantics):");
    let s1: String = String::from("hello");
    println!("   s1 = '{}' (owns heap-allocated data)", s1);
    
    // When we assign s1 to s2, ownership is MOVED
    // This is NOT a shallow copy - s1 is invalidated
    let s2: String = s1; // Move (not shallow copy)
    
    // println!("s1: {}", s1); // ERROR: value borrowed after move
    println!("   s1 moved to s2, s1 is no longer valid");
    println!("   s2 = '{}' (now owns the heap data)", s2);
    println!("   This prevents double-free errors!\n");

    // Example 3: Clone for deep copy
    println!("3. Clone for deep copy:");
    let s3: String = String::from("hello");
    let s4: String = s3.clone(); // Clone creates a deep copy
    println!("   s3 = '{}' (still valid)", s3);
    println!("   s4 = '{}' (independent copy)", s4);
    println!("   Both s3 and s4 own separate heap allocations");
    println!("   Note: clone() is expensive for large data\n");

    // Example 4: Types that implement Copy
    println!("4. Types that implement Copy trait:");
    // These types are copied automatically:
    // - All integer types (i32, u32, etc.)
    // - Boolean type (bool)
    // - Floating point types (f32, f64)
    // - Character type (char)
    // - Tuples if all elements implement Copy (e.g., (i32, i32))
    
    let a: i32 = 10;
    let b: i32 = a; // Copy
    println!("   i32: a = {}, b = {} (both valid)", a, b);
    
    let c: bool = true;
    let d: bool = c; // Copy
    println!("   bool: c = {}, d = {} (both valid)", c, d);
    
    let e: char = 'R';
    let f: char = e; // Copy
    println!("   char: e = '{}', f = '{}' (both valid)", e, f);
    
    let tuple1: (i32, i32) = (1, 2);
    let tuple2: (i32, i32) = tuple1; // Copy (all elements implement Copy)
    println!("   tuple: tuple1 = {:?}, tuple2 = {:?} (both valid)", tuple1, tuple2);
    println!();

    // Example 5: Types that DON'T implement Copy
    println!("5. Types that DON'T implement Copy (Move semantics):");
    // These types are moved:
    // - String
    // - Vec<T>
    // - HashMap<K, V>
    // - Box<T>
    // - etc.
    
    let vec1 = vec![1, 2, 3];
    let vec2 = vec1; // Move
    // println!("vec1: {:?}", vec1); // ERROR: value moved
    println!("   Vec: vec1 moved to vec2");
    println!("   vec2 = {:?}", vec2);
}
