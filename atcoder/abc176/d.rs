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
    input! {
        h: usize, w: usize,
        ch: usize1, cw: usize1,
        dh: usize1, dw: usize1,
        s: [chars; h],
    }
    let mut que = VecDeque::new();
    const INF: i32 = 1 << 28;
    let mut dist = vec![vec![INF; w]; h];
    que.push_back((0, ch, cw));
    while let Some((d, x, y)) = que.pop_front() {
        if dist[x][y] <= d {
            continue;
        }
        dist[x][y] = d;
        for dx in -1i32..2 {
            for dy in -1i32..2 {
                if (dx + dy) % 2 == 0 { continue; } 
                let nx = x.wrapping_add(dx as usize);
                let ny = y.wrapping_add(dy as usize);
                if nx >= h || ny >= w { continue; }
                if s[nx][ny] == '#' { continue; }
                if dist[nx][ny] <= d { continue; }
                que.push_front((d, nx, ny));
            }
        }
        for dx in -2i32..3 {
            for dy in -2i32..3 {
                let nx = x.wrapping_add(dx as usize);
                let ny = y.wrapping_add(dy as usize);
                if nx >= h || ny >= w { continue; }
                if s[nx][ny] == '#' { continue; }
                if dist[nx][ny] <= d + 1 { continue; }
                que.push_back((d + 1, nx, ny));
            }
        }
    }
    puts!("{}\n", if dist[dh][dw] >= INF { -1 } else { dist[dh][dw] });
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
