//! Survival analysis: Kaplan–Meier estimator and log-rank test (simplified)

/// Kaplan–Meier output
#[derive(Debug, Clone)]
pub struct KaplanMeier {
    pub times: Vec<f64>,
    pub survival_prob: Vec<f64>,
    pub at_risk: Vec<usize>,
    pub events: Vec<usize>,
}

/// Compute Kaplan–Meier survival curve.
/// times: event/censor times; events: true if event occurred, false if censored.
pub fn kaplan_meier(times: &[f64], events: &[bool]) -> KaplanMeier {
    assert_eq!(times.len(), events.len());
    let n = times.len();
    if n == 0 {
        return KaplanMeier {
            times: vec![],
            survival_prob: vec![],
            at_risk: vec![],
            events: vec![],
        };
    }
    // Combine and sort by time
    let mut data: Vec<(f64, bool)> = times.iter().cloned().zip(events.iter().cloned()).collect();
    data.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut uniq_times: Vec<f64> = Vec::new();
    let mut d: Vec<usize> = Vec::new(); // events
    let mut n_risk: Vec<usize> = Vec::new(); // at risk just prior

    let mut i = 0;
    while i < n {
        let t = data[i].0;
        let at_risk = n - i; // number still under observation just prior to t
        let mut events_t = 0usize;
        let mut j = i;
        while j < n && data[j].0 == t {
            if data[j].1 {
                events_t += 1;
            }
            j += 1;
        }
        uniq_times.push(t);
        d.push(events_t);
        n_risk.push(at_risk);
        i = j;
    }

    let mut s = 1.0f64;
    let mut surv: Vec<f64> = Vec::with_capacity(uniq_times.len());
    for (&ni, &di) in n_risk.iter().zip(d.iter()) {
        if ni == 0 {
            surv.push(s);
            continue;
        }
        let frac = 1.0 - (di as f64) / (ni as f64);
        s *= frac.max(0.0);
        surv.push(s);
    }

    KaplanMeier {
        times: uniq_times,
        survival_prob: surv,
        at_risk: n_risk,
        events: d,
    }
}

/// Simplified log-rank test (returns chi-square statistic p-value using approx. normal/chi-square)
/// For robustness and to avoid extra deps, we return the chi-square statistic only.
/// Caller can compare against critical values or compute p-value externally if desired.
pub fn log_rank_test_stat(
    group1_times: &[f64],
    group1_events: &[bool],
    group2_times: &[f64],
    group2_events: &[bool],
) -> f64 {
    assert_eq!(group1_times.len(), group1_events.len());
    assert_eq!(group2_times.len(), group2_events.len());
    // Merge unique times across groups
    let mut all: Vec<(f64, usize, bool)> = Vec::new();
    for i in 0..group1_times.len() {
        all.push((group1_times[i], 0, group1_events[i]));
    }
    for i in 0..group2_times.len() {
        all.push((group2_times[i], 1, group2_events[i]));
    }
    all.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    // At each unique time t with events, compute observed and expected in group 1
    let mut uniq_times: Vec<f64> = Vec::new();
    let mut d1_t: Vec<usize> = Vec::new();
    let mut d_t: Vec<usize> = Vec::new();
    let mut n1_t: Vec<usize> = Vec::new();
    let mut n_t: Vec<usize> = Vec::new();

    let mut i = 0;
    while i < all.len() {
        let t = all[i].0;
        let mut d1 = 0usize;
        let mut d = 0usize;
        let mut n1 = 0usize;
        let mut n = 0usize;
        // Count at risk just prior to t
        for item in all.iter().skip(i) {
            // at risk: those with time >= t
            n += 1;
            if item.1 == 0 {
                n1 += 1;
            }
        }
        // Events at t
        let mut j = i;
        while j < all.len() && all[j].0 == t {
            if all[j].2 {
                d += 1;
                if all[j].1 == 0 {
                    d1 += 1;
                }
            }
            j += 1;
        }
        if d > 0 {
            uniq_times.push(t);
            d1_t.push(d1);
            d_t.push(d);
            n1_t.push(n1);
            n_t.push(n);
        }
        i = j;
    }

    // Compute log-rank statistic: (sum (d1 - E1))^2 / sum Var(d1)
    let mut num = 0.0f64;
    let mut den = 0.0f64;
    for k in 0..uniq_times.len() {
        let n = n_t[k] as f64;
        let n1 = n1_t[k] as f64;
        let d = d_t[k] as f64;
        let d1 = d1_t[k] as f64;
        if n == 0.0 {
            continue;
        }
        let e1 = d * (n1 / n);
        let v1 = d * (n - d) * (n1 / n) * (1.0 - n1 / n) / (n - 1.0).max(1.0);
        num += d1 - e1;
        den += v1;
    }
    if den <= 0.0 {
        0.0
    } else {
        (num * num) / den
    }
}

