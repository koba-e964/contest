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
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn dist(s: &[Vec<char>], st: char, en: char) -> i64 {
    let n = s.len();
    let m = s[0].len();
    let mut dist = vec![1 << 30; n * m];
    let mut que = VecDeque::new();
    for i in 0..n {
        for j in 0..m {
            if s[i][j] == st {
                que.push_back((0, i * m + j));
            }
        }
    }
    let dxy = [(0i32, -1i32), (1, 0), (0, 1), (-1, 0)];
    while let Some((d, v)) = que.pop_front() {
        if dist[v] <= d {
            continue;
        }
        let x = v / m;
        let y = v % m;
        if s[x][y] == 'X' {
            continue;
        }
        dist[v] = d;
        if s[x][y] == en {
            return d;
        }
        for &(dx, dy) in &dxy {
            let nx = x.wrapping_add(dx as usize);
            let ny = y.wrapping_add(dy as usize);
            if nx >= n || ny >= m {
                continue;
            }
            que.push_back((d + 1, nx * m + ny));
        }
    }
    panic!();
}

fn main() {
    input! {
        h: usize, _w: usize, n: u8,
        s: [chars; h],
    }
    let mut tot = 0;
    for i in 0..n {
        let st = if i == 0 { 'S' } else { (b'0' + i) as char };
        let en = (b'0' + i + 1) as char;
        tot += dist(&s, st, en);
    }
    println!("{}", tot);
}
