fn main() {
    let input: u32 = 3_999_999_999;

    println!(
        "{} is an Armstrong number: {}",
        input,
        is_armstrong_number(input)
    );
}

pub fn is_armstrong_number(num: u32) -> bool {
    let num_string: String = num.to_string();
    let len = num_string.len() as u32;

    num_string
        .chars()
        .map(|c| c.to_string().parse::<u64>().unwrap())
        .map(|d| d.pow(len))
        .sum::<u64>()
        == u64::from(num)
}
