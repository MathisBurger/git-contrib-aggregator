use chrono::{Duration, Utc};

pub fn get_date_range() -> (String, String) {
    let today = Utc::now();
    let one_year_ago = today - Duration::days(365); // approx 1 year

    let from = one_year_ago.to_rfc3339(); // ISO 8601 format
    let to = today.to_rfc3339();

    (from, to)
}
