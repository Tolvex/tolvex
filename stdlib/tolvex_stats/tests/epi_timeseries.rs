use tolvex_stats::{
    exponential_moving_average, incidence_rate, moving_average, npv, ppv, prevalence, rolling_std,
    seir_model, sensitivity, sir_model, specificity,
};

#[test]
fn epi_measures() {
    assert!((incidence_rate(10.0, 1000.0) - 0.01).abs() < 1e-12);
    assert!((prevalence(50.0, 1000.0) - 0.05).abs() < 1e-12);
    assert!((sensitivity(80.0, 20.0) - 0.8).abs() < 1e-12);
    assert!((specificity(90.0, 10.0) - 0.9).abs() < 1e-12);
    assert!((ppv(80.0, 20.0) - 0.8).abs() < 1e-12);
    assert!((npv(90.0, 10.0) - 0.9).abs() < 1e-12);
}

#[test]
fn ts_moving_and_ema() {
    let xs = [1.0, 2.0, 3.0, 4.0];
    let ma = moving_average(&xs, 2);
    assert_eq!(ma.len(), 3);
    assert!((ma[0] - 1.5).abs() < 1e-12);
    assert!((ma[1] - 2.5).abs() < 1e-12);
    assert!((ma[2] - 3.5).abs() < 1e-12);

    let ema = exponential_moving_average(&xs, 0.5);
    assert_eq!(ema.len(), 4);
    // simple progression sanity checks
    assert!(ema[1] > ema[0]);
    assert!(ema[2] > ema[1]);
}

#[test]
fn sir_conserves_population_and_epidemic_rises_then_falls() {
    let traj = sir_model(0.99, 0.01, 0.0, 0.5, 0.1, 100.0, 0.1);
    assert_eq!(traj.t.len(), traj.s.len());
    assert_eq!(traj.t.len(), traj.i.len());
    assert_eq!(traj.t.len(), traj.r.len());

    // S + I + R should stay ~1.0 throughout (no births/deaths).
    for k in 0..traj.t.len() {
        let total = traj.s[k] + traj.i[k] + traj.r[k];
        assert!((total - 1.0).abs() < 1e-6, "total={total} at step {k}");
    }

    // With R0 = beta/gamma = 5 > 1, infections should rise above the
    // initial level before eventually declining as susceptibles deplete.
    let peak_i = traj.i.iter().cloned().fold(f64::MIN, f64::max);
    assert!(peak_i > 0.01);
    assert!(*traj.i.last().unwrap() < peak_i);
}

#[test]
fn sir_non_positive_horizon_returns_only_initial_point() {
    let traj = sir_model(0.99, 0.01, 0.0, 0.5, 0.1, 0.0, 0.1);
    assert_eq!(traj.t, vec![0.0]);
    assert_eq!(traj.s, vec![0.99]);
    assert_eq!(traj.i, vec![0.01]);
    assert_eq!(traj.r, vec![0.0]);
}

#[test]
fn seir_conserves_population_and_epidemic_rises_then_falls() {
    let traj = seir_model(0.99, 0.0, 0.01, 0.0, 0.5, 0.2, 0.1, 150.0, 0.1);
    assert_eq!(traj.t.len(), traj.s.len());
    assert_eq!(traj.t.len(), traj.e.len());
    assert_eq!(traj.t.len(), traj.i.len());
    assert_eq!(traj.t.len(), traj.r.len());

    for k in 0..traj.t.len() {
        let total = traj.s[k] + traj.e[k] + traj.i[k] + traj.r[k];
        assert!((total - 1.0).abs() < 1e-6, "total={total} at step {k}");
    }

    let peak_i = traj.i.iter().cloned().fold(f64::MIN, f64::max);
    assert!(peak_i > 0.01);
    assert!(*traj.i.last().unwrap() < peak_i);
}

#[test]
fn ts_rolling_std() {
    let xs = [1.0, 2.0, 3.0, 4.0];
    let rs = rolling_std(&xs, 2);
    assert_eq!(rs.len(), 3);
    // stddev of pairs: [1,2],[2,3],[3,4] is sqrt(0.5)
    for v in rs {
        assert!((v - std::f64::consts::FRAC_1_SQRT_2).abs() < 1e-12);
    }
}
