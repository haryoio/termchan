pub fn unix_now_time() -> f64 {
    let now = std::time::SystemTime::now();
    let now = now.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    now as f64
}
