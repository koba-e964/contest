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

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize,
        a: i64, b: i64, c: i64,
        g: [i64; n],
    }
    let mut g = g;
    g.sort();
    let mut tot: i64 = 0;
    let mut dp = vec![0i64; n + 1];
    for i in 0..n + 1 {
        dp[i] = i as i64;
    }
    let mut dpb = vec![0i64; n + 1];
    for i in 0..n {
        let idx = g.lower_bound(&(b + g[i] - a));
        dpb[i + 1] = dpb[i] + dp[idx];
    }
    let mut dpc = vec![0i64; n + 1];
    for i in 0..n {
        let idx = g.lower_bound(&(c + g[i] - b));
        dpc[i + 1] = dpc[i] + dpb[idx];
    }
    tot += dpc[n];
    debugln!("all: {}", dpc[n]);

    // in-ex
    if a < b {
        // idx[a] == idx[b]
        let mut sum = 0i64;
        for i in 0..n {
            let idx = g.lower_bound(&(c + g[i] - b));
            sum += idx as i64;
        }
        tot -= sum;
        debugln!("a = b: {}", sum);
    }
    if b < c {
        tot -= dpb[n];
        debugln!("b = c: {}", dpb[n]);
    }
    if a < c {
        let mut sum = 0i64;
        for i in 0..n {
            let idx0 = g.lower_bound(&(b + g[i] - a));
            let idx1 = g.upper_bound(&(b + g[i] - c));
            if idx1 < idx0 {
                sum += (idx0 - idx1) as i64;
            }
        }
        tot -= sum;
        debugln!("a = c: {}", sum);
    }

    if a < b && b < c {
        tot += 2 * n as i64;
    }

    puts!("{}\n", tot);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
