#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Write, BufWriter};
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes.by_ref().map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, [graph1; $len:expr]) => {{
        let mut g = vec![vec![]; $len];
        let ab = read_value!($next, [(usize1, usize1)]);
        for (a, b) in ab {
            g[a].push(b);
            g[b].push(a);
        }
        g
    }};
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// https://atcoder.jp/contests/joisc2007/submissions/18675713
use std::ops::*;
fn karatsuba_convolution_sub<R>(a: &[R], b: &[R], out: &mut [R])
where R: Copy +
    AddAssign<R> +
    SubAssign<R> +
    Mul<Output = R> +
    Default {
    let n = a.len();
    let m = b.len();
    if n == 0 || m == 0 {
        return;
    }
    if min(n, m) <= 64 {
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
    let mut lo = vec![R::default(); al.len() + bl.len()];
    karatsuba_convolution_sub(&al, &bl, &mut lo);
    let mut hi = vec![R::default(); ah.len() + bh.len()];
    karatsuba_convolution_sub(&ah, &bh, &mut hi);
    // al * bh + ah * bl = al * bl + ah * bh - (al - ah) * (bl - bh)
    let mut dif_a = vec![R::default(); l - l / 2]; // ah - al
    let mut dif_b = vec![R::default(); l - l / 2]; // bl - bh
    for i in 0..l / 2 {
        if i < al.len() {
            dif_a[i] -= al[i];
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
fn karatsuba_convolution<R>(a: &[R], b: &[R]) -> Vec<R>
where R: Copy +
    AddAssign<R> +
    SubAssign<R> +
    Mul<Output = R> +
    Default {
    let n = a.len();
    let m = b.len();
    let mut ret = vec![R::default(); n + m - 1];
    karatsuba_convolution_sub(a, b, &mut ret);
    ret
}

// Tags: fft, convolution, karatsuba
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        n: usize, q: usize,
        a: [i64; n],
        r: [usize; q],
    }
    let mut f = vec![0i64; n];
    let mut g = vec![0; n + 1];
    for i in 0..n {
        f[i] += a[i];
    }
    for r in r {
        g[n - r] += 1;
    }
    let res = karatsuba_convolution(&f, &g);
    let mut ans = vec![0; n];
    for i in 0..res.len() {
        ans[i % n] += res[i];
    }
    for i in 0..n {
        puts!("{}{}", ans[i], if i + 1 == n { "\n" } else { " " });
    }
}
