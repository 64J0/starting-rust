use time::Duration;
use time::PrimitiveDateTime as DateTime;

fn main() {
    println!("Hello, world!");

    after(DateTime::MIN);
}

pub fn after(start: DateTime) -> DateTime {
    let gigasecond: Duration = Duration::seconds(1_000_000_000);

    start + gigasecond
}
