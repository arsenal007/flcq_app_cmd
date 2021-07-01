pub fn calc_l(f1: f64, f2: f64, c1: f64, c2: f64) -> (f64, f64) {
    let f1_2 = f1 * f1;
    let f2_2 = f2 * f2;

    let c1f = c1 / 1000_000_000.0; // in farad
    let c2f = c2 / 1000_000_000.0;

    let c = (f1_2 * c1 - f2_2 * c2) / (f2_2 - f1_2);

    let l = (1.0 / f1_2 - 1.0 / f2_2)
        / (4.0 * std::f64::consts::PI * std::f64::consts::PI * (c1f - c2f)); // in Henry
    (c, l * 1000_000_000.0) // return in pico farads and micro Henrys
}

pub fn frequency(prescaler: u8, tmr0: u8, overflows_array: [u8; 4]) -> f64 {
    let overflows: u32;
    unsafe {
        overflows = std::mem::transmute::<[u8; 4], u32>(overflows_array);
    }
    let prescaler_values = [1.0f64, 2.0f64, 4.0f64, 8.0f64, 16.0f64];
    println!(
        "Overflows: {}, PSC: {},  Count: {}",
        overflows,
        prescaler_values[(prescaler + 1u8) as usize],
        tmr0 as f64
    );
    prescaler_values[(prescaler + 1u8) as usize] * (256.0f64 * overflows as f64 + tmr0 as f64)
}
