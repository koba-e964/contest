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
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        (0..len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    }};

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
}

fn dfs(v: usize, par: usize, target: usize, g: &[Vec<(usize, usize)>]) -> i64 {
    let mut ans = 0;
    let mut found = false;
    if target == v {
        return 0;
    }
    for &(w, idx) in &g[v] {
        if par == w { continue; }
        let sub = dfs(w, v, target, g);
        if sub >= 0 {
            ans |= sub | 1 << idx;
            found = true;
        }
    }
    if found { ans } else { -1 }
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize,
        ab: [(usize1, usize1); n - 1],
        m: usize,
        uv: [(usize1, usize1); m],
    }
    let mut g = vec![vec![]; n];
    for (i, &(a, b)) in ab.iter().enumerate() {
        g[a].push((b, i));
        g[b].push((a, i));
    }
    let mut tot: i64 = 0;
    let mut filled = vec![0; m];
    for i in 0..m {
        let (u, v) = uv[i];
        filled[i] = dfs(u, n, v, &g);
    }
    for bits in 0usize..1 << m {
        let mut wh = 0;
        for i in 0..m {
            if (bits & 1 << i) != 0 {
                wh |= filled[i];
            }
        }
        let bc = bits.count_ones();
        let whc = wh.count_ones() as usize;
        tot += if bc % 2 == 0 { 1 << (n - 1 - whc) } else { -1 << (n - 1 - whc) };
    }
    puts!("{}\n", tot);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
