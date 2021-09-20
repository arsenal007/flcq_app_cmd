use crate::flcq::{com, lc};

pub fn raw(flcq: &mut com::Flcq) -> std::option::Option<f64> {
    if let Some((c, _)) = lc::measurments(flcq) {
        let (c0, _) = flcq.c0_l0();
        let cx = c - c0;
        return Some(cx);
    };
    None
}
