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
    ($next:expr,) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

type C = (f64, f64);
fn addc((a, b): C, (c, d): C) -> C {
    (a + c, b + d)
}
fn mulc((a, b): C, (c, d): C) -> C {
    (a * c - b * d, a * d + b * c)
}
fn invc((a, b): C) -> C {
    (a, -b) // norm = 1, so inv = conj
}

fn mul((a, x): (C, C), (b, y): (C, C)) -> (C, C) {
    (mulc(a, b), addc(mulc(x, b), y))
}
fn inv((a, (x, y)): (C, C)) -> (C, C) {
    let inva = invc(a);
    (inva, mulc(inva, (-x, -y)))
}

// https://yukicoder.me/problems/no/1932 (3.5)
// 可逆なアファイン変換の範囲合成が欲しいので、セグメント木や累積和などでできる。
// Tags: complex-numbers, affine-transformations
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize,
        pqr: [(f64, f64, f64); n],
        q: usize,
        stxy: [(usize1, usize, (f64, f64)); q],
    }
    let mut acc = vec![((1.0, 0.0), (0.0, 0.0)); n + 1];
    for i in 0..n {
        let (p, q, r) = pqr[i];
        let r = r * std::f64::consts::PI / 180.0;
        let r = (r.cos(), r.sin());
        acc[i + 1] = mul(acc[i], (r, addc(mulc(r, (-p, -q)), (p, q))));
    }
    for (s, t, xy) in stxy {
        let (a, b) = mul(inv(acc[s]), acc[t]);
        let (x, y) = addc(mulc(a, xy), b);
        puts!("{} {}\n", x, y);
    }
}
