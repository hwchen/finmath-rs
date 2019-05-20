use roots::roots;
use std::cmp::Ordering;

// overall flow drawn from numpy financial.irr()
// but made more rustic
pub fn irr(input: &[f64]) -> Option<f64> {
    let input: Vec<f64> = input.into_iter().rev().cloned().collect();
    let rs = roots(input.as_slice());

    let has_pos_real = rs.iter()
        .any(|c| c.im == 0.0 && c.re > 0.0);

    if !has_pos_real {
        return None;
    }

    let rs = rs.iter()
        .map(|c| c.re)
        .filter(|x| *x > 0.0);

    // find rate from res
    // if there's multiple roots, then return the rate which
    // is closest to 0
    rs
        .map(|x| 1.0/x - 1.0)
        .min_by(|x, y| {
            // the default Ordering::Equal should never be hit,
            // since all roots at this point are floats > 0
            x.abs().partial_cmp(&y.abs())
                .unwrap_or(Ordering::Equal)
        })
}

// following r's FinCal twrr
//
// This one returns Option for now, until I get an error enum
pub fn twrr(ev: &[f64], bv: &[f64], cfr: &[f64]) -> Option<f64> {
    if ev.len() != bv.len() && bv.len() != cfr.len() {
        return None
    }

    let r = ev.len();

    let mut wr: f64 = 1.0;
    for i in 0..r {
        wr = wr * (hpr(ev[i], bv[i], cfr[i]) + 1.0);
    }

    let res = wr.powf(1.0 / r as f64) - 1.0;

    Some(res)
}

pub fn hpr(ev: f64, bv: f64, cfr: f64) -> f64 {
    (ev - bv + cfr) / bv
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_irr() {
        // examples drawn from numpy financial.irr()

        let rate = irr(&[-100.0, 39.0, 59.0, 55.0, 20.0]).unwrap();
        assert_eq!(format!("{:.5}", rate), "0.28095");

        let rate = irr(&[-100.0, 0.0, 0.0, 74.0]).unwrap();
        assert_eq!(format!("{:.5}", rate), "-0.09550");

        let rate = irr(&[-100.0, 100.0, 0.0, -7.0]).unwrap();
        assert_eq!(format!("{:.5}", rate), "-0.08330");

        let rate = irr(&[-100.0, 100.0, 0.0, 7.0]).unwrap();
        assert_eq!(format!("{:.5}", rate), "0.06206");

        let rate = irr(&[-5.0, 10.5, 1.0, -8.0, 1.0]).unwrap();
        assert_eq!(format!("{:.5}", rate), "0.08860");

        // from wikipedia example
        let rate = irr(&[-123400.0, 36200.0, 54800.0, 48100.0]).unwrap();
        assert_eq!(format!("{:.4}", rate), "0.0596");
    }

    #[test]
    fn test_twrr() {
        // example drawn from R FinCal twrr

        let twrr = twrr(&[120.0, 260.0,], &[100.0, 240.0], &[2.0, 4.0]).unwrap();
        assert_eq!(format!("{:.5}", twrr), "0.15845");

    }
}
