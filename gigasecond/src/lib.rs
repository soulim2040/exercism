use time::{PrimitiveDateTime, Duration};

pub fn after(start : PrimitiveDateTime) -> PrimitiveDateTime{
    let gigasec = Duration::new(10_i64.pow(9), 0);
    start.checked_add(gigasec).unwrap()
}