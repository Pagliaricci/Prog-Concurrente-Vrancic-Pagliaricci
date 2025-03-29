pub fn calculate_pi(terms: u64) -> f64 {
    let mut pi = 0.0;
    let mut denominator = 1.0;
    let mut sign = 1.0;

    for _ in 0..terms {
        pi += sign / denominator;
        denominator += 2.0;
        sign *= -1.0;
    }

    pi * 4.0
}