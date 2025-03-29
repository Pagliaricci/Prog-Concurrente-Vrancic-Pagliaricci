pub fn calculate_pi(terms: u64) -> f64 {
    let mut sum = 0.0;
    for k in 0..terms {
        let term = (-1.0f64).powi(k as i32) / (2 * k + 1) as f64;
        sum += term;
    }
    4.0 * sum
}
