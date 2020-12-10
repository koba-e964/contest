// https://atcoder.jp/contests/joisc2007/submissions/18675713
fn karatsuba_convolution_sub(a: &[i64], b: &[i64], out: &mut [i64]) {
    let n = a.len();
    let m = b.len();
    if n == 0 || m == 0 {
        return;
    }
    if min(n, m) <= 5 {
        for i in 0..n {
            for j in 0..m {
                if i + j < out.len() {
                    out[i + j] += a[i] * b[j];
                }
            }
        }
        return;
    }
    let l = max(n, m);
    let al = &a[..min(a.len(), l / 2)];
    let bl = &b[..min(b.len(), l / 2)];
    let ah = &a[min(a.len(), l / 2)..];
    let bh = &b[min(b.len(), l / 2)..];
    if ah.is_empty() || bh.is_empty() {
        karatsuba_convolution_sub(&al, &bl, out);
        if out.len() >= l / 2 {
            karatsuba_convolution_sub(&al, &bh, &mut out[l / 2..]);
            karatsuba_convolution_sub(&ah, &bl, &mut out[l / 2..]);
            if out.len() >= l / 2 * 2 {
                karatsuba_convolution_sub(&ah, &bh, &mut out[l / 2 * 2..]);
            }
        }
        return;
    }
    let mut lo = vec![0; al.len() + bl.len()];
    karatsuba_convolution_sub(&al, &bl, &mut lo);
    let mut hi = vec![0; ah.len() + bh.len()];
    karatsuba_convolution_sub(&ah, &bh, &mut hi);
    // al * bh + ah * bl = al * bl + ah * bh - (al - ah) * (bl - bh)
    let mut dif_a = vec![0; l - l / 2]; // ah - al
    let mut dif_b = vec![0; l - l / 2]; // bl - bh
    for i in 0..l / 2 {
        if i < al.len() {
            dif_a[i] = -al[i];
        }
        if i < bl.len() {
            dif_b[i] = bl[i];
        }
    }
    for i in 0..l - l / 2 {
        if i < ah.len() {
            dif_a[i] += ah[i];
        }
        if i < bh.len() {
            dif_b[i] -= bh[i];
        }
    }
    if out.len() > l / 2 {
        karatsuba_convolution_sub(&dif_a, &dif_b, &mut out[l / 2..]);
    }
    for i in 0..min(lo.len(), out.len()) {
        out[i] += lo[i];
        if i + l / 2 < out.len() {
            out[i + l / 2] += lo[i];
        }
    }
    for i in 0..min(hi.len(), out.len()) {
        if i + l / 2 * 2 < out.len() {
            out[i + l / 2 * 2] += hi[i];
        }
        if i + l / 2 < out.len() {
            out[i + l / 2] += hi[i];
        }
    }
}
fn karatsuba_convolution(a: &[i64], b: &[i64]) -> Vec<i64> {
    let n = a.len();
    let m = b.len();
    let mut ret = vec![0; n + m - 1];
    karatsuba_convolution_sub(a, b, &mut ret);
    ret
}
