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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// https://yukicoder.me/problems/no/1220 (3)
// F = M * C(N, K) / (全体), S = M^K (N - K + 1) / (全体) なので、F/S が計算できる。
// 対数で計算すると安全。
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        q: usize,
        nmk: [(usize, usize, usize); q],
    }
    const W: usize = 100_100;
    let mut ln = vec![0.0; W];
    let mut acc = vec![0.0; W];
    for i in 1..W {
        ln[i] = (i as f64).ln();
        acc[i] = acc[i - 1] + ln[i];
    }
    for (n, m, k) in nmk {
        let diff =
            ln[m] + acc[n] - acc[k] - acc[n - k]
            - (k as f64 * ln[m] + ln[n - k + 1]);
        puts!("{}\n", if diff < 0.0 { "Flush" } else { "Straight" });
    }
}
