#[allow(unused_imports)]
use std::cmp::*;
use std::io::*;
use std::ops::*;
#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok();
    return ret;
}
#[allow(dead_code)]
fn getword() -> String {
    let mut stdin = std::io::stdin();
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(16);
        loop {
            let res = stdin.read(&mut u8b);
            if res.is_err() ||u8b[0] <= ' ' as u8 {
                break;
            } else {
                buf.push(u8b[0]);
            }
        }
        if buf.len() >= 1 {
            let ret = std::string::String::from_utf8(buf).unwrap();
            return ret;
        }
    }
}
#[allow(dead_code)]
fn parse<T : std::str::FromStr>(s : &str) -> T {
     return s.parse::<T>().ok().unwrap();
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/*
 * Gaussian integer.
 */
struct Comp {
    x: i64, y: i64,
}
impl Comp {
    fn new() -> Self {
        Comp {x: 0, y: 0}
    }
    fn new_xy(x: i64, y: i64) -> Self {
        Comp {x: x, y: y}
    }
}
impl Sub for Comp {
    type Output = Comp;
    fn sub(self, r: Self) -> Self {
        Comp {x: self.x - r.x, y: self.y - r.y}
    }
}
impl Mul for Comp {
    type Output = Comp;
    fn mul(self, r: Self) -> Self {
        Comp {x: self.x * r.x - self.y * r.y, y: self.x * r.y + self.y * r.x}
    }
}

fn conjg(c: Comp) -> Comp {
    Comp {x: c.x, y: -c.y}
}
fn norm(c: Comp) -> i64 {
    c.x * c.x + c.y * c.y
}

fn good_quo(a: i64, b:i64) -> i64 {
    assert!(b > 0);
    if a < 0 {
        let r = a / b;
        if a % b < -b / 2 {
            return r - 1;
        } else {
            return r;
        }
        } else {
            let r = a / b;
            if a % b > b / 2 {
                return r + 1;
            } else {
                return r;
            }
        }
        
}
fn gcd(x: Comp, y: Comp) -> Comp {
    let mut p = x;
    let mut q = y;
    while q != Comp::new() {
        // p % q
        let pqc = p * conjg(q);
        let qn = norm(q);
        let quo = Comp::new_xy(good_quo(pqc.x, qn), good_quo(pqc.y, qn));
        let r = p - q * quo;
        p = q;
        q = r;
    }
    return p;
}

fn is_mul(a: Comp, b: Comp) -> bool {
    let abc = a * conjg(b);
    let bn = norm(b);
    if bn == 0 {
        return a == Comp::new();
    }
    return abc.x % bn == 0 && abc.y % bn == 0;
}

fn main() {
    let p: i64 = parse(&getword());
    let q: i64 = parse(&getword());
    let pq = Comp {x: p, y: q};
    let pq2 = Comp {x: p, y: -q};
    let n: usize = parse(&getword());
    let mut ary = vec![Comp::new(); n];
    for i in 0 .. n {
        let x = parse(&getword());
        let y = parse(&getword());
        ary[i] = Comp::new_xy(x, y);
    }
    let g = gcd(pq, pq2);
    println!("{}", ary.into_iter().filter(|x| is_mul(*x, g)).count());
}
