#[path = "freq.rs"]
pub mod freq;

fn swap_f(f1: f64, f2: f64) -> bool {
    let r;
    if f1 < f2 {
        r = false;
    } else {
        r = true;
    }
    r
}

fn swap_ff(
    f1: std::option::Option<(f64, f64, f64)>,
    f2: std::option::Option<(f64, f64, f64)>,
) -> (f64, f64) {
    if let (Some(mut f1), Some(mut f2)) = (f1, f2) {
        let (f1_v, _, _) = f1.clone();
        let (f2_v, _, _) = f2.clone();

        let need_swap_f1f2 = swap_f(f1_v, f2_v);
        if need_swap_f1f2 {
            let a = f2.clone();
            let b = f1.clone();
            f1 = a;
            f2 = b;
        }

        let (f1, ct1, st1) = f1;
        println!("F1: {0:.2} Hz, Calibration Temperature: {1:.2} C, Current Temperature: {2:.2} C, temperature difference: {3:.2} C", f1,ct1,st1,ct1-st1);
        let (f2, ct2, st2) = f2;
        println!("F2: {0:.2} Hz, Calibration Temperature: {1:.2} C, Current Temperature: {2:.2} C, temperature difference: {3:.2} C", f2,ct2,st2,ct2-st2);
        (f1, f2)
    } else {
        unreachable!();
    }
}

fn swap_c(c1: f64, c2: f64) -> bool {
    swap_f(c2, c1)
}

pub use self::freq::com::Flcq;

fn intern_lc(f1: f64, f2: f64, mut c1: f64, mut c2: f64, show: bool) -> (f64, f64) {
    if swap_c(c1, c2) {
        let a = c2;
        let b = c1;
        c1 = a;
        c2 = b;
    }

    println!("Referance:  C1 {0:.2} pF, C2 {1:.2} pF", c1, c2);
    /*let (f1, ct1, st1) = f1;
    println!("F1: {0:.2} Hz, Calibration Temperature: {1:.2} C, Current Temperature: {2:.2} C, temperature difference: {3:.2} C", f1,ct1,st1,ct1-st1);
    let (f2, ct2, st2) = f2;
    println!("F2: {0:.2} Hz, Calibration Temperature: {1:.2} C, Current Temperature: {2:.2} C, temperature difference: {3:.2} C", f2,ct2,st2,ct2-st2);
    */

    let (c, l) = {
        let f1_2 = f1 * f1;
        let f2_2 = f2 * f2;

        let c1f = c1 / 1000_000_000.0; // in farad
        let c2f = c2 / 1000_000_000.0;

        let c = (f1_2 * c1 - f2_2 * c2) / (f2_2 - f1_2);

        let l = (1.0 / f1_2 - 1.0 / f2_2)
            / (4.0 * std::f64::consts::PI * std::f64::consts::PI * (c1f - c2f)); // in Henry

        (c, l * 1000_000_000.0) // return in pico farads and micro Henrys
    };
    if show {
        println!(
            "Inductance L: {:.2} µH, Parasitic capacitance {:.2} pF",
            l, c
        );
    }
    (c, l)
}

pub fn calibration(
    port: &mut Flcq,
    c1: &std::option::Option<u16>,
    c2: &std::option::Option<u16>,
) -> std::option::Option<(f64, f64)> {
    let (f1, f2) = freq::f1f2(port);

    let (f1, f2) = swap_ff(f1, f2);

    if let (Some(c1), Some(c2)) = (c1, c2) {
        let c1 = *c1 as f64;
        let c2 = *c2 as f64;
        let (c0, l0) = intern_lc(f1, f2, c1, c2, true);
        Some((c0, l0))
    } else {
        unreachable!();
    }
}

pub fn measurments(port: &mut Flcq) -> std::option::Option<(f64, f64)> {
    let (f1, f2) = freq::f1f2(port);
    let (f1, f2) = swap_ff(f1, f2);
    let (c0, l0) = intern_lc(f1, f2, port.cref1(), port.cref2(), false);
    Some((c0, l0))
}