/// One distinct event time with the subjects who had an event there and where
/// the risk set (subjects with time >= t) begins in `order`.
struct TimeGroup {
    event_idx: Vec<usize>,
    risk_start: usize,
}

/// Sort subjects by time and group distinct times that have at least one event,
/// recording each group's event members and where its risk set starts in `order`.
fn time_groups(times: &[f64], events: &[bool]) -> (Vec<usize>, Vec<TimeGroup>) {
    let n = times.len();
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&a, &b| times[a].partial_cmp(&times[b]).unwrap());

    let mut groups = Vec::new();
    let mut i = 0;
    while i < n {
        let t = times[order[i]];
        let mut j = i;
        let mut event_idx = Vec::new();
        while j < n && times[order[j]] == t {
            if events[order[j]] {
                event_idx.push(order[j]);
            }
            j += 1;
        }
        if !event_idx.is_empty() {
            groups.push(TimeGroup {
                event_idx,
                risk_start: i,
            });
        }
        i = j;
    }
    (order, groups)
}

/// Breslow partial log-likelihood, score (gradient), and observed information
/// (negative Hessian) of a Cox proportional hazards model at a given `beta`.
fn score_info(
    covariates: &[Vec<f64>],
    order: &[usize],
    groups: &[TimeGroup],
    beta: &[f64],
    p: usize,
) -> (f64, Vec<f64>, Vec<Vec<f64>>) {
    let mut ll = 0.0f64;
    let mut grad = vec![0.0f64; p];
    let mut info = vec![vec![0.0f64; p]; p];

    for g in groups {
        let d_k = g.event_idx.len() as f64;

        let mut sum_event_lp = 0.0f64;
        let mut sum_event_cov = vec![0.0f64; p];
        for &idx in &g.event_idx {
            let lp: f64 = covariates[idx]
                .iter()
                .zip(beta.iter())
                .map(|(x, b)| x * b)
                .sum();
            sum_event_lp += lp;
            for (k, sum_k) in sum_event_cov.iter_mut().enumerate() {
                *sum_k += covariates[idx][k];
            }
        }

        let mut s0 = 0.0f64;
        let mut s1 = vec![0.0f64; p];
        let mut s2 = vec![vec![0.0f64; p]; p];
        for &idx in &order[g.risk_start..] {
            let lp: f64 = covariates[idx]
                .iter()
                .zip(beta.iter())
                .map(|(x, b)| x * b)
                .sum();
            let w = lp.exp();
            s0 += w;
            for a in 0..p {
                s1[a] += w * covariates[idx][a];
                for b in 0..p {
                    s2[a][b] += w * covariates[idx][a] * covariates[idx][b];
                }
            }
        }

        if s0 <= 0.0 {
            continue;
        }

        ll += sum_event_lp - d_k * s0.ln();
        for a in 0..p {
            grad[a] += sum_event_cov[a] - d_k * (s1[a] / s0);
            for b in 0..p {
                info[a][b] += d_k * (s2[a][b] / s0 - (s1[a] / s0) * (s1[b] / s0));
            }
        }
    }

    (ll, grad, info)
}

