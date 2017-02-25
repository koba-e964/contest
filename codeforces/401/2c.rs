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
    let mut stdin = std::io::stdin();
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
        let mut h = 0;
        while 1 << h <= t {
            h += 1;
        }
        h - 1
    }
    pub fn query(&self, f: usize, s: usize) -> T {
        assert!(f <= s);
        let b = Self::top_bit(s + 1 - f);
        let endpoint = s + 1 - (1 << b);
        (self.biop)(self.st[b][f], self.st[b][endpoint])
    }
}


fn solve() {
    let n = get();
    let m = get();
    let a: Vec<Vec<i64>> = (0 .. n).map(|_|
                         (0 .. m).map(|_| get::<i64>()).collect())
        .collect();
    let k = get();
    // We use either O(n^2 * m) or O(n * log(n) + k * m) algorithm,
    // depending on m.
    if m >= 320 {
        // Use O(n^2 * m) one.
        let mut tbl: Vec<Vec<Vec<bool>>> = vec![Vec::new(); n];
        for i in 0 .. n {
            let mut ok = vec![true; m];
            for j in i + 1 .. n {
                for c in 0 .. m {
                    ok[c] &= a[j][c] >= a[j - 1][c];
                }
                tbl[i].push(ok.clone());
            }
        }
        for _ in 0 .. k {
            let l: usize = get();
            let r: usize = get();
            let mut ok = false;
            if l < r {
                for &b in tbl[l - 1][r - l - 1].iter() {
                    ok |= b;
                }
            } else {
                ok = true;
            }
            println!("{}", if ok { "Yes" } else { "No" });
        }
    } else {
        // Use O(n * log(n) + k * m) one.
        let mut spts = Vec::new();
        for i in 0 .. m {
            let mut c = Vec::new();
            for j in 0 .. n - 1 {
                c.push(a[j + 1][i] >= a[j][i]);
            }
            spts.push(SparseTable::<bool, _>::new(&c, |x, y| x && y));
        }
        for _ in 0 .. k {
            let l: usize = get();
            let r: usize = get();
            let mut ok = false;
            if l < r {
                for i in 0 .. m {
                    ok |= spts[i].query(l - 1, r - 2);
                }
            } else {
                ok = true;
            }
            println!("{}", if ok { "Yes" } else { "No" });
        }
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
