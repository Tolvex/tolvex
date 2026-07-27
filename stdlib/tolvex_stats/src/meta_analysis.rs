//! Meta-analysis: fixed-effects and random-effects (DerSimonian–Laird)
//! pooling of study-level effect sizes via inverse-variance weighting.

/// 97.5th percentile of the standard normal distribution, used for 95% CIs.
const Z_975: f64 = 1.959963984540054;

/// Pooled result of a meta-analysis over `k` studies.
#[derive(Debug, Clone)]
pub struct MetaAnalysisResult {
    pub pooled_effect: f64,
    pub se: f64,
    pub ci_lower: f64,
    pub ci_upper: f64,
    /// Cochran's Q heterogeneity statistic (computed with fixed-effect weights).
    pub q_stat: f64,
    /// I²: percentage of total variability attributable to heterogeneity, in `[0, 100]`.
    pub i_squared: f64,
    /// Between-study variance (tau²); `0.0` for fixed-effects pooling.
    pub tau_squared: f64,
    pub k: usize,
}

fn weighted_pool(effects: &[f64], weights: &[f64]) -> (f64, f64) {
    let sum_w: f64 = weights.iter().sum();
    let pooled = effects
        .iter()
        .zip(weights.iter())
        .map(|(y, w)| y * w)
        .sum::<f64>()
        / sum_w;
    let se = (1.0 / sum_w).sqrt();
    (pooled, se)
}

fn cochrans_q(effects: &[f64], weights: &[f64], pooled: f64) -> f64 {
    effects
        .iter()
        .zip(weights.iter())
        .map(|(y, w)| w * (y - pooled).powi(2))
        .sum()
}

/// Validate `effects`/`variances` and, if valid, return the fixed-effect
/// (inverse-variance) pooling result alongside the weights used to compute
/// it, so callers needing those weights (e.g. the random-effects estimator)
/// don't have to recompute them.
///
/// Returns `None` if the inputs mismatch in length, are empty, or contain a
/// non-positive or NaN variance.
fn fixed_effects_core(
    effects: &[f64],
    variances: &[f64],
) -> Option<(MetaAnalysisResult, Vec<f64>)> {
    let k = effects.len();
    if k == 0 || k != variances.len() || variances.iter().any(|&v| v.is_nan() || v <= 0.0) {
        return None;
    }
    let weights: Vec<f64> = variances.iter().map(|v| 1.0 / v).collect();
    let (pooled, se) = weighted_pool(effects, &weights);
    let q_stat = cochrans_q(effects, &weights, pooled);
    let df = (k - 1) as f64;
    let i_squared = if q_stat > 0.0 {
        ((q_stat - df) / q_stat * 100.0).max(0.0)
    } else {
        0.0
    };
    let result = MetaAnalysisResult {
        pooled_effect: pooled,
        se,
        ci_lower: pooled - Z_975 * se,
        ci_upper: pooled + Z_975 * se,
        q_stat,
        i_squared,
        tau_squared: 0.0,
        k,
    };
    Some((result, weights))
}

/// Fixed-effects (inverse-variance) meta-analysis.
///
/// `effects` are per-study effect estimates (e.g. log risk ratios, mean
/// differences); `variances` are their corresponding sampling variances
/// (standard error squared). Returns `None` if the inputs mismatch in
/// length, are empty, or contain a non-positive or NaN variance.
pub fn fixed_effects_meta_analysis(
    effects: &[f64],
    variances: &[f64],
) -> Option<MetaAnalysisResult> {
    fixed_effects_core(effects, variances).map(|(result, _)| result)
}

