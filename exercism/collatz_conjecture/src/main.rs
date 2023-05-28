fn recursive_collatz(counter: u64, n: Option<u64>) -> Option<u64> {
    match n {
        Some(1) => Some(counter),
        Some(n) if n % 2 == 0 => {
            let new_counter = counter + 1;
            let new_n = n / 2;
            recursive_collatz(new_counter, Some(new_n))
        }
        Some(n) => {
            let new_counter = counter + 1;
            let new_n = n.checked_mul(3)?.checked_add(1);
            recursive_collatz(new_counter, new_n)
        }
        None => None,
    }
}

pub fn collatz(n: u64) -> Option<u64> {
    recursive_collatz(0, Some(n))
}

fn main() {
    println!("12: {:?}", collatz(12));
    println!("u64::MAX / 3: {:?}", collatz(u64::MAX / 3));
}
