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

/// Generates an Iterator over subsets of univ, in the descending order. 
/// Verified by: http://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=3050308
struct SubsetIter { bits: Option<usize>, univ: usize }
impl Iterator for SubsetIter {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        match self.bits {
            None => None,
            Some(bits) => {
                let ans = bits;
                self.bits =
                    if bits == 0 { None }
                else { Some((bits - 1) & self.univ) };
                Some(ans)
            }
        }
    }
}
fn subsets(univ: usize) -> SubsetIter {
    SubsetIter { bits: Some(univ), univ: univ }
}

const INF: i32 = 1 << 27;

// Tags: 3^n dp key-value-exchange
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        a: [[i32]],
    }
    for a in a {
        let n = a.len();
        let mut dp = vec![vec![vec![(INF, 0, 0, 0); 1 << n]; n + 1]; n + 1];
        let mut sum = vec![0; 1 << n];
        for i in 0..n {
            for bits in 0..1 << n {
                if (bits & 1 << i) != 0 {
                    sum[bits] += a[i];
                }
            }
        }
        dp[0][0][0] = (0, 0, 0, 0);
        for v in 0..n {
            for w in v..n {
                for c in 0..n {
                    for bits in 0..1 << n {
                        let old = dp[v][c][bits].0;
                        if (v > 0 && (bits & 1 << (v - 1)) == 0) || old >= INF {
                            continue;
                        }
                        if (bits & 1 << w) != 0 {
                            continue;
                        }
                        for sub in subsets((1 << n) - 1 - bits - (1 << w)) {
                            if old >= sum[sub] + a[w] {
                                continue;
                            }
                            let to = bits | sub | 1 << w;
                            dp[w + 1][c + 1][to] = min(
                                dp[w + 1][c + 1][to],
                                (sum[sub] + a[w], v, c, bits));
                        }
                    }
                }
            }
        }
        let mut ma = (0, 0);
        for i in 0..n + 1 {
            for j in 0..n + 1 {
                if dp[i][j][(1 << n) - 1].0 < INF {
                    ma = max(ma, (j, i));
                }
            }
        }
        puts!("{}\n", n - ma.0);
        // path recovery
        let mut cur = (ma.1, ma.0, (1 << n) - 1);
        let mut path = vec![]; // moves: (from, to) from > to
        while cur.1 > 0 {
            let (w, c, bits) = cur;
            let (_, nxt0, nxt1, nxt2) = dp[w][c][bits];
            let used = bits - nxt2;
            let mut v = vec![];
            for i in (0..n).rev() {
                if (used & 1 << i) != 0 { v.push(i); }
            }
            for i in 0..v.len() {
                if v[i] != w - 1 {
                    path.push((v[i], w - 1));
                }
            }
            cur = (nxt0, nxt1, nxt2);
        }
        let mut rem: Vec<usize> = (0..n).collect();
        for (from, to) in path {
            let idx1 = rem.iter().position(|&x| x == from).unwrap();
            let idx2 = rem.iter().position(|&x| x == to).unwrap();
            puts!("{} {}\n", idx1 + 1, idx2 + 1);
            rem.remove(idx1);
        }
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
