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
        n: usize,
        a: [usize; n],
    }
    const W: usize = 1_000_001;
    let mut f = vec![0; W];
    for &a in &a {
        f[a] += 1;
    }
    let mut div = vec![0; W];
    for i in 1..W {
        for j in 1..(W - 1) / i + 1 {
            div[i] += f[i * j];
        }
    }
    let mut ans = vec![0; n];
    for i in 1..W {
        let idx = n - div[i];
        if idx < n {
            ans[idx] = max(ans[idx], i);
        }
    }
    for i in 1..n {
        ans[i] = max(ans[i], ans[i - 1]);
    }
    for i in 0..n {
        puts!("{}\n", ans[i]);
    }
}
