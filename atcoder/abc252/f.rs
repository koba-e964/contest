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

// Solved with hints
// Tags: huffman-coding, greedy-algorithm
fn main() {
    input! {
        n: usize, l: i64,
        a: [i64; n],
    }
    let mut a = a;
    let s: i64 = a.iter().sum();
    if s < l {
        a.push(l - s);
    }
    let mut que = BinaryHeap::<i64>::new();
    que.extend(a.iter().map(|a| -a));
    let mut ans = 0;
    while let Some(v1) = que.pop() {
        if let Some(v2) = que.pop() {
            ans += v1 + v2;
            que.push(v1 + v2);
            continue;
        }
    }
    println!("{}", -ans);
}