/// Solve `a * x = b` via Gauss-Jordan elimination with partial pivoting.
/// Returns `None` if `a` is singular (or not square, or dimensions mismatch).
fn solve_linear_system(a: &[Vec<f64>], b: &[f64]) -> Option<Vec<f64>> {
    let n = b.len();
    if a.len() != n || a.iter().any(|row| row.len() != n) {
        return None;
    }
    let mut m: Vec<Vec<f64>> = a
        .iter()
        .zip(b.iter())
        .map(|(row, &bi)| {
            let mut r = row.clone();
            r.push(bi);
            r
        })
        .collect();

    for col in 0..n {
        let (pivot_row, &max_val) = m
            .iter()
            .enumerate()
            .skip(col)
            .map(|(r, row)| (r, &row[col]))
            .max_by(|a, b| a.1.abs().partial_cmp(&b.1.abs()).unwrap())?;
        if max_val.abs() < 1e-12 {
            return None;
        }
        m.swap(col, pivot_row);
        let pivot = m[col][col];
        for v in m[col].iter_mut() {
            *v /= pivot;
        }
        let pivot_row_vals = m[col].clone();
        for (r, row) in m.iter_mut().enumerate() {
            if r == col {
                continue;
            }
            let factor = row[col];
            if factor != 0.0 {
                for (c, pv) in pivot_row_vals.iter().enumerate() {
                    row[c] -= factor * pv;
                }
            }
        }
    }
    Some(m.iter().map(|row| row[n]).collect())
}

/// Invert a square matrix via Gauss-Jordan elimination with partial pivoting.
/// Returns `None` if the matrix is singular (or not square).
fn invert_matrix(a: &[Vec<f64>]) -> Option<Vec<Vec<f64>>> {
    let n = a.len();
    if n == 0 || a.iter().any(|row| row.len() != n) {
        return None;
    }
    let mut m: Vec<Vec<f64>> = a
        .iter()
        .enumerate()
        .map(|(i, row)| {
            let mut r = row.clone();
            r.extend((0..n).map(|j| if i == j { 1.0 } else { 0.0 }));
            r
        })
        .collect();

    for col in 0..n {
        let (pivot_row, &max_val) = m
            .iter()
            .enumerate()
            .skip(col)
            .map(|(r, row)| (r, &row[col]))
            .max_by(|a, b| a.1.abs().partial_cmp(&b.1.abs()).unwrap())?;
        if max_val.abs() < 1e-12 {
            return None;
        }
        m.swap(col, pivot_row);
        let pivot = m[col][col];
        for v in m[col].iter_mut() {
            *v /= pivot;
        }
        let pivot_row_vals = m[col].clone();
        for (r, row) in m.iter_mut().enumerate() {
            if r == col {
                continue;
            }
            let factor = row[col];
            if factor != 0.0 {
                for (c, pv) in pivot_row_vals.iter().enumerate() {
                    row[c] -= factor * pv;
                }
            }
        }
    }
    Some(m.iter().map(|row| row[n..].to_vec()).collect())
}

/// Cox partial log-likelihood (Breslow tie handling) at a given coefficient
/// vector. Exposed on its own so callers can build likelihood-ratio
/// comparisons between nested models.
///
/// `covariates` is a row-major n x p matrix (one row per subject).
/// Returns `None` on empty/mismatched input or when there are no events.
pub fn cox_partial_log_likelihood(
    covariates: &[Vec<f64>],
    times: &[f64],
    events: &[bool],
    beta: &[f64],
) -> Option<f64> {
    let n = times.len();
    if n == 0 || covariates.len() != n || events.len() != n {
        return None;
    }
    let p = beta.len();
    if p == 0 || covariates.iter().any(|row| row.len() != p) {
        return None;
    }
    let (order, groups) = time_groups(times, events);
    if groups.is_empty() {
        return None;
    }
    let (ll, _grad, _info) = score_info(covariates, &order, &groups, beta, p);
    Some(ll)
}

