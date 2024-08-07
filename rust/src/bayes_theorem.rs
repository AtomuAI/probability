mod test;

pub fn bayes_theorem(p_b_given_a: f64, p_a: f64, p_b: f64) -> f64 {
    if p_b == 0.0 {
        panic!("The probability of B (p_b) cannot be zero.");
    }
    (p_b_given_a * p_a) / p_b
}