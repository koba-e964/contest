use std::cmp::*;
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
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn int_cover(v: &[(usize, i64, i64)], ans: &mut [i64]) {
    let mut ev = vec![];
    for &(idx, l, r) in v {
        ev.push((l, 1, idx));
        ev.push((r, 0, idx));
    }
    ev.sort();
    let mut rem = vec![];
    for (pos, kind, idx) in ev {
        if kind == 0 {
            if ans[idx] == -1 {
                for idx in rem.drain(..) {
                    ans[idx] = pos - 1;
                }
            }
        } else {
            rem.push(idx);
        }
    }
}

// Tags: inverval-scheduling, interval-covering
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input!(abm: [[(i64, i64, i64)]]);
    for abm in abm {
        let n = abm.len();
        let mut hm = HashMap::new();
        for i in 0..n {
            let (a, b, m) = abm[i];
            let k = a + b - m;
            hm.entry(k).or_insert(vec![])
                .push((i, max(a - m, 0), a - max(m - b, 0) + 1));
        }
        let mut ans = vec![-1; n];
        for (_, v) in hm {
            int_cover(&v, &mut ans);
        }
        let mut coo = vec![];
        for i in 0..n {
            let (a, b, m) = abm[i];
            coo.push((ans[i], a + b - m - ans[i]));
        }
        coo.sort(); coo.dedup();
        puts!("{}\n", coo.len());
        for i in 0..n {
            let (a, _b, m) = abm[i];
            puts!("{} {}\n", a - ans[i], m + ans[i] - a);
        }
    }
}
