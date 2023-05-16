fn main() {
    let n = 10;

    println!("The square of sum: {}", square_of_sum(&n));
    println!("The sum of square: {}", sum_of_square(&n));

    let d = difference(&n);
    println!("The difference is: {}", d);
}

fn square_of_sum(n: &u32) -> u32 {
    let mut result = 0;

    for i in 1..(*n + 1) {
        result += i;
    }

    u32::pow(result, 2)
}

fn sum_of_squares(n: &u32) -> u32 {
    let mut result = 0;

    for i in 1..(*n + 1) {
        result += u32::pow(i, 2);
    }

    result
}

fn difference(n: &u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
