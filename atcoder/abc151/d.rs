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

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        h: usize, w: usize,
        s: [chars; h],
    }
    const INF: i64 = 1 << 50;
    let mut dist = vec![vec![INF; h * w]; h * w];
    let dxy = [(0, -1), (0, 1), (1, 0), (-1, 0)];
    let mut plain = vec![false; h * w];
    for i in 0..h {
        for j in 0..w {
            let v = i * w + j;
            if s[i][j] == '#' {
                continue;
            }
            plain[v] = true;
            for d in 0..4 {
                let (dx, dy) = dxy[d];
                let nx = i.wrapping_add(dx as usize);
                let ny = j.wrapping_add(dy as usize);
                if nx >= h || ny >= w {
                    continue;
                }
                let nx = nx as usize;
                let ny = ny as usize;
                let nv = nx * w + ny;
                if s[nx][ny] == '.' {
                    dist[v][nv] = 1;
                }
            }
        }
    }
    for k in 0..h * w {
        for i in 0..h * w {
            for j in 0..h * w {
                dist[i][j] = min(dist[i][j], dist[i][k] + dist[k][j]);
            }
        }
    }
    let mut ma = 0;
    for i in 0..h * w {
        if !plain[i] { continue; }
        for j in 0..i {
            if !plain[j] { continue; }
            if dist[i][j] < INF {
                ma = max(ma, dist[i][j]);
            }
        }
    }
    puts!("{}\n", ma);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
