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
        h: usize, w: usize, x: usize,
        s: [chars; h],
    }
    let neigh = |(x, y): (usize, usize)| {
        let d = [(-1, 0), (0, 1), (1, 0), (0, -1)];
        let mut ans = vec![];
        for dir in 0..4 {
            let (dx, dy) = d[dir];
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx < 0 || nx >= h as i32 || ny < 0 || ny >= w as i32 {
                continue;
            }
            ans.push((nx as usize, ny as usize));
        }
        ans
    };
    let mut a = (0, 0);
    let mut b = (0, 0);
    let mut boars = vec![];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == 'S' {
                a = (i, j);
            }
            if s[i][j] == 'G' {
                b = (i, j);
            }
            if s[i][j] == '@' {
                boars.push((i, j));
            }
        }
    }
    const INF: usize = 1 << 29;
    let mut safety = vec![vec![INF; w]; h];
    let mut que: VecDeque<_> = boars.into_iter().map(|x| (0, x)).collect();
    while let Some((d, v)) = que.pop_front() {
        if s[v.0][v.1] == '#' {
            continue;
        }
        if safety[v.0][v.1] <= d {
            continue;
        }
        safety[v.0][v.1] = d;
        for w in neigh(v) {
            que.push_back((d + 1, w));
        }
    }
    let mut dist = vec![vec![INF; w]; h];
    let mut que = VecDeque::new();
    que.push_back((0, a));
    while let Some((d, v)) = que.pop_front() {
        if s[v.0][v.1] == '#' {
            continue;
        }
        if safety[v.0][v.1] <= x {
            continue;
        }
        if dist[v.0][v.1] <= d {
            continue;
        }
        dist[v.0][v.1] = d;
        for w in neigh(v) {
            que.push_back((d + 1, w));
        }
    }
    let ans = dist[b.0][b.1];
    if ans >= INF {
        puts!("-1\n");
    } else {
        puts!("{}\n", ans);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
