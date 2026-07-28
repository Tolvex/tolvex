use approx::assert_abs_diff_eq;
use rand::{rngs::StdRng, Rng, SeedableRng};

use tolvex_privacy::{
    gaussian_sigma, laplace_scale, private_histogram_laplace, private_mean_laplace,
    private_sum_gaussian, private_sum_laplace, randomized_response, randomized_response_debias,
    randomized_response_probability, PrivacyBudget,
};

#[test]
fn laplace_scale_basic() {
    let b = laplace_scale(1.0, 2.0).expect("scale");
    assert_abs_diff_eq!(b, 0.5, epsilon = 1e-12);
    assert!(laplace_scale(1.0, 0.0).is_none());
}

#[test]
fn gaussian_sigma_basic() {
    let s = gaussian_sigma(1.0, 1.0, 1e-5).expect("sigma");
    assert!(s.is_finite());
    assert!(gaussian_sigma(1.0, 0.0, 1e-5).is_none());
    assert!(gaussian_sigma(1.0, 1.0, 0.0).is_none());
}

#[test]
fn private_sum_and_mean_deterministic_with_seeded_rng() {
    let xs = [1.0, 2.0, 3.0];

    let mut rng1 = StdRng::seed_from_u64(123);
    let mut rng2 = StdRng::seed_from_u64(123);

    let s1 = private_sum_laplace(&xs, 1.0, 1.0, &mut rng1).expect("sum");
    let s2 = private_sum_laplace(&xs, 1.0, 1.0, &mut rng2).expect("sum");
    assert_abs_diff_eq!(s1, s2, epsilon = 1e-12);

    let mut rng3 = StdRng::seed_from_u64(999);
    let m = private_mean_laplace(&xs, 1.0, 1.0, &mut rng3).expect("mean");
    assert!(m.is_finite());
}

#[test]
fn private_sum_gaussian_smoke() {
    let xs = [1.0, 2.0, 3.0];
    let mut rng = StdRng::seed_from_u64(7);
    let s = private_sum_gaussian(&xs, 1.0, 1.0, 1e-5, &mut rng).expect("sum");
    assert!(s.is_finite());
}

#[test]
fn private_histogram_nonnegative() {
    let counts = [10u64, 0u64, 5u64];
    let mut rng = StdRng::seed_from_u64(42);
    let out = private_histogram_laplace(&counts, 1.0, 1.0, &mut rng).expect("hist");
    assert_eq!(out.len(), 3);
    for v in out {
        assert!(v >= 0.0);
    }
}

#[test]
fn budget_spend_blocks_overuse() {
    let mut b = PrivacyBudget::new(1.0, 0.0);
    assert!(b.spend(0.4, 0.0));
    assert!(b.spend(0.6, 0.0));
    assert!(!b.spend(0.1, 0.0));
}

#[test]
fn randomized_response_probability_basic() {
    let p = randomized_response_probability(1.0).expect("p");
    assert!(p > 0.5 && p < 1.0);
    assert!(randomized_response_probability(0.0).is_none());
    assert!(randomized_response_probability(-1.0).is_none());
}

#[test]
fn randomized_response_deterministic_with_seeded_rng() {
    let mut rng1 = StdRng::seed_from_u64(5);
    let mut rng2 = StdRng::seed_from_u64(5);
    let r1 = randomized_response(true, 2.0, &mut rng1).expect("r1");
    let r2 = randomized_response(true, 2.0, &mut rng2).expect("r2");
    assert_eq!(r1, r2);
    assert!(randomized_response(true, 0.0, &mut rng1).is_none());
}

#[test]
fn randomized_response_debias_recovers_true_proportion_statistically() {
    // Simulate n respondents whose true bit is `true` with probability 0.7,
    // randomized at epsilon = 2.0, and check the debiased estimate is close
    // to the true underlying proportion.
    let epsilon = 2.0;
    let true_prop = 0.7;
    let n = 20_000usize;
    let mut rng = StdRng::seed_from_u64(99);
    let mut observed_true = 0u64;
    for _ in 0..n {
        let true_bit = rng.gen_range(0.0..1.0) < true_prop;
        let reported = randomized_response(true_bit, epsilon, &mut rng).expect("rr");
        if reported {
            observed_true += 1;
        }
    }
    let estimate = randomized_response_debias(observed_true, n, epsilon).expect("debias");
    assert!((estimate - true_prop).abs() < 0.05, "estimate={estimate}");
}

#[test]
fn randomized_response_debias_invalid_inputs() {
    assert!(randomized_response_debias(0, 0, 1.0).is_none());
    assert!(randomized_response_debias(5, 3, 1.0).is_none());
    assert!(randomized_response_debias(1, 10, 0.0).is_none());
    let d = randomized_response_debias(0, 10, 1.0).expect("d");
    assert!((0.0..=1.0).contains(&d));
}
