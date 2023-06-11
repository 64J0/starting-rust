fn main() {
    println!("test case: {:?}", sum_of_multiples(20, &[3, 5, 0]));
}

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples: Vec<u32> = Vec::new();
    let mut val = 1;

    while val < limit {
        if factors.iter().any(|&x| x != 0 && val % x == 0) {
            multiples.push(val);
        }

        val += 1;
    }

    multiples.iter().sum()
}
