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

mod pollard_rho {
    use std::collections::HashMap;
    /// binary gcd
    pub fn gcd(mut x: i64, mut y: i64) -> i64 {
        if y == 0 { return x; }
        if x == 0 { return y; }
        let k = (x | y).trailing_zeros();
        x >>= k;
        y >>= k;
        while (x & 1) == 0 { x >>= 1; }
        while y != 0 {
            y >>= y.trailing_zeros();
            if x > y { let t = x; x = y; y = t; }
            y -= x;
        }
        x << k
    }

    fn add_mod(x: i64, y: i64, n: i64) -> i64 {
        let z = x + y;
        if z >= n { z - n } else { z }
    }

    fn mul_mod(x: i64, y: i64, n: i64) -> i64 {
        let a = x as i128 * y as i128 % n as i128;
        a as i64
    }

    fn mod_pow(x: i64, mut e: i64, n: i64) -> i64 {
        let mut prod = if n == 1 { 0 } else { 1 };
        let mut cur = x % n;
        while e > 0 {
            if (e & 1) == 1 {
                prod = mul_mod(prod, cur, n);
            }
            e >>= 1;
            if e > 0 {
                cur = mul_mod(cur, cur, n);
            }
        }
        prod
    }

    pub fn is_prime(n: i64) -> bool {
        if n <= 1 { return false; }
        let small = [2, 3, 5, 7, 11, 13];
        if small.iter().any(|&u| u == n) { return true; }
        if small.iter().any(|&u| n % u == 0) { return false; }
        let mut d = n - 1;
        let e = d.trailing_zeros();
        d >>= e;
        // https://miller-rabin.appspot.com/
        let a = [2, 325, 9375, 28178, 450775, 9780504, 1795265022];
        a.iter().all(|&a| {
            if a % n == 0 { return true; }
            let mut x = mod_pow(a, d, n);
            if x == 1 { return true; }
            for _ in 0 .. e {
                if x == n - 1 {
                    return true;
                }
                x = mul_mod(x, x, n);
                if x == 1 { return false; }
            }
            x == 1
        })
    }

    fn pollard_rho(n: i64, c: &mut i64) -> i64 {
        if n % 2 == 0 { return 2; }
        loop {
            let mut x: i64 = 2;
            let mut y = 2;
            let mut d = 1;
            let cc = *c;
            let f = |i| add_mod(mul_mod(i, i, n), cc, n);
            while d == 1 {
                x = f(x);
                y = f(f(y));
                d = gcd((x - y).abs(), n);
            }
            if d == n {
                *c += 1;
                continue;
            }
            return d;
        }
    }

    /// Outputs (p, e) in p's ascending order.
    pub fn factorize(x: i64) -> Vec<(i64, usize)> {
        if x <= 1 {
            return Vec::new();
        }
        let mut hm = HashMap::new();
        let mut pool = vec![x];
        let mut c = 1;
        while let Some(u) = pool.pop() {
            if is_prime(u) {
                *hm.entry(u).or_insert(0) += 1;
                continue;
            }
            let p = pollard_rho(u, &mut c);
            pool.push(p);
            pool.push(u / p);
        }
        let mut v: Vec<_> = hm.into_iter().collect();
        v.sort();
        v
    }
} // mod pollard_rho

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input!(a: [i64]);
    for a in a {
        let fac = pollard_rho::factorize(a);
        let mut ps = vec![];
        for (p, e) in fac {
            for _ in 0..e {
                ps.push(p);
            }
        }
        puts!("{}", ps.len());
        for p in ps {
            puts!(" {}", p);
        }
        puts!("\n");
    }
}
