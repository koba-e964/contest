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

const INF: i32 = 100_000_000;

fn dfs(a: (i32, i32, i32), b: (i32, i32, i32), n: i32, r: i32,
       memo: &mut HashMap<((i32, i32, i32), (i32, i32, i32)), i32>,
       vis: &mut HashSet<((i32, i32, i32), (i32, i32, i32))>)
       -> Result<i32, ()> {
    if a.0 == 0 && a.1 == 0 {
        return Ok(-INF);
    }
    if let Some(&v) = memo.get(&(a, b)) {
        return Ok(v);
    }
    if vis.contains(&(a, b)) {
        return Err(());
    }
    vis.insert((a, b));
    let add = |x, p| if x + p >= n {
        if r == 0 {
            0
        } else {
            x + p - n
        }
    } else {
        x + p
    };
    let (ax, ay, am) = a;
    let (bx, by, bm) = b;
    let mut mi = INF + 1;
    if bx != 0 {
        if ax != 0 {
            mi.chmin(dfs((add(bx, ax), by, bm), a, n, r, memo, vis)?);
        }
        if ay != 0 {
            mi.chmin(dfs((add(bx, ay), by, bm), a, n, r, memo, vis)?);
        }
    }
    if by != 0 {
        if ax != 0 {
            mi.chmin(dfs((add(by, ax), bx, bm), a, n, r, memo, vis)?);
        }
        if ay != 0 {
            mi.chmin(dfs((add(by, ay), bx, bm), a, n, r, memo, vis)?);
        }
    }
    if ax == 0 && ay >= 2 && am >= 1 {
        for i in 1..ay {
            mi.chmin(dfs(b, (i, ay - i, am - 1), n, r, memo, vis)?);
        }
    }
    if ay == 0 && ax >= 2 && am >= 1 {
        for i in 1..ax {
            mi.chmin(dfs(b, (i, ax - i, am - 1), n, r, memo, vis)?);
        }
    }
    let sc = if mi > 0 {
        -mi + 1
    } else {
        -mi - 1
    };
    memo.insert((a, b), sc);
    Ok(sc)
}

// Tags: game, minimax
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: i32, m: i32, r: i32,
    }
    let mut memo = HashMap::new();
    let mut vis = HashSet::new();
    let result = dfs((1, 1, m), (1, 1, m), n, r, &mut memo, &mut vis);
    if result.is_err() {
        puts!("Infinite\n");
        return;
    }
    let result = result.unwrap();
    if result > 0 {
        puts!("First\n{}\n", INF - result);
    } else {
        puts!("Second\n{}\n", result + INF);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
