fn main() {
    // Rust has three kinds of loops: loop, while and for

    let mut counter = 0;

    let result = loop {
        counter += 1;

        println!("again!");

        if counter >= 5 {
            break counter * 2;
        }
    };

    println!("The result is {result}!");

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
