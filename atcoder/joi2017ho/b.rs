use std::cmp::*;
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
    input! {
        n: i64, m: usize, k: usize,
        a: i64, b: i64, c: i64,
        t: i64,
        s: [i64; m],
    }
    let mut tot = -1; // 1 is always good and always excluded
    let mut ev = vec![];
    for i in 0..m {
        let l = s[i];
        let r = if i == m - 1 { n + 1 } else { s[i + 1] };
        let q = (r - l - 1) * a + (l - 1) * b;
        if q <= t {
            tot += r - l;
            continue;
        }
        if (l - 1) * b > t {
            continue;
        }
        let q = (t - (l - 1) * b) / a;
        tot += q + 1;
        let mut cur = l + q + 1;
        let mut iter = 0;
        while iter < k - m && cur < r {
            let val = t - (l - 1) * b - (cur - l) * c;
            if val < 0 {
                break;
            }
            let q = min(r - cur - 1, val / a);
            ev.push(q + 1);
            cur += q + 1;
            iter += 1;
        }
    }
    ev.sort_unstable(); ev.reverse();
    for i in 0..min(ev.len(), k - m) {
        tot += ev[i];
    }
    println!("{}", tot);
}
