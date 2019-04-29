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

fn make_table() -> Vec<usize> {
    let mut dp = vec![0; 1 << 16];
    for pc in 1..16 + 1 {
        for bits in 0usize..1 << 16 {
            if (bits & 1) == 1 || bits.count_ones() != pc { continue; }
            let mut mi = 1 << 16;
            for i in 0..16 {
                if (bits & 1 << i) == 0 { continue; }
                mi = min(mi, dp[bits ^ 1 << i] + 1);
                for j in 0..16 {
                    if i == j || (bits & 1 << j) == 0 { continue; }
                    let newbits = bits ^ 1 << i ^ 1 << j ^ 1 << (i ^ j);
                    let app = if (newbits & 1 << (i ^ j)) == 0 { 1 } else { 0 };
                    mi = min(mi, dp[newbits] + 1 + app);
                }
            }
            dp[bits] = mi;
        }
    }
    dp
}

fn dfs(g: &[Vec<(usize, usize)>], v: usize, par: usize,
       b: &mut [usize], mut acc: usize) {
    for &(w, d) in &g[v] {
        if w == par { continue; }
        dfs(g, w, v, b, d);
        acc ^= d;
    }
    b[v] = acc;
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize,
        xya: [(usize, usize, usize); n - 1],
    }
    let tbl = make_table();
    /*
    for i in (0..16) {
        let i = 2 * i;
        for j in 0..5 {
            if (i & 1 << j) != 0 {
                eprint!("{} ", j);
            }
        }
        eprintln!(": {}", tbl[i]);
    }*/
    let mut g = vec![vec![]; n];
    for &(x, y, a) in &xya {
        g[x].push((y, a));
        g[y].push((x, a));
    }
    let mut b = vec![0; n];
    dfs(&g, 0, n, &mut b, 0);
    b[0] = 0;
    let mut freq = [0; 16];
    for &b in &b {
        freq[b] += 1;
    }
    // eprintln!("b = {:?}", b);
    let mut tot = 0;
    let mut rem = 0;
    for i in 1..16 {
        tot += freq[i] / 2;
        if freq[i] % 2 == 1 {
            rem |= 1 << i;
        }
    }
    tot += tbl[rem];
    puts!("{}\n", tot);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
