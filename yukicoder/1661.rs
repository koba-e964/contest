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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
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
    #[allow(unused)]
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

// return pi(n) + pi(n / 2)
fn calc(n: i64, prs: &[usize]) -> i64 {
    if n <= 1 {
        return 0;
    }
    let mut sqn = 0;
    while sqn * sqn <= n {
        sqn += 1;
    }
    sqn -= 1;
    let mut dp = DivDP::new(n, sqn);
    dp.init(|x| max(0, x - 1));
    for &p in prs {
        let p = p as i64;
        if p * p > n {
            break;
        }
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
    dp.get(n) + dp.get(n / 2)
}

// Tags: lucys-algorithm, prime-counting
fn main() {
    input!(l: i64, r: i64);
    let prs = primes(150_000);
    let mut val = calc(2 * r - 1, &prs) - calc(2 * l - 1, &prs);
    if is_prime(r) {
        val += 1;
    }
    if 2 * l - 1 < 2 && 2 <= 2 * r - 1 {
        val -= 1;
    }
    println!("{}", val);
}
