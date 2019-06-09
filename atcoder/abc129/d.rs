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
    let mut to = vec![vec![vec![0; w + 2]; h + 2]; 4];
    let start = [(0, 0), (0, 0), (0, w as i32 - 1), (h as i32 - 1, 0)];
    let step = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let side = [(1, 0), (0, 1), (1, 0), (0, 1)];
    let step_count = [w, h, w, h];
    let side_count = [h, w, h, w];
    for dir in 0..4 {
        let (sx, sy) = start[dir];
        let (stx, sty) = step[dir];
        let (sdx, sdy) = side[dir];
        for i in 0..side_count[dir] as i32 {
            for j in 0..step_count[dir] as i32 {
                let (x, y) = (sx + stx * j + sdx * i, sy + sty * j + sdy * i);
                let (bx, by) = (x - stx, y - sty);
                to[dir][(x + 1) as usize][(y + 1) as usize] =
                    if s[x as usize][y as usize] == '#' {
                        0
                    } else {
                        to[dir][(bx + 1) as usize][(by + 1) as usize] + 1
                    };
            }
        }
    }
    let mut ma = 0;
    for i in 0..h {
        for j in 0..w {
            let mut sum = 0;
            for dir in 0..4 {
                sum += to[dir][i + 1][j + 1];
            }
            ma = max(ma, sum);
        }
    }
    puts!("{}\n", ma - 3);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
