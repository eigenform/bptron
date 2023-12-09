
// FYI: this is probably not a good measure of 
fn shannon_entropy(x: u16) -> f64 {
    let mut res = 0.0;
    let n   = 16.0;
    let n_t = x.count_ones();
    let n_f = x.count_zeros();

    let p_t = (n_t as f64) / n;
    let p_f = (n_f as f64) / n;

    let res = -(p_t * p_t.log2() + p_f * p_f.log2());
    if res.is_nan() { 0.0 } else { res }
}


