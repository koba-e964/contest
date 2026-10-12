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
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

fn dfs1(v: usize, par: usize, g: &[Vec<(usize, i64)>], wei: &mut [i64]) -> usize {
    let n = g.len();
    let mut nv = 1;
    for &(w, ewei) in &g[v] {
        if w == par { continue; }
        let sub = dfs1(w, v, g, wei);
        nv += sub;
        if 2 * sub < n {
            wei[w] -= ewei;
            wei[0] += ewei;
        } else {
            wei[w] += ewei;
        }
    }
    nv
}

fn dfs2(v: usize, par: usize, g: &[Vec<(usize, i64)>], wei: &mut [i64], pro: i64) {
    wei[v] += pro;
    for &(w, _) in &g[v] {
        if w == par { continue; }
        dfs2(w, v, g, wei, wei[v]);
    }
}


fn solve() {
    #[allow(unused)]
    let out = std::io::stdout();
    #[allow(unused)]
    let mut out = BufWriter::new(out.lock());
    #[allow(unused)]
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    #[allow(unused)]
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    input! {
        n: usize,
        ab: [(usize1, usize1); n - 1],
    }
    let mut g = vec![vec![]; n];
    for i in 0..n - 1 {
        let (a, b) = ab[i];
        g[a].push((b, i as i64 + 1));
        g[b].push((a, i as i64 + 1));
    }
    let mut w = vec![0; n];
    dfs1(0, n, &g, &mut w);
    dfs2(0, n, &g, &mut w, 0);
    putvec!(w);
}
