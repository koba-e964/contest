use std::cmp::*;
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

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, m: usize,
        ab: [(usize, usize); n],
    }
    let mut orig = 0;
    let mut occ = vec![0; 2 * m + 1];
    for (a, b) in ab {
        orig += a;
        occ[b + m - a] += 1;
    }
    const INF: i32 = 1 << 25;
    let mut dp = vec![INF; m + 1];
    dp[orig] = 0;
    for i in 0..2 * m + 1 {
        if i == m { continue; }
        if occ[i] == 0 { continue; }
        let mut c = 1;
        let mut v = vec![];
        let mut rem = occ[i];
        while rem >= c {
            rem -= c;
            v.push(c);
            c *= 2;
        }
        if rem > 0 {
            v.push(rem);
        }
        if i < m {
            for &v in &v {
                let diff = (m - i) * v as usize;
                for i in 0..m + 1 - diff {
                    dp[i] = min(dp[i], dp[i + diff] + v);
                }
            }
        } else {
            for &v in &v {
                let diff = (i - m) * v as usize;
                for i in (0..m + 1 - diff).rev() {
                    dp[i + diff] = min(dp[i + diff], dp[i] + v);
                }
            }
        }
    }
    for i in 0..m + 1 {
        puts!("{}\n", if dp[i] < INF { dp[i] } else { -1 });
    }
}
