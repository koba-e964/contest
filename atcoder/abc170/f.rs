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

// The author read the editorial.
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        h: usize, w: usize, k: i64,
        xy1: (usize1, usize1),
        xy2: (usize1, usize1),
        c: [chars; h],
    }
    let mut que = BinaryHeap::new();
    que.push((0, xy1.0, xy1.1, 0));
    que.push((0, xy1.0, xy1.1, 1));
    const INF: i64 = 1 << 50;
    let mut dist = vec![vec![[INF; 2]; w]; h];
    while let Some((d, x, y, kind)) = que.pop() {
        let d = -d;
        if dist[x][y][kind] <= d {
            continue;
        }
        dist[x][y][kind] = d;
        let dir = [[(0i32, 1i32), (0, -1)], [(1, 0), (-1, 0)]][kind];
        for &(dx, dy) in &dir {
            let nx = x.wrapping_add(dx as usize);
            let ny = y.wrapping_add(dy as usize);
            if nx >= h || ny >= w { continue; }
            if c[nx][ny] == '@' { continue; }
            que.push((-(d + 1), nx, ny, kind));
        }
        que.push(((d + k - 1) / k * (-k), x, y, 1 - kind));
    }
    let &s = dist[xy2.0][xy2.1].iter().min().unwrap();
    puts!("{}\n", if s >= INF { -1 } else { (s + k - 1) / k });
}
