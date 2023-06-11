fn main() {
    // let n = get_next_prime(3, &vec![2]);
    // println!("n = {}", n);

    println!("Factors of {}: {:?}", 60, factors(60));
    println!("Factors of {}: {:?}", 1, factors(1));
    println!("Factors of {}: {:?}", 2, factors(2));
    println!("Factors of {}: {:?}", 9, factors(9));
    println!("Factors of {}: {:?}", 901_255, factors(901_255));
    println!(
        "Factors of {}: {:?}",
        93_819_012_551 as u64,
        factors(93_819_012_551)
    );
}

pub fn factors(n: u64) -> Vec<u64> {
    let mut divisors = Vec::new() as Vec<u64>;
    let mut candidates = 2..;
    let mut val = n;

    while val > 1 {
        let x = candidates.next().unwrap();

        while val % x == 0 {
            divisors.push(x);
            val /= x;
        }
    }

    divisors
}
