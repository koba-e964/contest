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
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// https://yukicoder.me/problems/no/2412 (3.5)
// 到達不能な条件は、
// # のマスを |dx| <= 3, |dy| <= 3, (|dx|, |dy|) != (3, 3) という条件で辺を設けたグラフで
// 左下と右上が連結であることと同値なはず。
// これは普通に距離を求める方法でできる。
// Solved with hints
// Tags: distance-on-grids, duality
fn main() {
    input! {
        h: usize, w: usize,
        s: [chars; h],
    }
    let mut s = s;
    for i in 0..h {
        s[i].insert(0, '#');
        s[i].push('#');
    }
    s.insert(0, vec!['#'; w + 2]);
    s.push(vec!['#'; w + 2]);
    const INF: i32 = 1 << 28;
    let mut que = BinaryHeap::new();
    let mut dist = vec![vec![INF; w + 2]; h + 2];
    for i in 4..w + 2 {
        que.push((Reverse(0), 0, i));
    }
    for i in 0..h - 2 {
        que.push((Reverse(0), i, w + 1));
    }
    while let Some((Reverse(d), x, y)) = que.pop() {
        if dist[x][y] <= d { continue; }
        dist[x][y] = d;
        for dx in -3i32..=3 {
            for dy in -3i32..=3 {
                if dx.abs() == 3 && dy.abs() == 3 {
                    continue;
                }
                let nx = (x as i32 + dx) as usize;
                let ny = (y as i32 + dy) as usize;
                if nx >= h + 2 || ny >= w + 2 {
                    continue;
                }
                if nx <= 3 && ny <= 3 {
                    continue;
                }
                if nx >= h - 2 && ny >= w - 2 {
                    continue;
                }
                que.push((Reverse(d + if s[nx][ny] == '#' { 0 } else { 1 }), nx, ny));
            }
        }
    }
    let mut ans = INF;
    for i in 4..h + 2 {
        ans = min(ans, dist[i][0]);
    }
    for i in 0..w - 2 {
        ans = min(ans, dist[h + 1][i]);
    }
    println!("{}", ans);
}
