use approx::assert_abs_diff_eq;

use tolvex_privacy::{derive_pairwise_seed, secure_mask_vector, secure_sum};

#[test]
fn secure_sum_recovers_true_sum_across_clients() {
    let vectors = [vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]];
    let n_clients = vectors.len();
    let master_secret = 42u64;

    let masked: Vec<Vec<f64>> = vectors
        .iter()
        .enumerate()
        .map(|(idx, v)| secure_mask_vector(v, idx, n_clients, master_secret).expect("mask"))
        .collect();

    let summed = secure_sum(&masked).expect("sum");
    assert_abs_diff_eq!(summed[0], 9.0, epsilon = 1e-6);
    assert_abs_diff_eq!(summed[1], 12.0, epsilon = 1e-6);
}

#[test]
fn secure_sum_over_subset_does_not_match_true_sum() {
    // A strict subset of masked vectors leaves uncancelled pairwise masks --
    // the "sum" is meaningless, not just noisy, so it should not match the
    // true partial sum of [1,2] + [3,4] = [4,6].
    let vectors = [vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]];
    let n_clients = vectors.len();
    let master_secret = 42u64;

    let masked: Vec<Vec<f64>> = vectors
        .iter()
        .enumerate()
        .map(|(idx, v)| secure_mask_vector(v, idx, n_clients, master_secret).expect("mask"))
        .collect();

    let partial = secure_sum(&masked[0..2]).expect("sum");
    assert!((partial[0] - 4.0).abs() > 1e-3 || (partial[1] - 6.0).abs() > 1e-3);
}

#[test]
fn secure_mask_vector_rejects_invalid_inputs() {
    assert!(secure_mask_vector(&[], 0, 2, 1).is_none());
    assert!(secure_mask_vector(&[1.0], 2, 2, 1).is_none());
    assert!(secure_mask_vector(&[1.0], 0, 0, 1).is_none());
}

#[test]
fn secure_sum_rejects_empty_or_mismatched() {
    assert!(secure_sum(&[]).is_none());
    assert!(secure_sum(&[vec![]]).is_none());
    assert!(secure_sum(&[vec![1.0, 2.0], vec![1.0]]).is_none());
}

#[test]
fn derive_pairwise_seed_is_symmetric_and_pair_specific() {
    assert_eq!(derive_pairwise_seed(7, 0, 1), derive_pairwise_seed(7, 1, 0));
    assert_ne!(derive_pairwise_seed(7, 0, 1), derive_pairwise_seed(7, 0, 2));
}
