/*
 * Chapter 5: Using Structs to Structure Related Data
 * Section: Struct Update Syntax
 * 
 * The struct update syntax allows you to create a new instance of a struct
 * that includes most of the values from another instance, but changes some.
 * This is done using the .. syntax.
 * 
 * KEY CONCEPTS:
 * - Use ..instance to copy remaining fields from another instance
 * - Only specify the fields you want to change
 * - The .. syntax must come last
 * - This creates a new instance (doesn't modify the original)
 */

// Define structs at module level
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    println!("=== Struct Update Syntax ===\n");

    // Example 1: Basic struct update syntax
    println!("1. Basic struct update syntax:");
    
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    
    println!("   user1:");
    println!("   - email: {}", user1.email);
    println!("   - username: {}", user1.username);
    println!("   - active: {}", user1.active);
    println!("   - sign_in_count: {}\n", user1.sign_in_count);

    // Create user2 using struct update syntax
    // Only change email, copy everything else from user1
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1  // Copy remaining fields from user1
    };
    
    println!("   user2 (created with update syntax):");
    println!("   - email: {} (changed)", user2.email);
    println!("   - username: {} (copied from user1)", user2.username);
    println!("   - active: {} (copied from user1)", user2.active);
    println!("   - sign_in_count: {} (copied from user1)\n", user2.sign_in_count);

    // Note: user1 can no longer be used because username and email were moved
    // println!("user1.username: {}", user1.username); // ERROR: value moved

    // Example 2: Updating multiple fields
    println!("2. Updating multiple fields:");
    
    let user3 = User {
        email: String::from("user3@example.com"),
        username: String::from("user3"),
        active: true,
        sign_in_count: 1,
    };
    
    let user4 = User {
        email: String::from("user4@example.com"),
        sign_in_count: 5,  // Change sign_in_count
        ..user3            // Copy username and active from user3
    };
    
    println!("   user4:");
    println!("   - email: {} (changed)", user4.email);
    println!("   - username: {} (copied from user3)", user4.username);
    println!("   - active: {} (copied from user3)", user4.active);
    println!("   - sign_in_count: {} (changed)\n", user4.sign_in_count);

    // Example 3: Using with Copy types (no move)
    println!("3. Struct update with Copy types:");
    
    let point1 = Point { x: 0, y: 0, z: 0 };
    
    let point2 = Point {
        x: 5,
        ..point1  // Copy x, y, z (all are Copy types)
    };
    
    println!("   point1: x={}, y={}, z={}", point1.x, point1.y, point1.z);
    println!("   point2: x={}, y={}, z={}", point2.x, point2.y, point2.z);
    println!("   point1 is still valid because i32 implements Copy\n");

    // Example 4: Update syntax in functions
    println!("4. Using update syntax in functions:");
    
    let base_user = User {
        email: String::from("base@example.com"),
        username: String::from("baseuser"),
        active: true,
        sign_in_count: 0,
    };
    
    let new_user = create_user_from_template(
        String::from("new@example.com"),
        String::from("newuser"),
        base_user,
    );
    
    println!("   new_user:");
    println!("   - email: {}", new_user.email);
    println!("   - username: {}", new_user.username);
    println!("   - active: {}", new_user.active);
    println!("   - sign_in_count: {}\n", new_user.sign_in_count);
}

// Function that uses struct update syntax
fn create_user_from_template(
    email: String,
    username: String,
    template: User,
) -> User {
    User {
        email,
        username,
        ..template  // Copy active and sign_in_count from template
    }
}

