pub fn close(a: f64, b: f64, tolerance: f64) -> bool {
    if tolerance <= 0.0 {
        panic!("tolerance is lesser than 0")
    }

    (a-b).abs() <= (tolerance * a.abs().max(b.abs())).max(f64::EPSILON)
}
