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

fn main() {
    input! {
        h: usize, w: usize,
        s: [chars; h],
    }
    let mut que = VecDeque::new();
    const INF: i32 = 1 << 28;
    let mut dist = vec![vec![INF; w]; h];
    que.push_back((0, 0, 0));
    let dxy = [(0i32, 1i32), (0, -1), (-1, 0), (1, 0)];
    while let Some((d, x, y)) = que.pop_front() {
        if dist[x][y] <= d { continue; }
        dist[x][y] = d;
        for &(dx, dy) in &dxy {
            let nx = (x as i32 + dx) as usize;
            let ny = (y as i32 + dy) as usize;
            if nx >= h || ny >= w { continue; }
            if s[nx][ny] == s[x][y] { continue; }
            que.push_back((d + 1, nx, ny));
        }
    }
    println!("{}", if dist[h - 1][w - 1] >= INF { -1 } else { dist[h - 1][w - 1] });
}
