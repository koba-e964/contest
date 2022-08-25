#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
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
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
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

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

// [2, 4, 0, 1, 3, 7, 6] ==> [[0, 2], [1, 4, 3], [6, 7]]
// Verified by: https://atcoder.jp/contests/joisc2007/submissions/24248388
fn decompose_into_cycles(a: &[usize]) -> Vec<Vec<usize>> {
    let n = a.len();
    let mut vis = vec![false; n];
    let mut ans = vec![];
    for i in 0..n {
        if vis[i] { continue; }
        vis[i] = true;
        let mut cyc = vec![i];
        let mut v = a[i];
        while v != i {
            vis[v] = true;
            cyc.push(v);
            v = a[v];
        }
        ans.push(cyc)
    }
    ans
}

fn solve() {
    input! {
        n: usize, m: usize,
        p: [usize1; n + m],
    }
    let cycs = decompose_into_cycles(&p);
    let mut homo_b = vec![];
    let mut homo_c = vec![];
    let mut hetero = vec![];
    for a in cycs {
        if a.len() == 1 {
            continue;
        }
        let mut b = 0;
        let mut c = 0;
        for &v in &a {
            if v < n {
                b += 1;
            } else {
                c += 1;
            }
        }
        if b > 0 && c > 0 {
            hetero.push(a.len());
        } else {
            if b > 0 {
                homo_b.push(a.len());
            } else {
                homo_c.push(a.len());
            }
        }
    }
    if false {
        eprintln!("homo_b = {:?}", homo_b);
        eprintln!("homo_c = {:?}", homo_c);
        eprintln!("hetero = {:?}", hetero);
    }
    let mut ans = 0;
    for &h in &homo_b {
        ans += h + 1;
    }
    for &h in &homo_c {
        ans += h + 1;
    }
    ans -= 2 * min(homo_b.len(), homo_c.len());
    for h in hetero {
        ans += h - 1;
    }
    println!("{}", ans);
}
