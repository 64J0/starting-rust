fn main() {
    println!("Hello, world!");
    let x = 5;
    let y = expression_function(x);
    another_function(y);
}

fn expression_function(x: i32) -> i32 {
    x + 3
}

// In function signatures, you must declare the type of each parameter.
fn another_function(x: i32) {
    println!("The value of x is: {x}.");
}
