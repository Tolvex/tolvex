use std::collections::HashMap;

use crate::{secure_mask_vector, secure_sum};

#[derive(Debug, Clone, PartialEq)]
pub struct ModelUpdate {
    pub client_id: String,
    pub weights: Vec<f64>,
    pub num_examples: u64,
}

impl ModelUpdate {
    pub fn new(client_id: impl Into<String>, weights: Vec<f64>, num_examples: u64) -> Self {
        Self {
            client_id: client_id.into(),
            weights,
            num_examples,
        }
    }
}

pub fn fedavg(updates: &[ModelUpdate]) -> Option<Vec<f64>> {
    if updates.is_empty() {
        return None;
    }

    let dim = updates[0].weights.len();
    if dim == 0 {
        return None;
    }

    let mut total_examples = 0u64;
    for u in updates {
        if u.weights.len() != dim {
            return None;
        }
        total_examples = total_examples.saturating_add(u.num_examples);
    }

    if total_examples == 0 {
        return None;
    }

    let mut out = vec![0.0f64; dim];
    for u in updates {
        let w = (u.num_examples as f64) / (total_examples as f64);
        for (i, slot) in out.iter_mut().enumerate() {
            *slot += w * u.weights[i];
        }
    }
    Some(out)
}

/// Build one client's plaintext contribution vector for secure federated
/// averaging: `update.weights` scaled by `update.num_examples`, with
/// `update.num_examples` itself appended as the last element. Bundling the
/// example count into the same vector means a single `secure_mask_vector` /
/// `secure_sum` round trip recovers both the weighted-sum numerator and the
/// total-examples denominator, mirroring `fedavg`'s weighting without the
/// aggregator ever seeing an individual client's raw weights or count.
fn secure_fedavg_contribution(update: &ModelUpdate) -> Vec<f64> {
    let n = update.num_examples as f64;
    let mut contribution: Vec<f64> = update.weights.iter().map(|w| w * n).collect();
    contribution.push(n);
    contribution
}

/// Recover the federated average from a `secure_sum` of every client's
/// `secure_fedavg_contribution` (after each was masked with
/// `secure_mask_vector` and summed). Returns `None` if `summed` is empty or
/// the recovered total example count is non-positive.
fn secure_fedavg_finalize(summed: &[f64]) -> Option<Vec<f64>> {
    let (total_examples, weighted_sum) = summed.split_last()?;
    if *total_examples <= 0.0 {
        return None;
    }
    Some(weighted_sum.iter().map(|w| w / total_examples).collect())
}

/// Secure-aggregation counterpart of `fedavg`: masks every client's
/// contribution with `secure_mask_vector` (using a `master_secret` shared
/// across the round, see `derive_pairwise_seed`), sums the masked
/// contributions via `secure_sum`, and finalizes the weighted average --
/// reproducing `fedavg`'s result without the aggregator observing any
/// individual client's weights or example count.
///
/// Returns `None` under the same conditions as `fedavg` (empty updates,
/// mismatched weight dimensions, or zero total examples).
pub fn secure_fedavg(updates: &[ModelUpdate], master_secret: u64) -> Option<Vec<f64>> {
    if updates.is_empty() {
        return None;
    }
    let n_clients = updates.len();
    let masked: Option<Vec<Vec<f64>>> = updates
        .iter()
        .enumerate()
        .map(|(idx, update)| {
            let contribution = secure_fedavg_contribution(update);
            secure_mask_vector(&contribution, idx, n_clients, master_secret)
        })
        .collect();
    let summed = secure_sum(&masked?)?;
    secure_fedavg_finalize(&summed)
}

#[derive(Debug, Clone, Default)]
pub struct FederatedCoordinator {
    rounds: HashMap<u64, Vec<ModelUpdate>>,
}

impl FederatedCoordinator {
    pub fn new() -> Self {
        Self {
            rounds: HashMap::new(),
        }
    }

    pub fn submit_update(&mut self, round: u64, update: ModelUpdate) {
        self.rounds.entry(round).or_default().push(update);
    }

    pub fn num_updates(&self, round: u64) -> usize {
        self.rounds.get(&round).map(|v| v.len()).unwrap_or(0)
    }

    pub fn aggregate_round(&self, round: u64) -> Option<Vec<f64>> {
        let updates = self.rounds.get(&round)?;
        fedavg(updates)
    }

    pub fn aggregate_round_min_clients(&self, round: u64, min_clients: usize) -> Option<Vec<f64>> {
        let updates = self.rounds.get(&round)?;
        if updates.len() < min_clients {
            return None;
        }
        fedavg(updates)
    }
}
