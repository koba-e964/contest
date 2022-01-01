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

// https://yukicoder.me/problems/no/861 (3)
// 2^25 通り全探索。
// -> TLE するので、BFS を使い連結な盤面をすべて生成することにした。
// -> TLE するので、BFS での連結性の判定を (初期局面に対する) bit 演算で高速化して AC。
fn main() {
    input!(c: [i64; 25]);
    let s: i64 = c.iter().sum();
    let mut mi = 1i64 << 50;
    let mut is_conn = vec![false; 1 << 25];
    let mut que = VecDeque::new();
    for i in 0..25 {
        que.push_back(1 << i);
    }
    while let Some(v) = que.pop_front() {
        if is_conn[v] { continue; }
        is_conn[v] = true;
        let mut adj = (v << 1) & 0x1ef7bde;
        adj |= (v >> 1) & 0xf7bdef;
        adj |= v << 5;
        adj |= v >> 5;
        adj &= !v;
        for i in 0..25 {
            if (adj & 1 << i) == 0 { continue; }
            let new = v | 1 << i;
            if is_conn[new] { continue; }
            que.push_back(new);
        }
    }
    for bits in 1..1 << 24 {
        if is_conn[bits] && is_conn[(1 << 25) - 1 - bits] {
            let mut tmp = s;
            for i in 0..25 {
                if (bits & 1 << i) != 0 {
                    tmp -= 2 * c[i];
                }
            }
            mi = min(mi, tmp.abs());
        }
    }
    println!("{}", mi);
}
