// Don't compile since we're trying to assign a value to the imutable variable
// x.
//
// fn main() {
//     let x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }

fn main() {
    let mut x = 5; // make the x variable mutable
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
