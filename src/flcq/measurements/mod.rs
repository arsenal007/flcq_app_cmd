use crate::flcq::{capacity as c, com, freq, lc};

pub fn frequency(port: &std::string::String) -> () {
    let mut flcq = com::open(port, false);
    if let Some((f, ct, t)) = freq::frequency_pack(&mut flcq, std::string::String::from("F")) {
        println!("Frequency: {0:.2} Hz, Calibration Temperature: {1:.2} C, Current Temperature: {2:.2} C, temperature difference: {3:.2} C", f,ct,t,ct-t);
    }
}

pub fn capacity(port: &std::string::String) -> std::option::Option<f64> {
    let mut flcq = com::open(port, false);
    if let Some(cx) = c::raw(&mut flcq) {
        println!("Measured capacity {:.2} pF", cx);
        return Some(cx);
    };
    None
}

pub fn induction(port: &std::string::String) -> () {
    let mut flcq = com::open(port, false);
    if let Some((c, l)) = lc::measurments(&mut flcq) {
        println!("Measured  induction {:.4} uH, capacity {:.2} pF", l, c);
    };
}

mod quartz;

pub fn quartz(port: &std::string::String, measure_cq: bool) -> () {
    if let Some((f, l, c)) = quartz::measurments(port, measure_cq) {
        println!(
            "Measured crystal quartz frequency [Fs]: {:.5} kHz, induction [Ls]: {:.4} mH, capacity [Cs]: {:.4} pF",
            f, l, c
        );
    };
}
