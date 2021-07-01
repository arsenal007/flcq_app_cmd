#[path = "lc.rs"]
mod lc;

pub use self::lc::freq::com;

fn frequency(port: &std::string::String, count: u8, ref_freq: f64) -> () {
    let mut flcq: com::Flcq = com::open(port, true);

    if let (Some((_, f)), _) = flcq.frequency_raw(&count) {
        let periode = f / ref_freq;
        println!("Periode {:.?} [Sec]", periode);
        flcq.set_frequency(periode, count);
    }
}

pub fn calibration(
    port: std::string::String,
    frequency_cal: bool,
    capacitance: bool,
    inductance: bool,
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
    } else if capacitance {
        let mut flcq: com::Flcq = com::open(&port, true);
        if let Some((c0, _l0)) = lc::calibration(&mut flcq, &c1, &c2) {
            println!("Attach Cx and repeat measure sequence");

            if let Some((c, _l)) = lc::calibration(&mut flcq, &c1, &c2) {
                let cx = c - c0;
                println!("Measured capacitance {:.2} pF", cx);
                flcq.set_cref1_cref2(cx);
            }
        }
    } else if inductance {
        let mut flcq: com::Flcq = com::open(&port, true);
        if let Some((c0, l0)) = lc::calibration(&mut flcq, &c1, &c2) {
            flcq.set_c0_l0(c0, l0);
        } else {
            unimplemented!("inductance calibration")
        }
    }
}
