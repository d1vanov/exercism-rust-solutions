use time::PrimitiveDateTime as DateTime;

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let gigasecond: i64 = 1000000000;
    let dt =
        time::OffsetDateTime::from_unix_timestamp(start.assume_utc().unix_timestamp() + gigasecond)
            .unwrap();
    DateTime::new(dt.date(), dt.time())
}
