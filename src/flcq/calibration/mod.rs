use crate::flcq::{capacity, com, lc};

fn frequency(port: &std::string::String, count: u8, ref_freq: f64) -> () {
    let mut flcq = com::open(port, true);

    if let (Some((_, f)), _) = flcq.frequency_raw(&count) {
        let periode = f / ref_freq;
        println!("Periode {:.?} [Sec]", periode);
        flcq.set_frequency(periode, count);
    }
}

fn quartz_calibration_cs(port: &std::string::String) {
    let mut flcq = com::open(port, false);
    if let Some(cx) = capacity::raw(&mut flcq) {
        println!("Measured capacity {:.2} pF", cx);
        flcq.set_quartz_crefs(cx);
    };
}

pub fn calibration(
    port: std::string::String,
    frequency_cal: bool,
    capacity: bool,
    induction: bool,
    quartz: bool,
    ref_frequency: std::option::Option<u32>,
    count: std::option::Option<u8>,
    c1: std::option::Option<u16>,
    c2: std::option::Option<u16>,
) {
    if frequency_cal {
        if let Some(f) = ref_frequency {
            if let Some(c) = count {
                frequency(&port, c, f as f64);
            }
        }
    } else if capacity {
        lc::capacity_calibration(port, c1, c2);
    } else if induction {
        lc::induction_calibration(port, c1, c2);
    } else if quartz {
        quartz_calibration_cs(&port);
    }
}
