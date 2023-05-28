fn push_to_acc(acc: Vec<char>, c: char) -> Vec<char> {
    let mut new_acc = acc.clone();
    new_acc.push(c);
    new_acc
}

fn check_acc_last(acc: Vec<char>, c: char) -> Vec<char> {
    match acc.split_last() {
        Some((last, elements)) => {
            if last == &c {
                elements.to_vec()
            } else {
                push_to_acc(acc, c)
            }
        }
        _ => push_to_acc(acc, c),
    }
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let init_acc: Vec<char> = vec![];

    let matchers = string.chars().fold(init_acc, |acc, c| match c {
        '[' => push_to_acc(acc, c),
        '{' => push_to_acc(acc, c),
        '(' => push_to_acc(acc, c),
        ')' => check_acc_last(acc, '('),
        '}' => check_acc_last(acc, '{'),
        ']' => check_acc_last(acc, '['),
        _ => acc,
    });

    println!("matchers: {:?}", matchers);

    matchers.is_empty()
}

fn main() {
    println!("Result: {:?}", brackets_are_balanced("[["));
    println!("Result: {:?}", brackets_are_balanced("[]"));
}
