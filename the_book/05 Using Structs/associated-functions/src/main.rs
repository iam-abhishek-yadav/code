/*
 * Chapter 5: Using Structs to Structure Related Data
 * Section: Associated Functions
 * 
 * Associated functions are functions that are associated with a type but
 * don't take `self` as a parameter. They're often used as constructors
 * or utility functions related to the type.
 * 
 * KEY CONCEPTS:
 * - Associated functions don't take `self` as a parameter
 * - They're called using the `::` syntax (e.g., String::from)
 * - They're often used as constructors (e.g., Rectangle::new)
 * - They can be called without an instance of the struct
 * - Methods are a special case of associated functions (those with `self`)
 */

fn main() {
    println!("=== Associated Functions ===\n");

    // Example 1: Basic associated function (constructor)
    println!("1. Basic associated function (constructor):");
    
    struct Rectangle {
        width: u32,
        height: u32,
    }
    
    impl Rectangle {
        // Associated function (constructor)
        fn new(width: u32, height: u32) -> Rectangle {
            Rectangle { width, height }
        }
        
        // Method (takes &self)
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }
    
    // Call associated function using ::
    let rect1 = Rectangle::new(30, 50);
    println!("   Created rect1 using Rectangle::new(30, 50)");
    println!("   rect1.area() = {}\n", rect1.area());

    // Example 2: Multiple associated functions
    println!("2. Multiple associated functions:");
    
    struct Point {
        x: f64,
        y: f64,
    }
    
    impl Point {
        // Constructor
        fn new(x: f64, y: f64) -> Point {
            Point { x, y }
        }
        
        // Another associated function (factory method)
        fn origin() -> Point {
            Point { x: 0.0, y: 0.0 }
        }
        
        // Another factory method
        fn from_polar(radius: f64, angle: f64) -> Point {
            Point {
                x: radius * angle.cos(),
                y: radius * angle.sin(),
            }
        }
        
        // Method
        fn distance_from_origin(&self) -> f64 {
            (self.x * self.x + self.y * self.y).sqrt()
        }
    }
    
    let point1 = Point::new(3.0, 4.0);
    let point2 = Point::origin();
    let point3 = Point::from_polar(5.0, std::f64::consts::PI / 4.0);
    
    println!("   point1: ({}, {})", point1.x, point1.y);
    println!("   point2 (origin): ({}, {})", point2.x, point2.y);
    println!("   point3 (from_polar): ({:.2}, {:.2})", point3.x, point3.y);
    println!("   point1.distance_from_origin() = {:.2}\n", point1.distance_from_origin());

    // Example 3: Common pattern: new() constructor
    println!("3. Common pattern: new() constructor:");
    
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }
    
    impl User {
        // Associated function used as constructor
        fn new(username: String, email: String) -> User {
            User {
                username,
                email,
                active: true,
                sign_in_count: 1,
            }
        }
        
        // Method to activate user
        fn activate(&mut self) {
            self.active = true;
        }
        
        // Method to get username
        fn get_username(&self) -> &str {
            &self.username
        }
    }
    
    let mut user1 = User::new(
        String::from("alice"),
        String::from("alice@example.com"),
    );
    
    println!("   Created user1 with User::new()");
    println!("   username: {}", user1.get_username());
    println!("   active: {}", user1.active);
    println!("   sign_in_count: {}\n", user1.sign_in_count);

    // Example 4: Associated functions vs methods
    println!("4. Associated functions vs methods:");
    
    println!("   Associated function: Rectangle::new(10, 20)");
    let rect2 = Rectangle::new(10, 20);
    
    println!("   Method: rect2.area() = {}", rect2.area());
    println!("   Note: Associated functions use ::, methods use .\n");

    // Example 5: Associated functions for utility operations
    println!("5. Associated functions for utility operations:");
    
    struct MathUtils;
    
    impl MathUtils {
        // Associated function (no self, no instance needed)
        fn square(x: i32) -> i32 {
            x * x
        }
        
        fn cube(x: i32) -> i32 {
            x * x * x
        }
    }
    
    println!("   MathUtils::square(5) = {}", MathUtils::square(5));
    println!("   MathUtils::cube(3) = {}", MathUtils::cube(3));
    println!("   No instance needed - just use the type name\n");

    // Example 6: Real-world example: String::from
    println!("6. Real-world example: String::from is an associated function");
    
    // String::from is an associated function, not a method
    let s1 = String::from("hello");
    let s2 = String::from("world");
    
    println!("   s1 = '{}' (created with String::from)", s1);
    println!("   s2 = '{}' (created with String::from)", s2);
    println!("   String::from is an associated function of String");
}

