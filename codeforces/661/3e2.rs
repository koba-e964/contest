#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
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


#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
}

fn dfs(g: &[Vec<(usize, i64, i32)>], v: usize, par: usize,
       info: &mut Vec<(i64, i64, i32)>) -> i64 {
    let mut sz = 0;
    let mut numch = 0;
    for &(to, w, c) in &g[v] {
        if to == par {
            continue;
        }
        numch += 1;
        let sub = dfs(g, to, v, info);
        sz += sub;
        info.push((w, sub, c));
    }
    if numch == 0 {
        sz += 1;
    }
    sz
}

// reduced
fn tbl(x: &[(i64, i64)]) -> Vec<i64> {
    let mut ans = vec![0];
    let mut tmp = vec![];
    for &(mut w, mul) in x {
        while w > 0 {
            tmp.push((w - w / 2) * mul);
            w /= 2;
        }
    }
    tmp.sort(); tmp.reverse();
    let mut sred = 0;
    for &t in &tmp {
        sred += t;
        ans.push(sred);
    }
    ans
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
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    let t: usize = get();
    for _ in 0..t {
        let n: usize = get();
        let s: i64 = get();
        let mut g = vec![vec![]; n];
        for _ in 0..n - 1 {
            let u = get::<usize>() - 1;
            let v = get::<usize>() - 1;
            let w: i64 = get();
            let c: i32 = get();
            g[v].push((u, w, c));
            g[u].push((v, w, c));
        }
        let mut info = vec![];
        dfs(&g, 0, n, &mut info);
        let mut one = vec![];
        let mut two = vec![];
        let mut goal = -s;
        for (w, mul, c) in info {
            if c == 1 {
                one.push((w, mul));
            } else {
                two.push((w, mul));
            }
            goal += w * mul;
        }
        if goal <= 0 {
            puts!("0\n");
            continue;
        }

        let one_tbl = tbl(&one);
        let two_tbl = tbl(&two);

        let mut mi = 1 << 28;
        for i in 0..one_tbl.len() {
            let rem = goal - one_tbl[i];
            let idx = two_tbl.lower_bound(&rem);
            if idx < two_tbl.len() {
                mi = min(mi, i + idx * 2);
            }
        }
        assert!(mi < 1 << 28);
        puts!("{}\n", mi);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
