fn main() {
    println!("{} % {} == 0? {}", 10, 2, 10 % 2 == 0);
    println!("{} % {} == 0? {}", 10, 3, 10 % 3 == 0);
    println!("0th prime: {}", nth(0));
    println!("6th prime: {}", nth(6));
    println!("7th prime: {}", nth(7));
    println!("10_000th prime: {}", nth(10_000));
}

// DOES NOT WORK DUE TO STACK OVERFLOW!
// It's something related to recursive functions apparently.
//
// pub fn recursive_get_nth_prime(n: u32, mut prime_nums: Vec<u32>, val: u32) -> u32 {
//     let has_dividend = prime_nums.iter().find(|&&x| val % x == 0);

//     match has_dividend {
//         Option::Some(_) => (),
//         Option::None => prime_nums.push(val),
//     }

//     let len = prime_nums.len() as u32;

//     if len != n {
//         // small optimization since we only need to check odd numbers
//         let next_val = val + 2;
//         recursive_get_nth_prime(n, prime_nums, next_val)
//     } else {
//         val
//     }
// }

pub fn nth(n: u32) -> u32 {
    let vec_size = n as usize;
    let mut prime_nums = vec![2];
    let mut val = 2;

    while prime_nums.len() <= vec_size {
        val += 1;

        let has_dividend = prime_nums.iter().any(|x| val % x == 0);

        match has_dividend {
            true => (),
            false => prime_nums.push(val),
        }
    }

    val
}
