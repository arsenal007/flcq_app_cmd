#[path = "com.rs"]
pub mod com;

#[path = "pause.rs"]
mod pause;

pub fn frequency(port: &mut com::Flcq) -> () {
    if let Some((f, ct, t)) = frequency_pack(port) {
        println!("Frequency: {0:.2} Hz, Calibration Temperature: {1:.2} C, Current Temperature: {2:.2} C, temperature difference: {3:.2} C", f,ct,t,ct-t);
    }
}

pub fn frequency_pack(port: &mut com::Flcq) -> std::option::Option<(f64, f64, f64)> {
    if port.is_init() {
        if let Some((t, f)) = port.frequency() {
            println!("temperature");
            let temp = port.temperature();

            if let (Some(current_temperature), _) = temp {
                let pack = (f, t, current_temperature);
                return Some(pack);
            }
        }
    }
    None
}

pub use self::com::Flcq;

pub fn f1f2(
    port: &mut Flcq,
) -> (
    std::option::Option<(f64, f64, f64)>,
    std::option::Option<(f64, f64, f64)>,
) {
    pause::pause();
    println!("measure F1...");
    let f1 = frequency_pack(port);
    pause::pause();
    println!("measure F2...");
    let f2 = frequency_pack(port);
    (f1, f2)
}
