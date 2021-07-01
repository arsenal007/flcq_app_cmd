#[path = "lc.rs"]
pub mod lc;

pub use self::lc::freq::com;

pub fn frequency(port: &std::string::String) -> () {
    let mut flcq: com::Flcq = com::open(port, false);
    if let Some((f, ct, t)) = lc::freq::frequency_pack(&mut flcq) {
        println!("Frequency: {0:.2} Hz, Calibration Temperature: {1:.2} C, Current Temperature: {2:.2} C, temperature difference: {3:.2} C", f,ct,t,ct-t);
    }
}

pub fn capacitance(port: &std::string::String) -> () {
    let mut flcq: com::Flcq = com::open(port, false);

    if let Some((c, _)) = lc::measurments(&mut flcq) {
        let (c0, _) = flcq.c0_l0();
        let cx = c - c0;
        println!("Measured capacitance {:.2} pF", cx);
    };
}

pub fn inductance(port: &std::string::String) -> () {
    let mut flcq: com::Flcq = com::open(port, false);
    if let Some((c, l)) = lc::measurments(&mut flcq) {
        println!("Measured  inductance {:.4} uH, capacitance {:.2} pF", l, c);
    };
}
