pub trait F {
    fn picofarad_to_farad(&self) -> f64;
    fn henry_to_micro_henry(&self) -> f64;
    fn henry_to_nano_henry(&self) -> f64;
}

impl F for f64 {
    fn picofarad_to_farad(&self) -> f64 {
        *self * 10.0_f64.powi(-12)
    }

    fn henry_to_micro_henry(&self) -> f64 {
        *self * 10.0_f64.powi(6)
    }

    fn henry_to_nano_henry(&self) -> f64 {
        *self * 10.0_f64.powi(9)
    }
}
