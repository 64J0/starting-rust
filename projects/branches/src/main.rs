fn main() {
    let number = 3;
    // let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // Because if is an expression, we can use it on the right side of a let
    // statement to assign the outcome to a variable
    let condition = true;
    let number: i32 = if condition { 5 } else { 6 };
    println!("The number is {}", number);
}
