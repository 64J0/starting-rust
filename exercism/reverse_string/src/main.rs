fn main() {
    let reversed_string = reverse("Hello world!");

    println!("reversed_string: {}", reversed_string);
}

pub fn reverse(input: &str) -> String {
    reverse_iterative(input)
}

fn reverse_iterative(input: &str) -> String {
    let mut result = String::from("");

    for c in input.chars().rev() {
        println!("c: {}", c);
        result.push(c);
    }

    println!("result: {}", result);
    result
}
