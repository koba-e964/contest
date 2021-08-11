use std::cmp::*;
use std::collections::*;
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
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input! {
        h: usize, w: usize,
        a: [[i64; w]; h],
    }
    let mut que = BinaryHeap::new();
    que.push((Reverse(0), 0, 0, 0));
    const INF: i64 = 1 << 30;
    let mut dist = vec![vec![vec![INF; h * w]; w]; h];
    let dxy = [(0i32, 1i32), (1, 0), (0, -1), (-1, 0)];
    while let Some((Reverse(d), x, y, c)) = que.pop() {
        if dist[x][y][c] <= d {
            continue;
        }
        dist[x][y][c] = d;
        for &(dx, dy) in &dxy {
            let nx = x.wrapping_add(dx as usize);
            let ny = y.wrapping_add(dy as usize);
            if nx >= h || ny >= w || c + 1 >= h * w {
                continue;
            }
            que.push((Reverse(d + a[nx][ny] * (2 * c as i64 + 1)), nx, ny, c + 1));
        }
    }
    println!("{}", dist[h - 1][w - 1].iter().min().unwrap());
}