/// Result of fitting a Cox proportional hazards model.
#[derive(Debug, Clone)]
pub struct CoxPhResult {
    pub coefficients: Vec<f64>,
    /// Standard errors from the inverse observed-information matrix at the fit.
    /// `NaN` for a coefficient if that matrix could not be inverted.
    pub std_errors: Vec<f64>,
    pub log_likelihood: f64,
    pub iterations: usize,
    pub converged: bool,
}

/// Fit a Cox proportional hazards model by Newton-Raphson on the Breslow
/// partial likelihood (ties are handled via Breslow's approximation).
///
/// `covariates` is a row-major n x p matrix (one row per subject, p >= 1
/// covariates); `times` are event/censoring times; `events` is true where an
/// event (not censoring) occurred. Returns `None` for empty/mismatched input
/// or when there are no events to fit against.
pub fn cox_ph(covariates: &[Vec<f64>], times: &[f64], events: &[bool]) -> Option<CoxPhResult> {
    let n = times.len();
    if n == 0 || covariates.len() != n || events.len() != n {
        return None;
    }
    let p = covariates[0].len();
    if p == 0 || covariates.iter().any(|row| row.len() != p) {
        return None;
    }
    let (order, groups) = time_groups(times, events);
    if groups.is_empty() {
        return None;
    }

    let max_iter = 50;
    let tol = 1e-8;
    let mut beta = vec![0.0f64; p];
    let mut iterations = 0;
    let mut converged = false;

    for iter in 0..max_iter {
        iterations = iter + 1;
        let (_ll, grad, info) = score_info(covariates, &order, &groups, &beta, p);
        let step = match solve_linear_system(&info, &grad) {
            Some(s) => s,
            None => break,
        };
        let mut max_change = 0.0f64;
        for (b, s) in beta.iter_mut().zip(step.iter()) {
            *b += s;
            max_change = max_change.max(s.abs());
        }
        if max_change < tol {
            converged = true;
            break;
        }
    }

    let (log_likelihood, _grad, info_final) = score_info(covariates, &order, &groups, &beta, p);
    let std_errors = match invert_matrix(&info_final) {
        Some(inv) => (0..p).map(|a| inv[a][a].max(0.0).sqrt()).collect(),
        None => vec![f64::NAN; p],
    };

    Some(CoxPhResult {
        coefficients: beta,
        std_errors,
        log_likelihood,
        iterations,
        converged,
    })
}

/// Result of a competing-risks cumulative incidence analysis.
#[derive(Debug, Clone)]
pub struct CompetingRisksResult {
    /// Distinct observation times, in ascending order.
    pub times: Vec<f64>,
    /// The distinct nonzero cause codes found in the input, in ascending order.
    pub causes: Vec<usize>,
    /// `cif[k][i]` is the cumulative incidence of `causes[k]` at `times[i]`.
    pub cif: Vec<Vec<f64>>,
    /// All-cause (overall) survival at `times[i]`.
    pub overall_survival: Vec<f64>,
}

