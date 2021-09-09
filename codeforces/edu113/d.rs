use std::collections::*;
use std::io::{Read, Write, BufWriter};

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

fn calc(a: &[i64]) -> i64 {
    let n = a.len() as i64;
    if n <= 1 {
        return 0;
    }
    let mut sum = n * (n - 1) / 2;
    let mut hm = HashMap::new();
    for &v in a {
        let &x = hm.get(&v).unwrap_or(&0);
        sum -= x;
        hm.insert(v, x + 1);
    }
    sum
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    let t: usize = get();
    for _ in 0..t {
        let n: usize = get();
        let m: usize = get();
        let k: usize = get();
        let x: Vec<i64> = (0..n).map(|_| get()).collect();
        let y: Vec<i64> = (0..m).map(|_| get()).collect();
        let p: Vec<_> = (0..k).map(|_| {
            let x: i64 = get();
            let y: i64 = get();
            (x, y)
        }).collect();
        let mut row = vec![vec![]; n];
        let mut col = vec![vec![]; m];
        for &(a, b) in &p {
            let i0 = x.lower_bound(&a);
            let i1 = y.lower_bound(&b);
            if x[i0] != a {
                row[i0].push(b);
            } else if y[i1] != b {
                col[i1].push(a);
            }
        }
        let mut ans = 0i64;
        for i in 0..n {
            ans += calc(&row[i]);
        }
        for i in 0..m {
            ans += calc(&col[i]);
        }
        puts!("{}\n", ans);
    }
}
