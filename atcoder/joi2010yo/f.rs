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

fn inb(x: (i64, i64), y: (i64, i64), z: (i64, i64)) -> bool {
    fn sub(x: i64, y: i64, z: i64) -> bool {
        (x - y) * (z - y) < 0
    }
    if x.0 == z.0 {
        x.0 == y.0 && sub(x.1, y.1, z.1)
    } else {
        x.1 == y.1 && sub(x.0, y.0, z.0)
    }
}

fn dfs(rem: u32, pos: usize, a: &[(i64, i64)], inb_tbl: &[Vec<u32>],
       memo: &mut HashMap<(u32, usize), i64>) -> i64 {
    let key = (rem, pos);
    if let Some(&v) = memo.get(&key) {
        return v;
    }
    if rem == 0 {
        return if pos == 0 { 1 } else { 0 };
    }
    let mut tot = 0;
    let n = a.len();
    for i in 0..n {
        if pos == i ||
            (rem & 1 << i) == 0 || (a[i].0 != a[pos].0 && a[i].1 != a[pos].1) {
            continue;
        }
        let ok = (rem & inb_tbl[i][pos]) == 0;
        if ok && ((rem ^ 1 << i) == 0 || i != 0) {
            tot += dfs(rem ^ 1 << i, i, a, inb_tbl, memo);
        }
    }
    memo.insert(key, tot);
    tot
}

// Tags: memoization, bit-dp
fn solve() {
    input! {
        m: usize, n: usize,
        a: [[i32; m]; n],
    }
    let mut ch = (-1, -1);
    let mut b = vec![];
    for i in 0..n {
        for j in 0..m {
            if a[i][j] == 1 {
                b.push((i as i64, j as i64));
            }
            if a[i][j] == 2 {
                ch = (i as i64, j as i64);
            }
        }
    }
    b.insert(0, ch);
    let kk = b.len();
    let mut inb_tbl = vec![vec![0; kk]; kk];
    for i in 0..kk {
        for j in 0..kk {
            for k in 1..kk {
                if i != k && j != k && inb(b[i], b[k], b[j]) {
                    inb_tbl[i][j] |= 1 << k;
                }
            }
        }
    }
    let mut memo = HashMap::new();
    println!("{}", dfs((1 << b.len()) - 1, 0, &b, &inb_tbl, &mut memo));
}
