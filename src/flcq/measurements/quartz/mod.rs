use crate::flcq::{capacity, com, freq};

mod calculation;

//use self::lc::freq::com;

pub fn measurments(
    port: &std::string::String,
    measure_cq: bool,
) -> std::option::Option<(f64, f64, f64, f64)> {
    let mut flcq = com::open(port, false);

    let (cq_eeprom, cs, c1, c2) = flcq.quartz_crefs();
    let mut cq = cq_eeprom;

    if measure_cq {
        if let Some(cq_measured) = capacity::raw(&mut flcq) {
            println!("Measured capacity  Cq {:.2} pF", cq);
            cq = cq_measured;
        }
    }

    let (f0, f1, f2) = freq::sorted3(&mut flcq);
    Some(calculation::crystal_quartz(f0, f1, f2, c1, c2))
}
