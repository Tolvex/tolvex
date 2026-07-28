use core::cmp::Ordering;
use rand::Rng;

pub fn laplace_scale(sensitivity: f64, epsilon: f64) -> Option<f64> {
    if sensitivity < 0.0 {
        return None;
    }
    if epsilon.partial_cmp(&0.0) != Some(Ordering::Greater) {
        return None;
    }
    Some(sensitivity / epsilon)
}

pub fn laplace_noise(scale: f64, rng: &mut impl Rng) -> Option<f64> {
    if scale.partial_cmp(&0.0) != Some(Ordering::Greater) {
        return None;
    }

    let u: f64 = rng.gen_range(-0.5..0.5);
    let s = if u >= 0.0 { 1.0 } else { -1.0 };
    let mag = (1.0 - (2.0 * u.abs())).ln();
    Some(-scale * s * mag)
}

pub fn laplace_mechanism(
    value: f64,
    sensitivity: f64,
    epsilon: f64,
    rng: &mut impl Rng,
) -> Option<f64> {
    let scale = laplace_scale(sensitivity, epsilon)?;
    let n = laplace_noise(scale, rng)?;
    Some(value + n)
}

pub fn gaussian_sigma(sensitivity: f64, epsilon: f64, delta: f64) -> Option<f64> {
    if sensitivity < 0.0 {
        return None;
    }
    if epsilon.partial_cmp(&0.0) != Some(Ordering::Greater) {
        return None;
    }
    if delta.partial_cmp(&0.0) != Some(Ordering::Greater) || delta >= 1.0 {
        return None;
    }

    let c = (2.0 * (1.25 / delta).ln()).sqrt();
    Some(c * sensitivity / epsilon)
}

pub fn gaussian_noise(sigma: f64, rng: &mut impl Rng) -> Option<f64> {
    if sigma.partial_cmp(&0.0) != Some(Ordering::Greater) {
        return None;
    }

    let u1: f64 = rng.gen_range(0.0..1.0);
    let u2: f64 = rng.gen_range(0.0..1.0);
    let r = (-2.0 * u1.ln()).sqrt();
    let theta = 2.0 * core::f64::consts::PI * u2;
    let z0 = r * theta.cos();
    Some(z0 * sigma)
}

pub fn gaussian_mechanism(
    value: f64,
    sensitivity: f64,
    epsilon: f64,
    delta: f64,
    rng: &mut impl Rng,
) -> Option<f64> {
    let sigma = gaussian_sigma(sensitivity, epsilon, delta)?;
    let n = gaussian_noise(sigma, rng)?;
    Some(value + n)
}

/// Probability of reporting the true bit under `epsilon`-local-differential-
/// privacy binary randomized response: `p = e^epsilon / (e^epsilon + 1)`.
/// Requires a finite `epsilon > 0`; `p` is always in `(0.5, 1.0)`.
///
/// Returns `None` for non-finite `epsilon` (e.g. `f64::INFINITY`), which
/// would otherwise compute `e^epsilon / (e^epsilon + 1)` as `INFINITY /
/// INFINITY = NaN`.
pub fn randomized_response_probability(epsilon: f64) -> Option<f64> {
    if epsilon.partial_cmp(&0.0) != Some(Ordering::Greater) || !epsilon.is_finite() {
        return None;
    }
    let e = epsilon.exp();
    Some(e / (e + 1.0))
}

/// Local-DP binary randomized response: reports `true_bit` with probability
/// `p = randomized_response_probability(epsilon)` and its flip otherwise, so
/// no single response reveals `true_bit` with confidence above what
/// `epsilon` permits.
pub fn randomized_response(true_bit: bool, epsilon: f64, rng: &mut impl Rng) -> Option<bool> {
    let p = randomized_response_probability(epsilon)?;
    let keep = rng.gen_range(0.0..1.0) < p;
    Some(if keep { true_bit } else { !true_bit })
}

/// Debias an aggregate of `randomized_response` reports back to an estimate
/// of the true population proportion of `true` values.
///
/// `observed_true_count` is the number of `true` reports out of `n`
/// respondents, each randomized at the given `epsilon`. Returns `None` if
/// `n == 0` or `epsilon` is invalid; the result is clamped to `[0, 1]` since
/// the raw method-of-moments estimator can fall outside that range for small
/// `n` or extreme observed counts.
pub fn randomized_response_debias(observed_true_count: u64, n: usize, epsilon: f64) -> Option<f64> {
    if n == 0 || observed_true_count > n as u64 {
        return None;
    }
    let p = randomized_response_probability(epsilon)?;
    let observed_freq = observed_true_count as f64 / n as f64;
    // p != 0.5 is guaranteed since epsilon > 0 => p in (0.5, 1.0).
    let estimate = (observed_freq + p - 1.0) / (2.0 * p - 1.0);
    Some(estimate.clamp(0.0, 1.0))
}
