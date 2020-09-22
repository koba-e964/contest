#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Read;

#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn get_word() -> String {
    let stdin = std::io::stdin();
    let mut stdin=stdin.lock();
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(16);
        loop {
            let res = stdin.read(&mut u8b);
            if res.unwrap_or(0) == 0 || u8b[0] <= b' ' {
                break;
            } else {
                buf.push(u8b[0]);
            }
        }
        if buf.len() >= 1 {
            let ret = String::from_utf8(buf).unwrap();
            return ret;
        }
    }
}

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

trait Int {
    fn a(&mut self, a: i64) -> i64;
    fn b(&mut self, a: i64) -> i64;
    fn c(&mut self, a: i64);
}

struct Real;
struct Fake {
    tbl: Vec<bool>,
    x: usize,
    nq: usize,
}

impl Int for Real {
    fn a(&mut self, a: i64) -> i64 {
        println!("A {}", a);
        let res: i64 = get();
        res
    }
    fn b(&mut self, a: i64) -> i64 {
        println!("B {}", a);
        let res: i64 = get();
        res
    }
    fn c(&mut self, a: i64) {
        println!("C {}", a);
    }
}

impl Fake {
    #[allow(unused)]
    fn new(n: usize, x: usize) -> Fake {
        let mut t = vec![true; n + 1];
        t[0] = false;
        Fake {
            tbl: t,
            x: x,
            nq: 0,
        }
    }
}

impl Int for Fake {
    fn a(&mut self, a: i64) -> i64 {
        assert!(a >= 1);
        let n = self.tbl.len() - 1;
        assert!(a <= n as i64);
        let mut tot = 0;
        for i in 1..n as i64 / a + 1 {
            if self.tbl[(i * a) as usize] {
                tot += 1;
            }
        }
        self.nq += 1;
        tot
    }
    fn b(&mut self, a: i64) -> i64 {
        assert!(a >= 2);
        let n = self.tbl.len() - 1;
        assert!(a <= n as i64);
        let mut tot = 0;
        for i in 1..n as i64 / a + 1 {
            let val = (i * a) as usize;
            if self.tbl[val] {
                tot += 1;
                if val != self.x {
                    self.tbl[val] = false;
                }
            }
        }
        self.nq += 1;
        tot
    }
    fn c(&mut self, a: i64) {
        self.nq += 1;
        assert_eq!(self.x as i64, a);
    }
}

fn solve_once(n: usize, int: &mut dyn Int) {
    if n == 1 {
        int.c(1);
        return;
    }
    let mut known: i64 = 1;
    let mut max: i64 = n as i64;
    let w: usize = n + 1;
    let mut pr = vec![true; w];
    pr[0] = false;
    pr[1] = false;
    for i in 2..w {
        if !pr[i] {
            continue;
        }
        for j in 2..(w - 1) / i + 1 {
            pr[i * j] = false;
        }
    }
    let mut p = 2;
    while p * p <= max {
        if !pr[p as usize] {
            p += 1;
            continue;
        }
        assert!(known <= n as i64);
        let mut e = 0;
        if known == 1 {
            int.b(known * p);
        }
        let val = int.a(known * p);
        if val != 0 {
            e += 1;
            let mut cur = known * p;
            loop {
                if cur * p > n as i64 || int.a(cur * p) == 0 {
                    break;
                }
                e += 1;
                cur *= p;
            }
        }
        for _ in 0..e {
            known *= p;
            max /= p;
        }
        p += 1;
    }
    let mut restp = vec![];
    for i in p..n as i64 + 1 {
        if pr[i as usize] {
            restp.push(i);
        }
    }
    let m = restp.len();
    // if known > 1, there's no need of sqrt decomposition.
    if known > 1 {
        for &p in &restp {
            if known * p <= n as i64 && int.a(known * p) == 1 {
                known *= p;
            }
        }
        int.c(known);
        return;
    }
    // x is a big prime.
    // Sqrt decomposition
    let mut sqm = 0;
    while sqm * sqm < m {
        sqm += 1;
    }
    for i in 0..(m + sqm - 1) / sqm + 1 {
        let lo = i * sqm;
        let hi = min(m, (i + 1) * sqm);
        for j in lo..hi {
            let p = restp[j];
            int.b(p);
        }
        let a1 = int.a(1);
        // The rest of restp, and 1.
        if a1 != (m - hi) as i64 + 1 {
            assert_eq!(a1, (m - hi) as i64 + 2);
            for j in lo..hi {
                if int.a(restp[j]) == 1 {
                    int.c(restp[j]);
                    return;
                }
            }
        }
    }
    int.c(known);
}

fn solve() {
    if true {
        let n: usize = get();
        solve_once(n, &mut Real);
    } else {
        let n = 1;
        let mut ma = 0;
        for x in 1..n + 1 {
            if x % 5000 == 0 {
                eprintln!("x: {}/{}, ma = {}", x, n, ma);
            }
            let mut int = Fake::new(n, x);
            solve_once(n, &mut int);
            ma = max(int.nq, ma);
        }
        eprintln!("n = {}, ma = {}", n, ma);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
