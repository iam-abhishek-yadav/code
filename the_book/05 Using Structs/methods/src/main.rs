/*
 * Chapter 5: Using Structs to Structure Related Data
 * Section: Methods
 * 
 * Methods are functions defined within the context of a struct (or enum or trait).
 * They're similar to functions, but they're associated with a particular type.
 * The first parameter of a method is always `self`, which represents the instance
 * of the struct the method is being called on.
 * 
 * KEY CONCEPTS:
 * - Methods are defined in impl blocks
 * - First parameter is always `self` (&self, &mut self, or self)
 * - Methods are called using dot notation
 * - &self: borrows immutably (read-only)
 * - &mut self: borrows mutably (can modify)
 * - self: takes ownership (consumes the instance)
 */

fn main() {
    println!("=== Methods ===\n");

    // Example 1: Basic method with &self
    println!("1. Basic method with &self (immutable borrow):");
    
    struct Rectangle {
        width: u32,
        height: u32,
    }
    
    impl Rectangle {
        // Method that borrows self immutably
        fn area(&self) -> u32 {
            self.width * self.height
        }
        
        // Another method with &self
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }
    
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    println!("   rect1: width={}, height={}", rect1.width, rect1.height);
    println!("   rect1.area() = {}", rect1.area());
    println!("   Note: rect1 is still valid after calling area()\n");

    // Example 2: Method with &mut self
    println!("2. Method with &mut self (mutable borrow):");
    
    struct Counter {
        value: u32,
    }
    
    impl Counter {
        fn new() -> Counter {
            Counter { value: 0 }
        }
        
        // Method that borrows self mutably
        fn increment(&mut self) {
            self.value += 1;
        }
        
        // Method that borrows self immutably
        fn get_value(&self) -> u32 {
            self.value
        }
    }
    
    let mut counter = Counter::new();
    println!("   Initial value: {}", counter.get_value());
    
    counter.increment();
    counter.increment();
    counter.increment();
    
    println!("   After 3 increments: {}", counter.get_value());
    println!("   Note: counter must be mutable to call increment()\n");

    // Example 3: Method with self (takes ownership)
    println!("3. Method with self (takes ownership):");
    
    struct Person {
        name: String,
        age: u32,
    }
    
    impl Person {
        fn new(name: String, age: u32) -> Person {
            Person { name, age }
        }
        
        // Method that takes ownership
        fn into_name(self) -> String {
            self.name  // Takes ownership of name
        }
        
        // Method that borrows
        fn get_age(&self) -> u32 {
            self.age
        }
    }
    
    let person = Person::new(String::from("Alice"), 30);
    println!("   person.age = {}", person.get_age());
    
    let name = person.into_name();  // person is consumed here
    println!("   Extracted name: {}", name);
    // println!("person.age: {}", person.get_age()); // ERROR: person was moved
    println!("   Note: person is no longer valid after into_name()\n");

    // Example 4: Multiple methods
    println!("4. Multiple methods on the same struct:");
    
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    
    println!("   rect1: {}x{}", rect1.width, rect1.height);
    println!("   rect2: {}x{}", rect2.width, rect2.height);
    println!("   rect3: {}x{}", rect3.width, rect3.height);
    println!("   rect1.can_hold(&rect2): {}", rect1.can_hold(&rect2));
    println!("   rect1.can_hold(&rect3): {}", rect1.can_hold(&rect3));
    println!();

    // Example 5: Methods vs Functions
    println!("5. Methods vs Functions:");
    
    // Function (not a method)
    fn area_function(rect: &Rectangle) -> u32 {
        rect.width * rect.height
    }
    
    println!("   Function call: area_function(&rect1) = {}", area_function(&rect1));
    println!("   Method call: rect1.area() = {}", rect1.area());
    println!("   Methods provide a more object-oriented interface\n");

    // Example 6: Method chaining
    println!("6. Method chaining:");
    
    struct StringBuilder {
        content: String,
    }
    
    impl StringBuilder {
        fn new() -> StringBuilder {
            StringBuilder {
                content: String::new(),
            }
        }
        
        // Returns &mut self to allow chaining
        fn append(&mut self, s: &str) -> &mut Self {
            self.content.push_str(s);
            self
        }
        
        fn build(self) -> String {
            self.content
        }
    }
    
    // Method chaining with mutable reference
    let mut builder = StringBuilder::new();
    builder.append("Hello, ").append("world!");
    let result = builder.build();
    
    println!("   Chained methods result: '{}'", result);
    println!("   Note: append() returns &mut Self for chaining");
    println!("   build() takes ownership, so it's called separately");
}

