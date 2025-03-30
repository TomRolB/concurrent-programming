pub fn compute_pi(digit_position: u32) -> f64 {
    (0..=digit_position)
        .map(|n| (-1i32).pow(n) as f64 / (2.0 * (n as f64) + 1.0))
        .sum::<f64>()
        * 4.0
}
