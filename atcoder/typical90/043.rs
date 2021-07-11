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

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
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
        h: usize, w: usize,
        rs: usize1, cs: usize1,
        rt: usize1, ct: usize1,
        s: [chars; h],
    }
    let mut que = VecDeque::new();
    const INF: i32 = 1 << 30;
    let mut dist = vec![vec![[INF; 4]; w]; h];
    for i in 0..4 {
        que.push_back((0, rs, cs, i));
    }
    let dxy = [(1i32, 0i32), (0, 1), (-1, 0), (0, -1)];
    while let Some((d, x, y, cdir)) = que.pop_front() {
        if dist[x][y][cdir] <= d {
            continue;
        }
        if s[x][y] == '#' {
            continue;
        }
        dist[x][y][cdir] = d;
        for dir in 0..4 {
            let (dx, dy) = dxy[dir];
            let nx = x.wrapping_add(dx as usize);
            let ny = y.wrapping_add(dy as usize);
            if nx >= h || ny >= w {
                continue;
            }
            if cdir == dir {
                if dist[nx][ny][dir] > d {
                    que.push_front((d, nx, ny, dir));
                }
            } else {
                if dist[nx][ny][dir] > d + 1 {
                    que.push_back((d + 1, nx, ny, dir));
                }
            }
        }
    }
    puts!("{}\n", dist[rt][ct].iter().min().unwrap());
}
