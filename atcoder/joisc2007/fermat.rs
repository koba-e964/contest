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

#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
}

fn powmod(x: i64, mut e: i64, m: i64) -> i64 {
    let mut sum = 1;
    let mut cur = x % m;
    while e > 0 {
        if e % 2 != 0 {
            sum = sum * cur % m;
        }
        cur = cur * cur % m;
        e /= 2;
    }
    sum
}

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

// Tags: convolution, fft, karatsuba
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    #[allow(unused)]
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    input! {
        p: usize,
        n: usize,
    }
    let mut freq = vec![0; p];
    for i in 0..p {
        let x = powmod(i as i64, n as i64, p as i64);
        freq[x as usize] += 1;
    }
    let conv = karatsuba_convolution(&freq, &freq);
    // eprintln!("conv = {:?}, freq = {:?}", conv, freq);
    let mut tot = 0;
    for i in 0..conv.len() {
        tot += freq[i % p] * conv[i];
    }
    puts!("{}\n", tot);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
