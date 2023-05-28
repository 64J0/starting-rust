pub fn brackets_are_balanced(string: &str) -> bool {
    let mut brackets: Vec<char> = vec![];

    for c in string.chars() {
        match c {
            '[' | '{' | '(' => brackets.push(c),
            ')' => {
                if brackets.pop() != Some('(') {
                    return false;
                }
            }
            '}' => {
                if brackets.pop() != Some('{') {
                    return false;
                }
            }
            ']' => {
                if brackets.pop() != Some('[') {
                    return false;
                }
            }
            _ => (),
        }
    }

    brackets.is_empty()
}

fn main() {
    println!("Result: {:?}", brackets_are_balanced("[["));
    println!("Result: {:?}", brackets_are_balanced("[]"));
}
