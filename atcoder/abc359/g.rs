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
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

type Dat = (i64 /* depth_delta */, HashMap<i32, (i64, i64)>);

fn unite(mut a: Dat, mut b: Dat) -> (i64, Dat) {
    if a.1.len() < b.1.len() {
        std::mem::swap(&mut a, &mut b);
    }
    let (ad, mut a) = a;
    let (bd, b) = b;
    let mut sum = 0;
    for (k, v) in b {
        let ent = a.entry(k).or_insert((0, 0));
        sum += v.0 * (ent.1 + ad * ent.0);
        sum += ent.0 * (v.1 + bd * v.0);
        ent.0 += v.0;
        ent.1 += v.1 + (bd - ad) * v.0;
    }
    (sum, (ad, a))
}

fn dfs(v: usize, par: usize, g: &[Vec<usize>], a: &[i32]) -> (i64, Dat) {
    let mut hm = HashMap::new();
    hm.insert(a[v], (1, 0));
    let mut hm = (0, hm);
    let mut sum = 0;
    for &w in &g[v] {
        if w == par {
            continue;
        }
        let (subsum, mut sub) = dfs(w, v, g, a);
        sum += subsum;
        sub.0 += 1;
        let (subsum, newhm) = unite(hm, sub);
        hm = newhm;
        sum += subsum;
    }
    (sum, hm)
}

fn solve() {
    input! {
        n: usize,
        uv: [(usize1, usize1); n - 1],
        a: [i32; n],
    }
    let mut g = vec![vec![]; n];
    for &(u, v) in &uv {
        g[u].push(v);
        g[v].push(u);
    }
    let (sum, _) = dfs(0, n, &g, &a);
    println!("{}", sum);
}