/// Compute cause-specific cumulative incidence functions via the
/// Aalen-Johansen estimator.
///
/// `event_types`: 0 means censored; any other value identifies the cause of
/// the event. At every `times[i]`, `overall_survival[i] + sum_k cif[k][i]`
/// equals 1 (up to floating-point error), since every subject is always
/// either event-free or has experienced exactly one cause.
/// Returns `None` on empty/mismatched input or when there are no events.
pub fn competing_risks(times: &[f64], event_types: &[usize]) -> Option<CompetingRisksResult> {
    let n = times.len();
    if n == 0 || event_types.len() != n {
        return None;
    }

    let mut causes: Vec<usize> = event_types.iter().copied().filter(|&c| c != 0).collect();
    causes.sort_unstable();
    causes.dedup();
    if causes.is_empty() {
        return None;
    }

    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&a, &b| times[a].partial_cmp(&times[b]).unwrap());

    let mut uniq_times = Vec::new();
    let mut overall_survival = Vec::new();
    let mut cif: Vec<Vec<f64>> = vec![Vec::new(); causes.len()];

    let mut s_prev = 1.0f64;
    let mut cif_prev = vec![0.0f64; causes.len()];

    let mut i = 0;
    while i < n {
        let t = times[order[i]];
        let at_risk = (n - i) as f64; // n - i >= 1 since i < n
        let mut j = i;
        let mut d_cause = vec![0usize; causes.len()];
        while j < n && times[order[j]] == t {
            let et = event_types[order[j]];
            if et != 0 {
                if let Some(pos) = causes.iter().position(|&c| c == et) {
                    d_cause[pos] += 1;
                }
            }
            j += 1;
        }
        let d_total: usize = d_cause.iter().sum();

        let s_next = s_prev * (1.0 - (d_total as f64) / at_risk);
        for (c, &dc) in d_cause.iter().enumerate() {
            let cif_next = cif_prev[c] + s_prev * (dc as f64) / at_risk;
            cif[c].push(cif_next);
            cif_prev[c] = cif_next;
        }
        overall_survival.push(s_next);
        s_prev = s_next;
        uniq_times.push(t);
        i = j;
    }

    Some(CompetingRisksResult {
        times: uniq_times,
        causes,
        cif,
        overall_survival,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn km_survival_monotone() {
        let times = vec![1.0, 2.0, 2.0, 3.0, 5.0];
        let events = vec![true, false, true, true, false];
        let km = kaplan_meier(&times, &events);
        for w in km.survival_prob.windows(2) {
            assert!(w[1] <= w[0] + 1e-12);
        }
        assert_eq!(km.times.len(), km.survival_prob.len());
    }

    #[test]
    fn log_rank_basic_nonnegative() {
        // Simple synthetic datasets
        let t1 = vec![1.0, 2.0, 3.0, 4.0];
        let e1 = vec![true, true, false, true];
        let t2 = vec![1.5, 2.5, 3.5, 4.5];
        let e2 = vec![true, false, true, false];
        let stat = log_rank_test_stat(&t1, &e1, &t2, &e2);
        assert!(stat >= 0.0);
    }

    #[test]
    fn cox_ph_rejects_invalid_input() {
        assert!(cox_ph(&[], &[], &[]).is_none());
        assert!(cox_ph(&[vec![1.0]], &[1.0, 2.0], &[true, true]).is_none());
        assert!(cox_ph(&[vec![]], &[1.0], &[true]).is_none());
        // No events at all: nothing to fit against.
        assert!(cox_ph(&[vec![0.0], vec![1.0]], &[1.0, 2.0], &[false, false]).is_none());
    }

    #[test]
    fn cox_ph_recovers_expected_effect_direction() {
        // Subjects with x=1 mostly experience events earlier than x=0, with one
        // overlap (x=1 at t=9 comes after several x=0 events) so there is no
        // perfect separation and Newton-Raphson converges to a finite beta.
        let covariates: Vec<Vec<f64>> = vec![
            vec![1.0],
            vec![1.0],
            vec![1.0],
            vec![1.0],
            vec![0.0],
            vec![0.0],
            vec![0.0],
            vec![0.0],
            vec![1.0],
            vec![0.0],
        ];
        let times = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let events = vec![true; 10];

        let fit = cox_ph(&covariates, &times, &events).expect("should fit");
        assert!(fit.converged);
        assert_eq!(fit.coefficients.len(), 1);
        // x=1 correlates with earlier events => higher hazard => positive coefficient.
        assert!(
            fit.coefficients[0] > 0.0,
            "expected positive coefficient, got {}",
            fit.coefficients[0]
        );
        assert!(fit.std_errors[0].is_finite() && fit.std_errors[0] > 0.0);
    }

    #[test]
    fn cox_ph_fit_is_a_stationary_point_of_the_partial_likelihood() {
        // The gradient of `cox_partial_log_likelihood` (checked via finite
        // differences, an API independent of cox_ph's internal Newton step)
        // must vanish at the coefficients cox_ph reports as the fit.
        let covariates: Vec<Vec<f64>> = vec![
            vec![1.0, 0.2],
            vec![0.0, 1.0],
            vec![1.0, 0.5],
            vec![0.0, 0.1],
            vec![1.0, 0.8],
            vec![0.0, 0.4],
            vec![1.0, 0.6],
            vec![0.0, 0.9],
        ];
        let times = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let events = vec![true, true, true, false, true, true, false, true];

        let fit = cox_ph(&covariates, &times, &events).expect("should fit");
        assert!(fit.converged);

        let eps = 1e-4;
        for k in 0..fit.coefficients.len() {
            let mut plus = fit.coefficients.clone();
            plus[k] += eps;
            let mut minus = fit.coefficients.clone();
            minus[k] -= eps;
            let ll_plus = cox_partial_log_likelihood(&covariates, &times, &events, &plus).unwrap();
            let ll_minus =
                cox_partial_log_likelihood(&covariates, &times, &events, &minus).unwrap();
            let numerical_grad = (ll_plus - ll_minus) / (2.0 * eps);
            assert!(
                numerical_grad.abs() < 1e-3,
                "gradient for coefficient {k} not near zero: {numerical_grad}"
            );
        }
    }

    #[test]
    fn cox_ph_handles_tied_event_times() {
        let covariates: Vec<Vec<f64>> = vec![vec![0.0], vec![1.0], vec![1.0], vec![0.0]];
        let times = vec![1.0, 1.0, 2.0, 2.0];
        let events = vec![true, true, true, false];
        let fit = cox_ph(&covariates, &times, &events).expect("should fit with ties");
        assert!(fit.iterations > 0);
        assert!(fit.log_likelihood.is_finite());
    }

    #[test]
    fn competing_risks_rejects_invalid_input() {
        assert!(competing_risks(&[], &[]).is_none());
        assert!(competing_risks(&[1.0, 2.0], &[1]).is_none());
        // All censored: no cause to compute a CIF for.
        assert!(competing_risks(&[1.0, 2.0], &[0, 0]).is_none());
    }

    #[test]
    fn competing_risks_matches_hand_computed_aalen_johansen() {
        // cause 1 at t=1, cause 2 at t=2, censored at t=3, cause 1 at t=4.
        let times = vec![1.0, 2.0, 3.0, 4.0];
        let event_types = vec![1usize, 2, 0, 1];
        let res = competing_risks(&times, &event_types).expect("should compute CIF");

        assert_eq!(res.causes, vec![1, 2]);
        assert_eq!(res.times, vec![1.0, 2.0, 3.0, 4.0]);

        let expected_surv = [0.75, 0.5, 0.5, 0.0];
        let expected_cif1 = [0.25, 0.25, 0.25, 0.75];
        let expected_cif2 = [0.0, 0.25, 0.25, 0.25];
        for i in 0..4 {
            assert!((res.overall_survival[i] - expected_surv[i]).abs() < 1e-9);
            assert!((res.cif[0][i] - expected_cif1[i]).abs() < 1e-9);
            assert!((res.cif[1][i] - expected_cif2[i]).abs() < 1e-9);
        }
    }

    #[test]
    fn competing_risks_probabilities_partition_unity_and_are_monotone() {
        let times = vec![1.0, 2.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let event_types = vec![1usize, 2, 0, 1, 3, 0, 2];
        let res = competing_risks(&times, &event_types).expect("should compute CIF");

        for i in 0..res.times.len() {
            let sum: f64 = res.overall_survival[i] + res.cif.iter().map(|c| c[i]).sum::<f64>();
            assert!(
                (sum - 1.0).abs() < 1e-9,
                "identity broken at index {i}: {sum}"
            );
            assert!((0.0..=1.0).contains(&res.overall_survival[i]));
            for c in &res.cif {
                assert!((0.0..=1.0).contains(&c[i]));
            }
        }
        for c in &res.cif {
            for w in c.windows(2) {
                assert!(w[1] + 1e-12 >= w[0], "CIF must be non-decreasing");
            }
        }
    }
}
