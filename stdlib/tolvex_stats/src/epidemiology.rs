pub fn incidence_rate(new_cases: f64, person_time: f64) -> f64 {
    if person_time == 0.0 {
        f64::NAN
    } else {
        new_cases / person_time
    }
}

pub fn prevalence(existing_cases: f64, population: f64) -> f64 {
    if population == 0.0 {
        f64::NAN
    } else {
        existing_cases / population
    }
}

pub fn sensitivity(tp: f64, fn_: f64) -> f64 {
    let denom = tp + fn_;
    if denom == 0.0 {
        f64::NAN
    } else {
        tp / denom
    }
}

pub fn specificity(tn: f64, fp: f64) -> f64 {
    let denom = tn + fp;
    if denom == 0.0 {
        f64::NAN
    } else {
        tn / denom
    }
}

pub fn ppv(tp: f64, fp: f64) -> f64 {
    let denom = tp + fp;
    if denom == 0.0 {
        f64::NAN
    } else {
        tp / denom
    }
}

pub fn npv(tn: f64, fn_: f64) -> f64 {
    let denom = tn + fn_;
    if denom == 0.0 {
        f64::NAN
    } else {
        tn / denom
    }
}

pub fn lr_positive(sens: f64, spec: f64) -> f64 {
    if 1.0 - spec == 0.0 {
        f64::INFINITY
    } else {
        sens / (1.0 - spec)
    }
}

pub fn lr_negative(sens: f64, spec: f64) -> f64 {
    if sens == 0.0 {
        f64::INFINITY
    } else {
        (1.0 - spec) / sens
    }
}

/// Upper bound on the number of forward-Euler steps a compartmental model
/// will simulate, regardless of the requested `days`/`dt`. Protects against
/// unbounded memory growth (or an effectively-infinite loop) from an
/// oversized horizon or an overly fine step size; the trajectory is
/// truncated at this many steps rather than erroring, since `days`/`dt` are
/// plain `f64`s with no natural validity bound of their own.
const MAX_EPIDEMIC_STEPS: usize = 10_000_000;

/// Number of forward-Euler steps to take for a `days`/`dt` horizon, clamped
/// to `MAX_EPIDEMIC_STEPS`. Returns `0` if `dt` or `days` is non-positive.
fn euler_step_count(days: f64, dt: f64) -> usize {
    if dt <= 0.0 || days <= 0.0 {
        0
    } else {
        ((days / dt).round().max(0.0) as usize).min(MAX_EPIDEMIC_STEPS)
    }
}

/// Integrate `derivs` forward from `state0` for `steps` steps of size `dt`
/// using the forward Euler method, returning the time points and the state
/// at each point (including the initial state at `t = 0`).
fn euler_integrate<const N: usize>(
    state0: [f64; N],
    steps: usize,
    dt: f64,
    derivs: impl Fn([f64; N]) -> [f64; N],
) -> (Vec<f64>, Vec<[f64; N]>) {
    let mut t_series = Vec::with_capacity(steps + 1);
    let mut state_series = Vec::with_capacity(steps + 1);
    let mut state = state0;
    let mut t = 0.0;
    t_series.push(t);
    state_series.push(state);
    for _ in 0..steps {
        let d = derivs(state);
        for k in 0..N {
            state[k] += d[k] * dt;
        }
        t += dt;
        t_series.push(t);
        state_series.push(state);
    }
    (t_series, state_series)
}

/// Time series produced by [`sir_model`]: susceptible, infected, and
/// recovered compartments (as fractions or counts, matching the inputs) at
/// each simulated time point.
#[derive(Debug, Clone)]
pub struct SirTrajectory {
    pub t: Vec<f64>,
    pub s: Vec<f64>,
    pub i: Vec<f64>,
    pub r: Vec<f64>,
}

