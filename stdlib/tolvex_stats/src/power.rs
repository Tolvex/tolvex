//! Statistical power analysis utilities (feature-gated for pvalue/statrs)

#[cfg(feature = "pvalue")]
use statrs::distribution::{ContinuousCDF, Normal};

/// Compute Cohen's d from means and pooled standard deviation
pub fn cohen_d(mean1: f64, mean2: f64, pooled_sd: f64) -> f64 {
    if pooled_sd <= 0.0 {
        0.0
    } else {
        (mean1 - mean2) / pooled_sd
    }
}

/// Approximate power for two-sample t-test (two-sided) using normal approximation.
/// effect_size = Cohen's d, n_per_group = samples in each group, alpha in (0,1).
#[cfg(feature = "pvalue")]
pub fn power_two_sample_t(effect_size: f64, alpha: f64, n_per_group: usize) -> f64 {
    if effect_size <= 0.0 || alpha <= 0.0 || alpha >= 1.0 || n_per_group == 0 {
        return 0.0;
    }
    let n = n_per_group as f64;
    let z_alpha = Normal::new(0.0, 1.0)
        .unwrap()
        .inverse_cdf(1.0 - alpha / 2.0);
    let z = effect_size * (n / 2.0).sqrt();
    let phi = |x: f64| Normal::new(0.0, 1.0).unwrap().cdf(x);
    // Two-sided power ~ 1 - beta, beta = Phi(z_alpha - z) - Phi(-z_alpha - z)
    let beta = phi(z_alpha - z) - phi(-z_alpha - z);
    (1.0 - beta).clamp(0.0, 1.0)
}

/// Approximate required per-group sample size to achieve target power
/// for two-sample t-test (two-sided) using normal approximation.
#[cfg(feature = "pvalue")]
pub fn sample_size_two_sample_t(effect_size: f64, alpha: f64, target_power: f64) -> usize {
    if effect_size <= 0.0 || alpha <= 0.0 || alpha >= 1.0 || !(0.0..=1.0).contains(&target_power) {
        return 0;
    }
    let norm = Normal::new(0.0, 1.0).unwrap();
    // simple search over n to achieve power >= target
    let mut n = 2usize;
    loop {
        let p = power_two_sample_t(effect_size, alpha, n);
        if p >= target_power {
            return n;
        }
        n += 1;
        if n > 100000 {
            return 0;
        }
    }
}

/// Approximate required per-group sample size to detect a difference
/// between two proportions (two-sided two-proportion z-test) using the
/// normal approximation (Fleiss). Returns `0` for invalid inputs
/// (out-of-range probabilities/alpha/power, or `p1 == p2`, which requires
/// infinite `n`).
#[cfg(feature = "pvalue")]
pub fn sample_size_proportion(p1: f64, p2: f64, alpha: f64, target_power: f64) -> usize {
    if !(0.0..=1.0).contains(&p1)
        || !(0.0..=1.0).contains(&p2)
        || alpha <= 0.0
        || alpha >= 1.0
        || !(0.0..1.0).contains(&target_power)
        || p1 == p2
    {
        return 0;
    }
    let z_alpha = Normal::new(0.0, 1.0)
        .unwrap()
        .inverse_cdf(1.0 - alpha / 2.0);
    let z_beta = Normal::new(0.0, 1.0).unwrap().inverse_cdf(target_power);
    let p_bar = (p1 + p2) / 2.0;
    let q_bar = 1.0 - p_bar;
    let diff = (p1 - p2).abs();
    let n = (z_alpha * (2.0 * p_bar * q_bar).sqrt()
        + z_beta * (p1 * (1.0 - p1) + p2 * (1.0 - p2)).sqrt())
    .powi(2)
        / diff.powi(2);
    n.ceil().max(1.0) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d_zero_when_sd_zero() {
        assert_eq!(cohen_d(1.0, 0.0, 0.0), 0.0);
    }

    #[cfg(feature = "pvalue")]
    #[test]
    fn power_increases_with_n() {
        let e = 0.5; // medium effect
        let p_small = power_two_sample_t(e, 0.05, 10);
        let p_large = power_two_sample_t(e, 0.05, 50);
        assert!(p_large > p_small);
    }

    #[cfg(feature = "pvalue")]
    #[test]
    fn sample_size_proportion_basic() {
        // Larger effect size (bigger |p1 - p2|) should need fewer subjects.
        let n_small_effect = sample_size_proportion(0.30, 0.35, 0.05, 0.8);
        let n_large_effect = sample_size_proportion(0.30, 0.60, 0.05, 0.8);
        assert!(n_small_effect > n_large_effect);
        assert!(n_large_effect > 0);
    }

    #[cfg(feature = "pvalue")]
    #[test]
    fn sample_size_proportion_invalid_inputs() {
        assert_eq!(sample_size_proportion(0.3, 0.3, 0.05, 0.8), 0);
        assert_eq!(sample_size_proportion(1.5, 0.3, 0.05, 0.8), 0);
        assert_eq!(sample_size_proportion(0.3, 0.5, 0.0, 0.8), 0);
        assert_eq!(sample_size_proportion(0.3, 0.5, 0.05, 1.0), 0);
    }
}
