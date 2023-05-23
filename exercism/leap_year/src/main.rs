pub fn is_leap_year(year: u64) -> bool {
    let evenly_divisible_by_four = year % 4 == 0;
    let evenly_divisible_by_hundred = year % 100 == 0;
    let evenly_divisible_by_four_hundred = year % 400 == 0;

    evenly_divisible_by_four && (!evenly_divisible_by_hundred || evenly_divisible_by_four_hundred)
}

fn main() {
    println!("result: {}", is_leap_year(2000));
}
