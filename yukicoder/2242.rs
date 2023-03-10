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
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

trait Bisect<T> {
    fn lower_bound(&self, val: &T) -> usize;
    fn upper_bound(&self, val: &T) -> usize;
}

impl<T: Ord> Bisect<T> for [T] {
    fn lower_bound(&self, val: &T) -> usize {
        let mut pass = self.len() + 1;
        let mut fail = 0;
        while pass - fail > 1 {
            let mid = (pass + fail) / 2;
            if &self[mid - 1] >= val {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        pass - 1
    }
    fn upper_bound(&self, val: &T) -> usize {
        let mut pass = self.len() + 1;
        let mut fail = 0;
        while pass - fail > 1 {
            let mid = (pass + fail) / 2;
            if &self[mid - 1] > val {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        pass - 1
    }
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize,
        h: [i64; n],
        t: [i64; n],
        q: usize,
        ab: [(usize1, usize1); q],
    }
    let mut hi = vec![];
    for i in 0..n {
        hi.push((h[i], i));
    }
    hi.sort();
    let mut map = vec![0; n];
    for i in 0..n {
        map[hi[i].1] = i;
    }
    let mut to = vec![0; n];
    for i in 0..n {
        let idx = hi.upper_bound(&(t[i], n));
        to[map[i]] = idx;
    }
    const B: usize = 18;
    let mut bin = vec![vec![0; n + 1]; B];
    for i in 0..n {
        bin[0][i + 1] = max(bin[0][i], to[i]);
    }
    for i in 0..B - 1 {
        for j in 0..n + 1 {
            bin[i + 1][j] = bin[i][bin[i][j]];
        }
    }
    for (a, b) in ab {
        let a = map[a];
        let b = map[b];
        let mut a = to[a];
        if b < a {
            puts!("1\n");
            continue;
        }
        if bin[0][a] <= a {
            puts!("-1\n");
            continue;
        }
        let mut x = 0;
        for i in (0..B).rev() {
            if bin[i][a] <= b {
                x |= 1 << i;
                a = bin[i][a];
            }
        }
        if bin[0][a] <= b {
            puts!("-1\n");
        } else {
            puts!("{}\n", x + 2);
        }
    }
}
