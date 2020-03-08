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
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
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
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        (0..len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    }};

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
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

const W: usize = 50100;

// #{(x, y) in S | x >= a, y >= b, 0 < (x - a) * (y - b) <= s}
fn calc(xn: &[Vec<usize>], yn: &[Vec<usize>], s: i64)
        -> i64 {
    let sq = ((s as f64).sqrt() + 1.0) as usize;
    let mut tot = 0i64;
    for a in 0..W {
        for &b in &xn[a] {
            for i in a + 1..min(W, a + sq) {
                let hi = s as usize / (i - a) + b;
                let hiidx = xn[i].upper_bound(&hi);
                let loidx = xn[i].upper_bound(&b);
                tot += (hiidx - loidx) as i64;
            }
            for i in b + 1..min(W, b + max(sq, s as usize / sq + 1)) {
                if i >= W { continue; }
                let hi = s as usize / (i - b) + a;
                let lo = a + sq;
                let hiidx = yn[i].upper_bound(&hi);
                let loidx = yn[i].lower_bound(&lo);
                tot += (max(hiidx, loidx) - loidx) as i64;
            }
        }
    }
    tot
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        n: usize, s1: i64, s2: i64,
        ab: [(usize, usize); n],
    }
    let mut xn = vec![vec![]; W];
    let mut yn = vec![vec![]; W];
    for &(a, b) in &ab {
        xn[a].push(b);
        yn[b].push(a);
    }
    for i in 0..W {
        xn[i].sort();
        yn[i].sort();
    }
    let mut tot: i64 = 0;
    tot += calc(&xn, &yn, s2) - calc(&xn, &yn, s1 - 1);
    puts!("{}\n", tot);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