/// Random-effects meta-analysis using the DerSimonian–Laird estimator of
/// between-study variance (tau²).
///
/// Returns `None` under the same conditions as [`fixed_effects_meta_analysis`].
pub fn random_effects_meta_analysis(
    effects: &[f64],
    variances: &[f64],
) -> Option<MetaAnalysisResult> {
    let (fixed, fixed_weights) = fixed_effects_core(effects, variances)?;
    let k = fixed.k;

    let sum_w: f64 = fixed_weights.iter().sum();
    let sum_w2: f64 = fixed_weights.iter().map(|w| w * w).sum();
    let df = (k - 1) as f64;
    let c = sum_w - sum_w2 / sum_w;
    let tau_squared = if c > 0.0 {
        ((fixed.q_stat - df) / c).max(0.0)
    } else {
        0.0
    };

    let weights: Vec<f64> = variances.iter().map(|v| 1.0 / (v + tau_squared)).collect();
    let (pooled, se) = weighted_pool(effects, &weights);

    Some(MetaAnalysisResult {
        pooled_effect: pooled,
        se,
        ci_lower: pooled - Z_975 * se,
        ci_upper: pooled + Z_975 * se,
        q_stat: fixed.q_stat,
        i_squared: fixed.i_squared,
        tau_squared,
        k,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_effects_identical_studies_reduces_to_input() {
        let effects = [0.5, 0.5, 0.5];
        let variances = [0.04, 0.04, 0.04];
        let r = fixed_effects_meta_analysis(&effects, &variances).unwrap();
        assert!((r.pooled_effect - 0.5).abs() < 1e-12);
        assert!(r.q_stat.abs() < 1e-9);
        assert_eq!(r.i_squared, 0.0);
        assert_eq!(r.tau_squared, 0.0);
    }

    #[test]
    fn fixed_effects_hand_computed_two_studies() {
        // w1 = 1/0.25 = 4, w2 = 1/1.0 = 1
        // pooled = (4*1.0 + 1*2.0) / 5 = 1.2, se = sqrt(1/5)
        let effects = [1.0, 2.0];
        let variances = [0.25, 1.0];
        let r = fixed_effects_meta_analysis(&effects, &variances).unwrap();
        assert!((r.pooled_effect - 1.2).abs() < 1e-9);
        assert!((r.se - (1.0f64 / 5.0).sqrt()).abs() < 1e-9);
        assert!(r.ci_lower < r.pooled_effect && r.pooled_effect < r.ci_upper);
    }

    #[test]
    fn random_effects_heterogeneous_studies_widens_ci_vs_fixed() {
        let effects = [0.1, 0.9, 0.5, -0.3];
        let variances = [0.05, 0.05, 0.05, 0.05];
        let fixed = fixed_effects_meta_analysis(&effects, &variances).unwrap();
        let random = random_effects_meta_analysis(&effects, &variances).unwrap();
        assert!(random.tau_squared > 0.0);
        assert!(random.se >= fixed.se);
        assert!(random.ci_upper - random.ci_lower >= fixed.ci_upper - fixed.ci_lower);
    }

    #[test]
    fn random_effects_homogeneous_studies_matches_fixed() {
        // No between-study heterogeneity -> tau^2 == 0 and pooling matches fixed-effects.
        let effects = [0.5, 0.5, 0.5];
        let variances = [0.04, 0.09, 0.02];
        let fixed = fixed_effects_meta_analysis(&effects, &variances).unwrap();
        let random = random_effects_meta_analysis(&effects, &variances).unwrap();
        assert_eq!(random.tau_squared, 0.0);
        assert!((random.pooled_effect - fixed.pooled_effect).abs() < 1e-9);
    }

    #[test]
    fn invalid_inputs_return_none() {
        assert!(fixed_effects_meta_analysis(&[], &[]).is_none());
        assert!(fixed_effects_meta_analysis(&[1.0], &[1.0, 2.0]).is_none());
        assert!(fixed_effects_meta_analysis(&[1.0], &[0.0]).is_none());
        assert!(random_effects_meta_analysis(&[1.0], &[-1.0]).is_none());
        assert!(fixed_effects_meta_analysis(&[1.0, 2.0], &[0.25, f64::NAN]).is_none());
        assert!(random_effects_meta_analysis(&[1.0, 2.0], &[0.25, f64::NAN]).is_none());
    }
}
