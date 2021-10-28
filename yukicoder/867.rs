use std::cmp::*;
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
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn find_dist(a: &[Vec<i32>], gx: usize, gy: usize, k: i32) -> Vec<Vec<i32>> {
    let n = a.len();
    let m = a[0].len();
    const INF: i32 = 1 << 30;
    let mut que = BinaryHeap::new();
    que.push((Reverse(0), gx, gy));
    let mut dist = vec![vec![INF; m]; n];
    let dxy = [(0i32, 1i32), (1, 0), (0, -1), (-1, 0)];
    while let Some((Reverse(d), x, y)) = que.pop() {
        if dist[x][y] <= d {
            continue;
        }
        dist[x][y] = d;
        for &(dx, dy) in &dxy {
            let nx = (x as i32 + dx) as usize;
            let ny = (y as i32 + dy) as usize;
            if nx >= n || ny >= m { continue; }
            let nd = d + k + a[nx][ny];
            if dist[nx][ny] > nd {
                if dist[nx][ny] < nd - 1 {
                    dist[nx][ny] = nd - 1;
                }
                que.push((Reverse(nd), nx, ny));
            }
        }
    }
    dist
}

// Tags: exploit-small-constraints
// In this problem K is essentially at most 710
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    const KLIM: usize = 231;
    input! {
        h: usize, w: usize,
        gx: usize1, gy: usize1,
        a: [[i32; w]; h],
        q: usize,
        xyk: [(usize1, usize1, i64); q],
    }
    let mut dist = vec![vec![]; KLIM];
    for i in 0..KLIM {
        dist[i] = find_dist(&a, gx, gy, i as i32 * i as i32);
    }
    for (x, y, k) in xyk {
        let d;
        if k >= KLIM as i64 {
            let tmp = dist[KLIM - 1][x][y];
            let div = (KLIM - 1) as i32 * (KLIM - 1) as i32;
            let q = tmp / div;
            let r = tmp % div;
            d = q as i64 * k * k + r as i64;
            
        } else {
            d = dist[k as usize][x][y] as i64;
        }
        puts!("{}\n", d + k * k + a[gx][gy] as i64);
    }
}
