fn main() {
    greet("World");
    println!("1 + 2 = {}", add(1, 2));
    println!("1 - 2 = {}", subtract(1, 2));
}

fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn subtract(a: i32, b: i32) -> i32 {
    return a - b
}