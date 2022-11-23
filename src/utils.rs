pub fn time_it<F>(to_time: F) -> u128
where
    F: FnOnce(),
{
    let start = std::time::Instant::now();
    to_time();
    let end = std::time::Instant::now();
    (end - start).as_micros()
}
