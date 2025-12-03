/*
 * Chapter 5: Using Structs to Structure Related Data
 * Section: Field Init Shorthand
 * 
 * When the variable names match the struct field names, Rust provides
 * a shorthand syntax that allows you to omit the field name and just
 * write the variable name.
 * 
 * KEY CONCEPTS:
 * - If variable name matches field name, you can use shorthand
 * - This reduces repetition and makes code cleaner
 * - Shorthand: field_name (instead of field_name: field_name)
 */

// Define struct at module level
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    println!("=== Field Init Shorthand ===\n");

    // Example 1: Without shorthand (verbose)
    println!("1. Without shorthand (verbose):");
    
    let email = String::from("user@example.com");
    let username = String::from("someuser");
    
    // Verbose way: repeating field names
    let user1 = User {
        email: email.clone(),
        username: username.clone(),
        active: true,
        sign_in_count: 1,
    };
    
    println!("   user1.email = '{}'", user1.email);
    println!("   user1.username = '{}'\n", user1.username);

    // Example 2: With shorthand (cleaner)
    println!("2. With shorthand (cleaner):");
    
    // When variable names match field names, use shorthand
    let email2 = String::from("user2@example.com");
    let username2 = String::from("someuser2");
    
    let user2 = User {
        email: email2,      // Shorthand: email instead of email: email2
        username: username2, // Shorthand: username instead of username: username2
        active: true,
        sign_in_count: 1,
    };
    
    println!("   user2.email = '{}'", user2.email);
    println!("   user2.username = '{}'\n", user2.username);

    // Example 3: Mixing shorthand and explicit
    println!("3. Mixing shorthand and explicit syntax:");
    
    let email3 = String::from("user3@example.com");
    let username3 = String::from("someuser3");
    
    let user3 = User {
        email: email3,        // Shorthand
        username: username3,  // Shorthand
        active: true,         // Explicit (no variable with this name)
        sign_in_count: 1,     // Explicit (no variable with this name)
    };
    
    println!("   user3.email = '{}'", user3.email);
    println!("   user3.username = '{}'", user3.username);
    println!("   user3.active = {}", user3.active);
    println!("   user3.sign_in_count = {}\n", user3.sign_in_count);

    // Example 4: Using shorthand in a function
    println!("4. Using shorthand in a function:");
    
    let user4 = build_user(
        String::from("user4@example.com"),
        String::from("someuser4"),
    );
    
    println!("   user4.email = '{}'", user4.email);
    println!("   user4.username = '{}'", user4.username);
    println!("   Note: The build_user function uses shorthand syntax\n");
}

// Function using field init shorthand
fn build_user(email: String, username: String) -> User {
    User {
        email,      // Shorthand for email: email
        username,   // Shorthand for username: username
        active: true,
        sign_in_count: 1,
    }
}

