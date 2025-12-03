/*
 * Chapter 5: Using Structs to Structure Related Data
 * Section: Unit-Like Structs
 * 
 * Unit-like structs are structs that don't have any fields. They are
 * similar to the unit type () but are useful when you need to implement
 * traits on a type but don't have any data to store in the type itself.
 * 
 * KEY CONCEPTS:
 * - Unit-like structs have no fields
 * - They're useful for implementing traits without data
 * - They can be used as markers or for type-level programming
 * - They take up no space in memory
 */

fn main() {
    println!("=== Unit-Like Structs ===\n");

    // Example 1: Basic unit-like struct
    println!("1. Basic unit-like struct:");
    
    struct AlwaysEqual;
    
    let subject = AlwaysEqual;
    println!("   Created instance of AlwaysEqual");
    println!("   Unit-like structs have no fields to access\n");

    // Example 2: Using unit-like structs as markers
    println!("2. Using unit-like structs as markers:");
    
    struct Marker;
    struct AnotherMarker;
    
    let marker1 = Marker;
    let marker2 = AnotherMarker;
    
    println!("   marker1 and marker2 are different types");
    println!("   They can be used to distinguish between different contexts\n");

    // Example 3: Unit-like structs in generic contexts
    println!("3. Unit-like structs in generic contexts:");
    
    struct Phantom<T> {
        _phantom: std::marker::PhantomData<T>,
    }
    
    // This is a common pattern, but unit-like structs are simpler
    struct TypeMarker;
    
    let _marker = TypeMarker;
    println!("   Unit-like structs are useful for type-level programming\n");

    // Example 4: Unit-like structs implementing traits
    println!("4. Unit-like structs for trait implementation:");
    
    // Example: A struct that implements a trait but has no data
    struct Counter;
    
    impl Counter {
        fn new() -> Self {
            Counter
        }
        
        fn count(&self) -> u32 {
            0  // Simplified example
        }
    }
    
    let counter = Counter::new();
    println!("   counter.count() = {}", counter.count());
    println!("   Unit-like structs can have methods even without fields\n");

    // Example 5: Unit-like structs as type parameters
    println!("5. Unit-like structs as type parameters:");
    
    struct Event<T> {
        _data: T,
    }
    
    struct ClickEvent;
    struct KeyPressEvent;
    
    let click = Event { _data: ClickEvent };
    let keypress = Event { _data: KeyPressEvent };
    
    println!("   Created Event<ClickEvent> and Event<KeyPressEvent>");
    println!("   Different unit-like structs create different types\n");

    // Example 6: Comparison with unit type
    println!("6. Comparison with unit type ():");
    
    struct UnitStruct;
    let unit_value = ();
    let struct_value = UnitStruct;
    
    println!("   unit_value: {:?}", unit_value);
    println!("   struct_value: UnitStruct (has its own type)");
    println!("   Unit-like structs are distinct types, unlike ()");
}

