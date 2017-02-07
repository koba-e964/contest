#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::*;
#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok();
    return ret;
}
fn get_word() -> String {
    let mut stdin = std::io::stdin();
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(16);
        loop {
            let res = stdin.read(&mut u8b);
            if res.is_err() || res.ok().unwrap() == 0 || u8b[0] <= ' ' as u8 {
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
fn parse<T: std::str::FromStr>(s: &str) -> T { s.parse::<T>().ok().unwrap() }

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { parse(&get_word()) }

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


fn solve(a: &[(i32, usize)]) -> i32 {
    let n = a.len();
    let mut dp = vec![vec![0i32; n]; n];
    let mut dp_sparse = Vec::<SparseTable<i32, _>>::new();
    for i in 0 .. n {
        dp[i][a[i].1] = 1;
        for j in 0 .. i {
            if a[j].0 == a[i].0 { continue; }
            // >= a[j].1
            let mut ret = 1;
            if a[i].1 > a[j].1 {
                ret = max(ret, dp_sparse[j].query(a[j].1, a[i].1 - 1) + 1);
            } else {
                // <= a[j].1
                ret = max(ret, dp_sparse[j].query(a[i].1 + 1, a[j].1) + 1);
            }
            dp[i][a[j].1] = ret;
        }
        dp_sparse.push(SparseTable::new(&dp[i], |x, y| max(x, y)));
    }
    *dp.iter().map(|w| w.iter().max().unwrap()).max().unwrap()
}

fn main() {
    let n = get();
    let mut a: Vec<(i32, usize)> = (0 .. n).map(|i| (get(), i)).collect();
    a.sort();
    a.reverse();
    let t2 = solve(&a);
    println!("{}", t2);
}
