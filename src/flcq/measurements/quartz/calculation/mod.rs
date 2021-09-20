use crate::flcq::converters::F;

pub fn crystal_quartz2(
    f1: f64, // shorted SW
    f2: f64, // opened with c_switch
    c_quartz: f64,
    c_switch: f64,
    c1: f64,
    c2: f64,
) -> (f64, f64, f64) {
    // convert picofarad to farad
    let (c1, c2, c_quartz, c_switch) = (
        c1.picofarad_to_farad(),
        c2.picofarad_to_farad(),
        c_quartz.picofarad_to_farad(),
        c_switch.picofarad_to_farad(),
    ); // convert to fahrad

    let c_a = 1.0_f64 / (1.0_f64 / c1 + 1.0_f64 / c2) + c_quartz;
    let c_b = 1.0_f64 / (1.0_f64 / c_switch + 1.0_f64 / c1 + 1.0_f64 / c2) + c_quartz;

    let c_d = c_a * c_b * (f2.powi(2) - f1.powi(2)) / (c_a * f1.powi(2) - c_b * f2.powi(2)); // Cd in Fahrad [F]
    let c = c_a * c_d / (c_a + c_d);

    let l_d = 1.0_f64 / ({ 2.0_f64 * std::f64::consts::PI * f1 }.powi(2) * c); // Ld in Henry [H]
    let f_o = 1.0_f64 / (2.0_f64 * std::f64::consts::PI * { l_d * c_d }.sqrt()); // in Herz [Hz]
    (
        f_o * 10.0_f64.powi(-3), // Hz
        l_d * 10.0_f64.powi(3),  // mH
        c_d * 10.0_f64.powi(15), // fF
    ) // return in kHz, mH and fF
}

pub fn crystal_quartz(
    f0: f64,
    f1: f64, // witch c_switch1
    f2: f64, // witch c_switch2
    c1: f64,
    c2: f64,
) -> (f64, f64, f64, f64) {
    let c_m = (c1 - c2) / (f0 / (2.0_f64 * (f1 - f0)) - f0 / (2.0_f64 * (f2 - f0)));
    let l_m = 1.0_f64 / (c_m * { 2.0_f64 * std::f64::consts::PI * f0 }.powi(2));
    let f_s = 1.0_f64 / (2.0_f64 * std::f64::consts::PI * { l_m * c_m }.sqrt());
    // in Herz [Hz]

    (
        f_s * 10.0_f64.powi(-3), // Hz
        l_m * 10.0_f64.powi(3),  // mH
        c_m * 10.0_f64.powi(15), // fF
    )
}
