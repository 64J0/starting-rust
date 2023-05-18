fn main() {
    let n = 10;

    println!("The square of sum: {}", square_of_sum(&n));
    println!("The sum of square: {}", sum_of_squares(&n));

    let d = difference(&n);
    println!("The difference is: {}", d);
}

fn square_of_sum(n: &u32) -> u32 {
    let limit = n + 1;
    (1..limit).sum::<u32>().pow(2)
}

fn sum_of_squares(n: &u32) -> u32 {
    let limit = n + 1;
    (1..limit).fold(0, |sum, x| sum + u32::pow(x, 2))
}

fn difference(n: &u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
