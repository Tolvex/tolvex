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
/// `days` is non-positive.
pub fn sir_model(
    s0: f64,
    i0: f64,
    r0: f64,
    beta: f64,
    gamma: f64,
    days: f64,
    dt: f64,
) -> SirTrajectory {
    let mut traj = SirTrajectory {
        t: vec![0.0],
        s: vec![s0],
        i: vec![i0],
        r: vec![r0],
    };
    if dt <= 0.0 || days <= 0.0 {
        return traj;
    }
    let steps = (days / dt).round().max(0.0) as usize;
    let (mut s, mut i, mut r) = (s0, i0, r0);
    let mut t = 0.0;
    for _ in 0..steps {
        let ds = -beta * s * i;
        let di = beta * s * i - gamma * i;
        let dr = gamma * i;
        s += ds * dt;
        i += di * dt;
        r += dr * dt;
        t += dt;
        traj.t.push(t);
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
/// if `dt` or `days` is non-positive.
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
    let mut traj = SeirTrajectory {
        t: vec![0.0],
        s: vec![s0],
        e: vec![e0],
        i: vec![i0],
        r: vec![r0],
    };
    if dt <= 0.0 || days <= 0.0 {
        return traj;
    }
    let steps = (days / dt).round().max(0.0) as usize;
    let (mut s, mut e, mut i, mut r) = (s0, e0, i0, r0);
    let mut t = 0.0;
    for _ in 0..steps {
        let ds = -beta * s * i;
        let de = beta * s * i - sigma * e;
        let di = sigma * e - gamma * i;
        let dr = gamma * i;
        s += ds * dt;
        e += de * dt;
        i += di * dt;
        r += dr * dt;
        t += dt;
        traj.t.push(t);
        traj.s.push(s);
        traj.e.push(e);
        traj.i.push(i);
        traj.r.push(r);
    }
    traj
}