/// Simulate the SIR (Susceptible-Infected-Recovered) compartmental epidemic
/// model via forward Euler integration.
///
/// `s0`/`i0`/`r0` are the initial compartment sizes (fractions summing to 1,
/// or absolute counts); `beta` is the transmission rate, `gamma` the
/// recovery rate, `days` the simulation horizon, and `dt` the integration
/// step. Returns a trajectory containing only the initial point if `dt` or
/// `days` is non-positive; the trajectory is truncated at
/// [`MAX_EPIDEMIC_STEPS`] steps if `days / dt` would otherwise exceed it.
pub fn sir_model(
    s0: f64,
    i0: f64,
    r0: f64,
    beta: f64,
    gamma: f64,
    days: f64,
    dt: f64,
) -> SirTrajectory {
    let steps = euler_step_count(days, dt);
    let (t, states) = euler_integrate([s0, i0, r0], steps, dt, |[s, i, _r]| {
        [-beta * s * i, beta * s * i - gamma * i, gamma * i]
    });
    let mut traj = SirTrajectory {
        t,
        s: Vec::with_capacity(states.len()),
        i: Vec::with_capacity(states.len()),
        r: Vec::with_capacity(states.len()),
    };
    for [s, i, r] in states {
        traj.s.push(s);
        traj.i.push(i);
        traj.r.push(r);
    }
    traj
}

/// Time series produced by [`seir_model`]: susceptible, exposed, infected,
/// and recovered compartments at each simulated time point.
#[derive(Debug, Clone)]
pub struct SeirTrajectory {
    pub t: Vec<f64>,
    pub s: Vec<f64>,
    pub e: Vec<f64>,
    pub i: Vec<f64>,
    pub r: Vec<f64>,
}

/// Simulate the SEIR (Susceptible-Exposed-Infected-Recovered) compartmental
/// epidemic model via forward Euler integration.
///
/// `s0`/`e0`/`i0`/`r0` are the initial compartment sizes; `beta` is the
/// transmission rate, `sigma` the incubation rate (1/latent period), `gamma`
/// the recovery rate, `days` the simulation horizon, and `dt` the
/// integration step. Returns a trajectory containing only the initial point
/// if `dt` or `days` is non-positive; the trajectory is truncated at
/// [`MAX_EPIDEMIC_STEPS`] steps if `days / dt` would otherwise exceed it.
#[allow(clippy::too_many_arguments)]
pub fn seir_model(
    s0: f64,
    e0: f64,
    i0: f64,
    r0: f64,
    beta: f64,
    sigma: f64,
    gamma: f64,
    days: f64,
    dt: f64,
) -> SeirTrajectory {
    let steps = euler_step_count(days, dt);
    let (t, states) = euler_integrate([s0, e0, i0, r0], steps, dt, |[s, e, i, _r]| {
        [
            -beta * s * i,
            beta * s * i - sigma * e,
            sigma * e - gamma * i,
            gamma * i,
        ]
    });
    let mut traj = SeirTrajectory {
        t,
        s: Vec::with_capacity(states.len()),
        e: Vec::with_capacity(states.len()),
        i: Vec::with_capacity(states.len()),
        r: Vec::with_capacity(states.len()),
    };
    for [s, e, i, r] in states {
        traj.s.push(s);
        traj.e.push(e);
        traj.i.push(i);
        traj.r.push(r);
    }
    traj
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn euler_step_count_clamps_to_max_epidemic_steps() {
        // days/dt is astronomically larger than MAX_EPIDEMIC_STEPS; the
        // count must be clamped rather than overflowing/hanging downstream
        // Vec growth in sir_model/seir_model.
        assert_eq!(euler_step_count(1e12, 1e-6), MAX_EPIDEMIC_STEPS);
    }

    #[test]
    fn euler_step_count_non_positive_inputs_are_zero() {
        assert_eq!(euler_step_count(0.0, 0.1), 0);
        assert_eq!(euler_step_count(10.0, 0.0), 0);
        assert_eq!(euler_step_count(-1.0, 0.1), 0);
    }
}
