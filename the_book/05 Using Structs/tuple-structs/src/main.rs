/*
 * Chapter 5: Using Structs to Structure Related Data
 * Section: Tuple Structs
 * 
 * Tuple structs are similar to tuples, but they have a name for the type.
 * They're useful when you want to give the whole tuple a name and make
 * it a different type from other tuples, but don't want to name each field.
 * 
 * KEY CONCEPTS:
 * - Tuple structs have a name but no named fields
 * - They're useful for creating distinct types with the same data
 * - Access fields using dot notation with index (e.g., .0, .1)
 * - Each tuple struct is its own type, even if fields are the same
 */

// Define tuple structs at module level
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    println!("=== Tuple Structs ===\n");

    // Example 1: Basic tuple structs
    println!("1. Basic tuple structs:");
    
    // Even though Color and Point have the same field types,
    // they are different types
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
    println!("   black: Color({}, {}, {})", black.0, black.1, black.2);
    println!("   origin: Point({}, {}, {})", origin.0, origin.1, origin.2);
    println!("   Note: Color and Point are different types, even with same values\n");

    // Example 2: Accessing tuple struct fields
    println!("2. Accessing tuple struct fields:");
    
    let red = Color(255, 0, 0);
    let green = Color(0, 255, 0);
    let blue = Color(0, 0, 255);
    
    println!("   red: R={}, G={}, B={}", red.0, red.1, red.2);
    println!("   green: R={}, G={}, B={}", green.0, green.1, green.2);
    println!("   blue: R={}, G={}, B={}\n", blue.0, blue.1, blue.2);

    // Example 3: Different types with same structure
    println!("3. Different types with same structure:");
    
    struct Inches(i32);
    struct Centimeters(i32);
    
    let length_in_inches = Inches(10);
    let length_in_cm = Centimeters(25);
    
    // These are different types, so you can't mix them
    // let sum = length_in_inches.0 + length_in_cm.0; // Would need explicit conversion
    
    println!("   length_in_inches: {} inches", length_in_inches.0);
    println!("   length_in_cm: {} cm", length_in_cm.0);
    println!("   Note: Inches and Centimeters are different types\n");

    // Example 4: Tuple structs with different field types
    println!("4. Tuple structs with different field types:");
    
    struct Person(String, u32, bool);
    
    let person1 = Person(String::from("Alice"), 30, true);
    let person2 = Person(String::from("Bob"), 25, false);
    
    println!("   person1: name='{}', age={}, active={}", 
             person1.0, person1.1, person1.2);
    println!("   person2: name='{}', age={}, active={}\n", 
             person2.0, person2.1, person2.2);

    // Example 5: Using tuple structs in functions
    println!("5. Using tuple structs in functions:");
    
    let point1 = Point(3, 4, 5);
    let point2 = Point(1, 2, 3);
    
    let distance = calculate_distance(point1, point2);
    println!("   Distance between point1 and point2: {:.2}\n", distance);

    // Example 6: Destructuring tuple structs
    println!("6. Destructuring tuple structs:");
    
    let color = Color(128, 64, 32);
    let Color(r, g, b) = color;  // Destructure
    
    println!("   Destructured color: R={}, G={}, B={}", r, g, b);
}

// Function that works with tuple structs
fn calculate_distance(p1: Point, p2: Point) -> f64 {
    let dx = (p1.0 - p2.0) as f64;
    let dy = (p1.1 - p2.1) as f64;
    let dz = (p1.2 - p2.2) as f64;
    
    (dx * dx + dy * dy + dz * dz).sqrt()
}

