use crate::flcq::com;

mod pause;

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

pub fn swap_c(c1: f64, c2: f64) -> bool {
    swap_f(c2, c1)
}

pub fn frequency_pack(
    port: &mut com::Flcq,
    f1_2: std::string::String,
) -> std::option::Option<(f64, f64, f64)> {
    let s = format!("Measure frequency {} (please whait a few seconds):", f1_2);
    let mut animation = super::animation::Animation::new(s);
    let mut res = None;
    if let Some((t, f)) = port.frequency() {
        let temp = port.temperature();

        if let (Some(current_temperature), _) = temp {
            let pack = (f, t, current_temperature);

            res = Some(pack);
        }
    }
    animation.end();
    res
}

pub fn f1f2(
    port: &mut com::Flcq,
) -> (
    std::option::Option<(f64, f64, f64)>,
    std::option::Option<(f64, f64, f64)>,
) {
    pause::pause();
    let f1 = frequency_pack(port, std::string::String::from("F1"));
    pause::pause();

    let f2 = frequency_pack(port, std::string::String::from("F2"));
    (f1, f2)
}

pub fn f0f1f2(
    port: &mut com::Flcq,
) -> (
    std::option::Option<(f64, f64, f64)>,
    std::option::Option<(f64, f64, f64)>,
    std::option::Option<(f64, f64, f64)>,
) {
    pause::pause();
    let f0 = frequency_pack(port, std::string::String::from("F0"));

    pause::pause();
    let f1 = frequency_pack(port, std::string::String::from("F1"));

    pause::pause();

    let f2 = frequency_pack(port, std::string::String::from("F2"));
    (f0, f1, f2)
}

// f1 < f2
pub fn sorted2(port: &mut com::Flcq) -> (f64, f64) {
    let (f1, f2) = f1f2(port);
    swap_ff(f1, f2)
}

// f0 < f1 < f2
pub fn sorted3(port: &mut com::Flcq) -> (f64, f64) {
    let (f0, f1, f2) = f0f1f2(port);
    swap_fff(f0, f1, f2)
}
