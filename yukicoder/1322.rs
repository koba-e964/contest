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
    ($next:expr, ) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, [graph1; $len:expr]) => {{
        let mut g = vec![vec![]; $len];
        let ab = read_value!($next, [(usize1, usize1)]);
        for (a, b) in ab {
            g[a].push(b);
            g[b].push(a);
        }
        g
    }};
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

trait Change {
    fn chmax(&mut self, x: Self);
    fn chmin(&mut self, x: Self);
}
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) {
        if *self < x { *self = x; }
    }
    fn chmin(&mut self, x: T) {
        if *self > x { *self = x; }
    }
}

#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
}

struct DivDP {
    // stores dp[n], dp[n/2], ..., dp[n/b].
    dp_big: Vec<i64>,
    dp: Vec<i64>,
    n: i64,
    b: i64,
}

impl DivDP {
    fn new(n: i64, b: i64) -> Self {
        let dp_big = vec![0; b as usize + 1];
        let dp = vec![0; (n / b) as usize];
        DivDP {
            dp_big: dp_big,
            dp: dp,
            n: n,
            b: b,
        }
    }
    // pos should be of form floor(n / ???).
    fn upd<F>(&mut self, pos: i64, f: F) where F: Fn(i64) -> i64 {
        if pos >= self.n / self.b {
            let idx = self.n / pos;
            debug_assert_eq!(pos, self.n / idx);
            self.dp_big[idx as usize] = f(self.dp_big[idx as usize]);
            return;
        }
        let idx = pos as usize;
        self.dp[idx] = f(self.dp[idx]);
    }
    fn get(&self, pos: i64) -> i64 {
        if pos >= self.n / self.b {
            let idx = self.n / pos;
            debug_assert_eq!(pos, self.n / idx);
            return self.dp_big[idx as usize];
        }
        let idx = pos as usize;
        self.dp[idx]
    }
    fn init<F>(&mut self, f: F) where F: Fn(i64) -> i64 {
        for i in 0..self.dp.len() {
            self.dp[i] = f(i as i64);
        }
        for i in (1..self.dp_big.len()).rev() {
            self.dp_big[i] = f(self.n / i as i64);
        }
    }
    fn upd_all<F>(&mut self, f: F) where F: Fn(i64, i64) -> i64 {
        for i in 0..self.dp.len() {
            self.dp[i] = f(i as i64, self.dp[i]);
        }
        for i in (1..self.dp_big.len()).rev() {
            self.dp_big[i] = f(self.n / i as i64, self.dp_big[i]);
        }
    }
}

impl std::fmt::Debug for DivDP {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in 0..self.dp.len() {
            writeln!(f, "{}: {}", i, self.dp[i])?;
        }
        for i in (1..self.dp_big.len()).rev() {
            writeln!(f, "{}: {}", self.n / i as i64, self.dp_big[i])?;
        }
        Ok(())
    }
}

fn primes(v: usize) -> Vec<usize> {
    let mut pr = vec![true; v + 1];
    pr[0] = false;
    pr[1] = false;
    for i in 2..v + 1 {
        if !pr[i] {
            continue;
        }
        for j in 2..v / i + 1 {
            pr[i * j] = false;
        }
    }
    let prs: Vec<_> = (0..v + 1).filter(|&i| pr[i]).collect();
    prs
}

fn is_prime(x: i64) -> bool {
    if x <= 1 {
        return false;
    }
    let mut i = 2;
    while i * i <= x {
        if x % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

// Tags: sieve, prime-sieve, Lucy's-sieve
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    #[allow(unused)]
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    input! {
        n: i64,
    }
    let mut sqn = 0;
    while sqn * sqn <= n {
        sqn += 1;
    }
    sqn -= 1;
    let prs = primes(sqn as usize + 1);
    let mut dp = DivDP::new(n, sqn);
    dp.init(|x| max(0, x - 1));
    for &p in &prs {
        let p = p as i64;
        for i in 1..=min(sqn, n / p / p) {
            let val = dp.get(n / i / p);
            let val = val - dp.get(p - 1);
            dp.upd(n / i, |x| x - val);
        }
        for i in (p * p..n / sqn).rev() {
            let val = dp.get(i / p);
            let val = val - dp.get(p - 1);
            dp.upd(i, |x| x - val);
        }
    }
    // dp[j] = #{x <= j | x is prime}
    // eprintln!("dp:\n{:?}", dp);
    // dp[j] = #{x <= j + 1 | x is prime}
    dp.upd_all(|idx, val| val + if is_prime(idx + 1) { 1 } else { 0 });
    // eprintln!("dp:\n{:?}", dp);

    for &p in prs.iter().rev() {
        let p = p as i64;
        for i in 1..=min(sqn, n / p / (p - 1)) {
            let mut cur = p - 1;
            let mut val = 0;
            while cur * p <= n && n / i >= cur * p {
                val += dp.get(n / i / cur);
                val -= dp.get(p - 1);
                cur *= p;
                val += 1; // for cur * p
            }
            dp.upd(n / i, |x| x + val);
        }
        for i in (p * (p - 1)..n / sqn).rev() {
            let mut cur = p - 1;
            let mut val = 0;
            while cur * p <= n && i >= cur * p {
                val += dp.get(i / cur);
                val -= dp.get(p - 1);
                cur *= p;
                val += 1; // for cur * p
            }
            dp.upd(i, |x| x + val);
        }
        // eprintln!("p = {}, dp = {:?}", p, dp);
    }
    // dp[j] = #{x | phi(x) <= j}
    // eprintln!("dp = {:?}", dp);
    puts!("{}\n", dp.get(n) + 1);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
