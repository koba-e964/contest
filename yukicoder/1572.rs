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
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// https://yukicoder.me/problems/no/1572 (3)
// (位置, 1 の向き) という状態それぞれに対して頂点を作って BFS。
fn main() {
    input! {
        h: usize, w: usize,
        sx: usize1, sy: usize1,
        gx: usize1, gy: usize1,
        a: [chars; h],
    }
    const INF: i64 = 1 << 28;
    let mut dist = vec![vec![[INF; 6]; w]; h];
    let mut que = VecDeque::new();
    que.push_back((0, sx, sy, 4));
    let dxy = [(0i32, 1i32), (-1, 0), (0, -1), (1, 0)];
    while let Some((d, x, y, z)) = que.pop_front() {
        if dist[x][y][z] <= d { continue; }
        dist[x][y][z] = d;
        for i in 0..4 {
            let nz = if z == 4 {
                i
            } else if z == 5 {
                (i + 2) % 4
            } else if z == i {
                5
            } else if z == (i + 2) % 4 {
                4
            } else {
                z
            };
            let (dx, dy) = dxy[i];
            let nx = (x as i32 + dx) as usize;
            let ny = (y as i32 + dy) as usize;
            if nx >= h || ny >= w {
                continue;
            }
            if a[nx][ny] == '#' { continue; }
            que.push_back((d + 1, nx, ny, nz));
        }
    }
    let ans = dist[gx][gy][4];
    println!("{}", if ans >= INF { -1 } else { ans });
}
