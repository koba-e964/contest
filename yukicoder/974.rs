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

fn calc(abc: &[(i64, i64, i64)], w2: usize) -> i64 {
    let w = abc.len();
    let mut former = vec![];
    for bits in 0..1 << w2 {
        let lower = bits / 3;
        if (lower | lower << 1) != bits {
            continue;
        }
        let mut sum = 0;
        let mut like = 0;
        for i in 0..w2 {
            if (lower << 1 & 1 << i) != 0 {
                sum -= abc[i - 1].0 + abc[i].2;
                like += abc[i].1;
            } else {
                sum += abc[i].0;
            }
        }
        former.push((sum, like));
    }
    former.sort();
    for i in (0..former.len() - 1).rev() {
        former[i].1 = max(former[i].1, former[i + 1].1);
    }
    let mut ma = 0;
    for bits in 0..1 << (w - w2) {
        let lower = bits / 3;
        if (lower | lower << 1) != bits {
            continue;
        }
        let mut sum = 0;
        let mut like = 0;
        for i in 0..w - w2 {
            if (lower << 1 & 1 << i) != 0 {
                sum -= abc[w2 + i - 1].0 + abc[w2 + i].2;
                like += abc[w2 + i].1;
            } else {
                sum += abc[w2 + i].0;
            }
        }
        let idx = former.lower_bound(&(-sum, -1 << 50));
        if idx < former.len() {
            ma = max(ma, former[idx].1 + like);
        }
    }
    ma
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        w: usize,
        abc: [(i64, i64, i64); w],
    }
    let w2 = w / 2;
    let mut ma = calc(&abc, w2);
    if w >= 2 {
        ma = max(ma, calc(&abc, w2 - 1));
    }
    puts!("{}\n", ma);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
