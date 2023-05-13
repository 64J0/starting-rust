fn main() {
    // create a String from a string literal using the from function
    let mut s = String::from("hello");

    takes_onwership(s);

    s.push_str(", world!"); // error

    let x = 5;
    let y = x;
    makes_copy(x);
    println!("x: {},\ny: {}", x, y);

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1: {},\ns2: {}", s1, s2);
}

fn takes_onwership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
