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
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input! {
        h: usize, w: usize, x: usize1, y: usize1,
        a: [[i64; w]; h],
    }
    let mut s = a[x][y];
    let mut vis = vec![vec![false; w]; h];
    let mut que = BinaryHeap::new();
    que.push((Reverse(0), x, y));
    let dxy = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut rem = h * w;
    while let Some((Reverse(st), x, y)) = que.pop() {
        if vis[x][y] { continue; }
        if s <= st {
            break;
        }
        vis[x][y] = true;
        rem -= 1;
        s += st;
        for &(dx, dy) in &dxy {
            let nx = (x as i32 + dx) as usize;
            let ny = (y as i32 + dy) as usize;
            if nx >= h || ny >= w { continue; }
            if vis[nx][ny] { continue; }
            que.push((Reverse(a[nx][ny]), nx, ny));
        }
    }
    println!("{}", if rem > 0 { "No" } else { "Yes" });
}
