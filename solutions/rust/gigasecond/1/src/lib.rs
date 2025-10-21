use time::{PrimitiveDateTime as DateTime, Duration};

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start.checked_add(Duration::seconds(1000000000)).unwrap()
}
