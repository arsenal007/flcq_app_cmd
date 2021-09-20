mod calculation {

    use crate::flcq::{converters::F, freq};

    pub fn intern_lc(f1: f64, f2: f64, mut c1: f64, mut c2: f64, show: bool) -> (f64, f64) {
        if freq::swap_c(c1, c2) {
            let a = c2;
            let b = c1;
            c1 = a;
            c2 = b;
        }

        println!("Referance:  C1 {0:.2} pF, C2 {1:.2} pF", c1, c2);

        let (c, l) = {
            let c = (f1.powi(2) * c1 - f2.powi(2) * c2) / (f2.powi(2) - f1.powi(2));

            let l = (1.0 / f1.powi(2) - 1.0 / f2.powi(2))
                / (4.0
                    * std::f64::consts::PI
                    * std::f64::consts::PI
                    * (c1.picofarad_to_farad() - c2.picofarad_to_farad())); // in Henry

            (c, l.henry_to_micro_henry()) // return in pico farads and micro Henrys
        };

        if show {
            println!("induction L: {:.2} µH, Parasitic capacity {:.2} pF", l, c);
        }
        (c, l)
    }
}

use crate::flcq::{com, freq};

pub fn calibration(
    port: &mut com::Flcq,
    c1: &std::option::Option<u16>,
    c2: &std::option::Option<u16>,
) -> std::option::Option<(f64, f64)> {
    // get f1 < f2
    let (f1, f2): (f64, f64) = freq::sorted2(port);

    if let (Some(c1), Some(c2)) = (c1, c2) {
        let c1 = *c1 as f64;
        let c2 = *c2 as f64;
        let (c0, l0) = calculation::intern_lc(f1, f2, c1, c2, true);
        Some((c0, l0))
    } else {
        unreachable!();
    }
}

pub fn capacity_calibration(
    port: std::string::String,
    c1: std::option::Option<u16>,
    c2: std::option::Option<u16>,
) -> () {
    let mut flcq = com::open(&port, true);

    if let Some((c0, _l0)) = calibration(&mut flcq, &c1, &c2) {
        println!("Attach Cx and repeat measure sequence");

        if let Some((c, _l)) = calibration(&mut flcq, &c1, &c2) {
            let cx = c - c0;
            println!("Measured capacity {:.2} pF", cx);
            flcq.set_lc_crefs(cx);
        }
    }
}

pub fn induction_calibration(
    port: std::string::String,
    c1: std::option::Option<u16>,
    c2: std::option::Option<u16>,
) -> () {
    let mut flcq = com::open(&port, true);
    if let Some((c0, l0)) = calibration(&mut flcq, &c1, &c2) {
        flcq.set_c0_l0(c0, l0);
    } else {
        unimplemented!("induction calibration")
    }
}

pub fn measurments(port: &mut com::Flcq) -> std::option::Option<(f64, f64)> {
    let (f1, f2) = freq::sorted2(port);
    let (c0, l0) = calculation::intern_lc(f1, f2, port.cref1(), port.cref2(), false);
    Some((c0, l0))
}
