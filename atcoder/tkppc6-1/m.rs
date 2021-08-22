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

// Solved with hints
// Tags: many-lcas, euler-tour
fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

fn euler_tour(v: usize, ch: &[Vec<usize>], cnt: &mut usize, rng: &mut [(usize, usize)]) {
    rng[v].0 = *cnt;
    *cnt += 1;
    for &w in &ch[v] {
        euler_tour(w, ch, cnt, rng);
    }
    rng[v].1 = *cnt;
}

fn dfs(v: usize, ch: &[Vec<usize>], ok: &[bool], a: &[i64], ma: &mut [i64], mut x: i64) {
    if ok[v] {
        x = max(x, a[v]);
    }
    ma[v] = x;
    for &w in &ch[v] {
        dfs(w, ch, ok, a, ma, x);
    }
}

fn solve() {
    input! {
        n: usize,
        p: [usize1; n - 1],
        a: [i64; n],
        m: usize,
        v: [usize1; m],
    }
    let mut ch = vec![vec![]; n];
    for i in 0..n - 1 {
        ch[p[i]].push(i + 1);
    }
    let mut rng = vec![(0, 0); n];
    let mut cnt = 0;
    euler_tour(0, &ch, &mut cnt, &mut rng);
    let mut acc = vec![0; n + 1];
    for &v in &v {
        acc[rng[v].0 + 1] += 1;
    }
    for i in 1..n + 1 {
        acc[i] += acc[i - 1];
    }
    let mut ok = vec![false; n];
    for i in 0..n {
        let v = rng[i].0;
        if acc[v] < acc[v + 1] {
            ok[i] = true;
            continue;
        }
        let mut c = 0;
        for &w in &ch[i] {
            let (st, en) = rng[w];
            if acc[st] < acc[en] {
                c += 1;
            }
        }
        if c >= 2 {
            ok[i] = true;
        }
    }
    let mut ma = vec![0; n];
    dfs(0, &ch, &ok, &a, &mut ma, 0);
    let mut tot = 0;
    for v in v {
        tot += ma[v];
    }
    println!("{}", tot);
}
