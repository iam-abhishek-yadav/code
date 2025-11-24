fn main() {
    let number = 5;

    if number < 10 {
        println!("first condition is true");
    } else if number < 20 {
        println!("second condition is true");
    } else {
        println!("third condition is true");
    }

    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("number is {number}");

    loop {
        println!("again!");
        break;
    }

    let mut count = 0;
    loop {
        count += 1;
        println!("count = {count}");
        if count == 10 {
            break;
        }
    }

    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }
    for element in a.iter() {
        println!("the value is: {element}");
    }
    for number in 1..4 {
        println!("the value is: {number}");
    }
}
