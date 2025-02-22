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
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    input! {
        n: usize,
        a: [chars; n],
    }
    let mut que = VecDeque::new();
    for i in 0..n {
        que.push_back((0, i, i, '-'));
    }
    for i in 0..n {
        for j in 0..n {
            if a[i][j] != '-' {
                que.push_back((1, i, j, '-'));
            }
        }
    }
    const INF: i32 = 1 << 28;
    let mut dist = vec![vec![INF; n]; n];
    let mut dist_half = vec![vec![[INF; 26]; n]; n];
    while let Some((d, x, y, me)) = que.pop_front() {
        if me == '-' {
            if dist[x][y] <= d {
                continue;
            }
            dist[x][y] = d;
            for j in 0..n {
                if a[j][x] != '-' {
                    que.push_back((d + 1, j, y, a[j][x]));
                }
            }
        } else {
            let idx = me as usize - 'a' as usize;
            if dist_half[x][y][idx] <= d {
                continue;
            }
            dist_half[x][y][idx] = d;
            for j in 0..n {
                if a[y][j] == me {
                    que.push_back((d + 1, x, j, '-'));
                }
            }
        }
    }
    for i in 0..n {
        for v in &mut dist[i] {
            if *v == INF {
                *v = -1;
            }
        }
        putvec!(dist[i]);
    }
}
