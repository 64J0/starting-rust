use std::cmp::Ordering;

fn main() {
    let list = vec![
        "nail", "shoe", "horse", "rider", "message", "battle", "kingdom",
    ];
    // let list = vec![];

    let proverb = build_proverb(&list);

    println!("Proverb: \n{}", proverb);
}

pub fn build_proverb(list: &[&str]) -> String {
    let len = list.len();

    if len == 0 {
        String::from("")
    } else {
        let last_index = len - 1;

        (0..=last_index)
            .map(|i| match i.cmp(&last_index) {
                Ordering::Equal => format!("And all for the want of a {}.", list[0]),
                _ => format!("For want of a {} the {} was lost.\n", list[i], list[i + 1]),
            })
            .collect::<Vec<String>>()
            .join("")
    }
}
