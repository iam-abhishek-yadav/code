fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const COUNT: u32 = 100_000;
    println!("The value of COUNT is: {}", COUNT);

    let y: i32 = 5;
    println!("The value of y is: {}", y);
    let y = 6;
    println!("The value of y is: {}", y);
    let y = "Hello world!";
    println!("The value of y is: {}", y);
}
