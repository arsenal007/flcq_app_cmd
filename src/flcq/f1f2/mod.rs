/*fn swap_f(f1: f64, f2: f64) -> bool {
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

pub fn swap_c(c1: f64, c2: f64) -> bool {
    swap_f(c2, c1)
}

use crate::flcq::com;
use crate::flcq::freq;

// f1 < f2
fn sorted(port: &mut com::Flcq) -> (f64, f64) {
    let (f1, f2) = freq::f1f2(port);
    swap_ff(f1, f2)
}*/
