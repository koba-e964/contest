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

/**
 * Sparse Table.
 * BiOp should be the type of a binary operator which is
 * associative, commutative and idempotent.
 * (For example, both min and gcd satisfy these properties.)
 * Verified by: AtCoder CODE FESTIVAL 2016 Tournament Round 3 (Parallel) B
 * (http://cf16-tournament-round3-open.contest.atcoder.jp/submissions/1026294)
 */
struct SparseTable<T, BiOp> {
    biop: BiOp,
    st: Vec<Vec<T>>,
}

impl<T, BiOp> SparseTable<T, BiOp>
    where BiOp: Fn(T, T) -> T,
          T: Copy {
    pub fn new(ary: &[T], biop: BiOp) -> Self {
        let n = ary.len();
        let mut h = 1;
        while 1 << h < n {
            h += 1;
        }
        let mut st: Vec<Vec<T>> = vec![Vec::from(ary); h + 1];
        for i in 0 .. n {
            st[0][i] = ary[i];
        }
        for b in 1 .. (h + 1) {
            if n + 1 < 1 << b {
                break;
            }
            for i in 0 .. (n + 1 - (1 << b)) {
                let next_idx = (1 << (b - 1)) + i;
                st[b][i] = biop(st[b - 1][i], st[b - 1][next_idx]);
            }
        }
        SparseTable {biop: biop, st: st}
    }
    fn top_bit(t: usize) -> usize {
        8 * std::mem::size_of::<usize>() - 1 - t.leading_zeros() as usize
    }
    pub fn query(&self, f: usize, s: usize) -> T {
        assert!(f <= s);
        let b = Self::top_bit(s + 1 - f);
        let endpoint = s + 1 - (1 << b);
        (self.biop)(self.st[b][f], self.st[b][endpoint])
    }
}

fn gcd(mut x: i64, mut y: i64) -> i64 {
    while y != 0 {
        let r = x % y;
        x = y;
        y = r;
    }
    x
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        n: usize,
        a: [i64; n],
    }
    let spt = SparseTable::new(&a, gcd);
    let mut tot: i64 = 0;
    let mut pos = 0;
    for i in 0..n {
        pos = max(pos, i);
        while pos < n {
            if spt.query(i, pos) != 1 {
                pos += 1;
            } else {
                break;
            }
        }
        tot += (n - pos) as i64;
    }
    puts!("{}\n", tot);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
