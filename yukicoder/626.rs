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
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
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

fn dfs(vw: &[(i64, i64)], pos: usize, mut vsum: i64, mut wsum: i64, lim: i64, acc: i64, ma: &mut i64) {
    if *ma >= acc + vsum {
        return;
    }
    if lim >= wsum {
        ma.chmax(acc + vsum);
        return;
    }
    ma.chmax(acc);
    if pos >= vw.len() {
        return;
    }
    let mut rel = acc;
    let mut rem = lim;
    for i in pos..vw.len() {
        let (v, w) = vw[i];
        if rem >= w {
            rel += v;
            rem -= w;
        } else {
            rel += ((v as i128 * rem as i128) / w as i128) as i64;
            break;
        }
    }
    if rel <= *ma {
        return;
    }
    let to = min(vw.len(), pos + 10);
    for i in pos..to {
        let (v, w) = vw[i];
        vsum -= v;
        wsum -= w;
        if lim >= w {
            dfs(vw, i + 1, vsum, wsum, lim - w, acc + v, ma);
        }
    }
    dfs(vw, to, vsum, wsum, lim, acc, ma);
}

// The author read the editorial before implementing this.
fn solve() {
    input! {
        n: usize, lim: i64,
        vw: [(i64, i64); n],
    }
    let mut vw = vw;
    vw.sort_by_key(|&(v, w)| -v * 1_000_000 / w);
    let mut ma = 0;
    let mut vsum = 0;
    let mut wsum = 0;
    for i in 0..n {
        vsum += vw[i].0;
        wsum += vw[i].1;
    }
    dfs(&vw, 0, vsum, wsum, lim, 0, &mut ma);
    println!("{}", ma);
}
