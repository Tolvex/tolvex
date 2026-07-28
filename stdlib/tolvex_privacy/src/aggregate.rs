use rand::Rng;

use crate::{gaussian_mechanism, laplace_mechanism, PrivacyBudget};

pub fn private_sum_laplace(
    xs: &[f64],
    sensitivity: f64,
    epsilon: f64,
    rng: &mut impl Rng,
) -> Option<f64> {
    let s = xs.iter().sum::<f64>();
    laplace_mechanism(s, sensitivity, epsilon, rng)
}

pub fn private_mean_laplace(
    xs: &[f64],
    sensitivity: f64,
    epsilon: f64,
    rng: &mut impl Rng,
) -> Option<f64> {
    if xs.is_empty() {
        return None;
    }
    let mean = xs.iter().sum::<f64>() / (xs.len() as f64);
    laplace_mechanism(mean, sensitivity, epsilon, rng)
}

pub fn private_sum_gaussian(
    xs: &[f64],
    sensitivity: f64,
    epsilon: f64,
    delta: f64,
    rng: &mut impl Rng,
) -> Option<f64> {
    let s = xs.iter().sum::<f64>();
    gaussian_mechanism(s, sensitivity, epsilon, delta, rng)
}

pub fn private_histogram_laplace(
    counts: &[u64],
    sensitivity: f64,
    epsilon: f64,
    rng: &mut impl Rng,
) -> Option<Vec<f64>> {
    let mut out = Vec::with_capacity(counts.len());
    for &c in counts {
        let noisy = laplace_mechanism(c as f64, sensitivity, epsilon, rng)?;
        out.push(noisy.max(0.0));
    }
    Some(out)
}

pub fn private_sum_laplace_budgeted(
    xs: &[f64],
    sensitivity: f64,
    epsilon: f64,
    budget: &mut PrivacyBudget,
    rng: &mut impl Rng,
) -> Option<f64> {
    if !budget.spend(epsilon, 0.0) {
        return None;
    }
    private_sum_laplace(xs, sensitivity, epsilon, rng)
}

/// Deterministic pairwise mask vector of length `dim`, seeded so that both
/// parties in a client pair derive the identical mask (see
/// `derive_pairwise_seed`) without further communication.
fn pairwise_mask_vector(seed: u64, dim: usize) -> Vec<f64> {
    use rand::{rngs::StdRng, SeedableRng};
    let mut rng = StdRng::seed_from_u64(seed);
    (0..dim).map(|_| rng.gen_range(-1.0e6..1.0e6)).collect()
}

/// Symmetric shared seed for a client pair, derived from a common
/// `master_secret` known to both (in a full deployment this would come from
/// a per-pair Diffie-Hellman exchange; this models the already-agreed-key
/// stage of Bonawitz et al.'s secure-aggregation protocol). Order-independent
/// in `client_a`/`client_b` so both clients compute the same seed.
pub fn derive_pairwise_seed(master_secret: u64, client_a: usize, client_b: usize) -> u64 {
    let (lo, hi) = if client_a <= client_b {
        (client_a, client_b)
    } else {
        (client_b, client_a)
    };
    master_secret
        ^ (lo as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15)
        ^ (hi as u64).wrapping_mul(0xC2B2_AE3D_27D4_EB4F)
}

/// Mask `vector` (one client's private contribution) for secure
/// aggregation among `n_clients` participants via pairwise additive masking.
/// `client_idx` is this client's 0-based index among `n_clients`.
///
/// Every other client's mask uses the same `master_secret` and the
/// symmetric `derive_pairwise_seed`, so once every client's masked vector is
/// summed by `secure_sum`, all pairwise masks cancel exactly and only the
/// true sum is revealed -- no individual masked vector (nor a strict subset
/// of them) exposes this client's raw `vector`.
///
/// Returns `None` if `vector` is empty, `n_clients == 0`, or
/// `client_idx >= n_clients`.
pub fn secure_mask_vector(
    vector: &[f64],
    client_idx: usize,
    n_clients: usize,
    master_secret: u64,
) -> Option<Vec<f64>> {
    if vector.is_empty() || n_clients == 0 || client_idx >= n_clients {
        return None;
    }
    let dim = vector.len();
    let mut masked = vector.to_vec();
    for j in 0..n_clients {
        if j == client_idx {
            continue;
        }
        let seed = derive_pairwise_seed(master_secret, client_idx, j);
        let mask = pairwise_mask_vector(seed, dim);
        let sign = if j > client_idx { 1.0 } else { -1.0 };
        for (m, mask_val) in masked.iter_mut().zip(mask.iter()) {
            *m += sign * mask_val;
        }
    }
    Some(masked)
}

/// Sum masked vectors produced by `secure_mask_vector`, recovering the true
/// sum of the underlying private vectors once the pairwise masks cancel.
///
/// Requires every participating client's masked vector to be present --
/// summing a strict subset leaves uncancelled masks and returns a
/// meaningless (not merely noisy) result. Returns `None` if `masked_vectors`
/// is empty or the vectors have mismatched or zero length.
pub fn secure_sum(masked_vectors: &[Vec<f64>]) -> Option<Vec<f64>> {
    let dim = masked_vectors.first()?.len();
    if dim == 0 || masked_vectors.iter().any(|v| v.len() != dim) {
        return None;
    }
    let mut out = vec![0.0f64; dim];
    for v in masked_vectors {
        for (o, x) in out.iter_mut().zip(v.iter()) {
            *o += x;
        }
    }
    Some(out)
}
