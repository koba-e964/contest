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
    ($next:expr, ) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, [graph1; $len:expr]) => {{
        let mut g = vec![vec![]; $len];
        let ab = read_value!($next, [(usize1, usize1)]);
        for (a, b) in ab {
            g[a].push(b);
            g[b].push(a);
        }
        g
    }};
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };
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

trait Change {
    fn chmax(&mut self, x: Self);
    fn chmin(&mut self, x: Self);
}
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) {
        if *self < x { *self = x; }
    }
    fn chmin(&mut self, x: T) {
        if *self > x { *self = x; }
    }
}

#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    #[allow(unused)]
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    input! {
        h: usize, w: usize,
        s: [chars; h],
    }
    let mut adj = vec![0; 1 << (h * w)];
    for i in 0..h {
        for j in 0..w {
            let v1 = i * w + j;
            if j > 0 {
                let v2 = i * w + j - 1;
                adj[v1] |= 1 << v2;
                adj[v2] |= 1 << v1;
            }
            if i > 0 {
                let v2 = i * w - w + j;
                adj[v1] |= 1 << v2;
                adj[v2] |= 1 << v1;
            }
        }
    }
    let mut dp = vec![vec![None::<usize>; 1 << (h * w)]; h * w];
    for i in 0..h * w {
        dp[i][1 << i] = Some(i);
    }
    for bits in 0..1 << (h * w) {
        for i in 0..h * w {
            if (bits & 1 << i) == 0 || dp[i][bits].is_none() {
                continue;
            }
            for j in 0..h * w {
                if (bits & 1 << j) != 0 || (adj[i] & 1 << j) == 0 {
                    continue;
                }
                dp[j][bits | 1 << j] = Some(i);
            }
        }
    }
    let mut cur = 0;
    let mut idx = h * w;
    let mut path = vec![];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                cur |= 1 << (i * w + j);
            }
        }
    }
    'outer:
    for i in 0..h * w {
        if dp[i][cur].is_some() {
            idx = i;
            break 'outer;
        }
    }
    while let Some(pre) = dp[idx][cur] {
        // eprintln!("{} {} -> {}", idx, cur, pre);
        path.push((idx / w, idx % w));
        if idx == pre {
            break;
        }
        cur ^= 1 << idx;
        idx = pre;
    }
    puts!("{}\n", path.len());
    for (x, y) in path {
        puts!("{} {}\n", x + 1, y + 1);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
