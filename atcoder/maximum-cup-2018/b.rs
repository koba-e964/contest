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

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        a: usize, b: usize,
        h: usize, w: usize,
        c: [chars; h],
    }
    let dir = [(0i32, 1i32), (1, 0), (0, -1), (-1, 0)];
    let mut que = VecDeque::new();
    que.push_back((0, 1usize, 1usize, 1, a, b));
    let mut dist = HashMap::new();
    while let Some((d, x, y, ori, a, b)) = que.pop_front() {
        if let Some(&cd) = dist.get(&(x, y, ori, a, b)) {
            if cd <= d {
                continue;
            }
        }
        dist.insert((x, y, ori, a, b), d);
        let nx = x.wrapping_add(dir[ori].0 as usize);
        let ny = y.wrapping_add(dir[ori].1 as usize);
        if c[nx][ny] == '.' {
            que.push_back((d + 1, nx, ny, ori, a, b));
        }
        if a > 0 {
            let idx = (ori + 3) % 4;
            let nx = x.wrapping_add(dir[idx].0 as usize);
            let ny = y.wrapping_add(dir[idx].1 as usize);
            if c[nx][ny] == '.' {
                que.push_back((d + 1, nx, ny, idx, a - 1, b));
            }
        }
        if b > 0 {
            let idx = (ori + 1) % 4;
            let nx = x.wrapping_add(dir[idx].0 as usize);
            let ny = y.wrapping_add(dir[idx].1 as usize);
            if c[nx][ny] == '.' {
                que.push_back((d + 1, nx, ny, idx, a, b - 1));
            }
        }
    }
    let mut ok = false;
    for ((x, y, _, a, b), _) in dist {
        if x == h - 2 && y == w - 2 && a == 0 && b == 0 {
            ok = true;
        }
    }
    puts!("{}\n", if ok { "Yes" } else { "No" });
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
