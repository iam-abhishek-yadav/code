fn main() {
    // Integers
    let a = 98_222;       // Decimal
    let b = 0xff;         // Hexadecimal
    let c = 0o77;         // Octal
    let d = 0b1111_0000;  // Binary
    let e = b'A';         // Byte (u8 only)
    let f: u8 = 255; 

    println!("a: {}", a);
    println!("b: {}", b);
    println!("c: {}", c);
    println!("d: {}", d);
    println!("e: {}", e);
    println!("f: {}", f);

    // Floating point numbers
    let g = 2.0;
    let h = 3.0;

    // addition 
    let result = 5 + 10;
    println!("result: {}", result);

    // subtraction
    let result = 95.5 - 4.3;
    println!("result: {}", result);

    // multiplication
    let result = 4 * 30;
    println!("result: {}", result);

    // division
    let result = 56.7 / 32.2;
    println!("result: {}", result);

    // remainder
    let result = 43 % 5;
    println!("result: {}", result);

    // Booleans
    let i = true;
    let j = false;

    println!("i: {}", i);
    println!("j: {}", j);

    // Character
    let k = 'a';
    println!("k: {}", k);

    let l = "A";
    println!("l: {}", l);

    // Compound Types
    // Tuple
    let tup = ("Hello World", 500, 6.4);
    let (x, y, z) = tup;

    println!("x: {}", x);
    println!("y: {}", y);
    println!("z: {}", z);

    let m = tup.0;
    println!("m: {}", m);

    let n = tup.1;
    println!("n: {}", n);

    let o = tup.2;
    println!("o: {}", o);

    // Array
    let arr = [1, 2, 3, 4, 5];
    println!("arr: {:?}", arr);

    let arr = [3; 5];
    println!("arr: {:?}", arr);
}
