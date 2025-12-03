/*
 * Chapter 5: Using Structs to Structure Related Data
 * Section: Defining and Instantiating Structs
 * 
 * Structs are custom data types that let you group related values together.
 * They are similar to tuples, but each field has a name, making the code
 * more readable and easier to work with.
 * 
 * KEY CONCEPTS:
 * - Structs group related data together
 * - Each field has a name and a type
 * - Fields can be accessed using dot notation
 * - Structs can be mutable or immutable
 */

// Define structs at module level so they can be used by functions
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    println!("=== Defining and Instantiating Structs ===\n");

    // Example 1: Basic struct definition and instantiation
    println!("1. Basic struct definition:");
    
    // Create an instance of the struct
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    
    println!("   User struct created:");
    println!("   - username: {}", user1.username);
    println!("   - email: {}", user1.email);
    println!("   - active: {}", user1.active);
    println!("   - sign_in_count: {}\n", user1.sign_in_count);

    // Example 2: Accessing struct fields
    println!("2. Accessing struct fields:");
    println!("   user1.username = '{}'", user1.username);
    println!("   user1.email = '{}'", user1.email);
    println!("   user1.active = {}", user1.active);
    println!("   user1.sign_in_count = {}\n", user1.sign_in_count);

    // Example 3: Mutable structs
    println!("3. Mutable structs:");
    let mut user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotheruser"),
        active: false,
        sign_in_count: 0,
    };
    
    println!("   Before modification:");
    println!("   - active: {}", user2.active);
    println!("   - sign_in_count: {}", user2.sign_in_count);
    
    // Modify fields of a mutable struct
    user2.active = true;
    user2.sign_in_count = 1;
    
    println!("   After modification:");
    println!("   - active: {}", user2.active);
    println!("   - sign_in_count: {}\n", user2.sign_in_count);

    // Example 4: Structs with different field types
    println!("4. Structs with different field types:");
    
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    println!("   Rectangle:");
    println!("   - width: {} pixels", rect1.width);
    println!("   - height: {} pixels", rect1.height);
    println!("   - area: {} square pixels\n", rect1.width * rect1.height);

    // Example 5: Using structs in functions
    println!("5. Using structs in functions:");
    let user3 = build_user(
        String::from("user3@example.com"),
        String::from("user3"),
    );
    
    println!("   Built user:");
    println!("   - username: {}", user3.username);
    println!("   - email: {}", user3.email);
    println!("   - active: {}", user3.active);
    println!("   - sign_in_count: {}\n", user3.sign_in_count);
}

// Function that returns a User struct
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

