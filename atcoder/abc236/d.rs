use std::cmp::*;
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

fn dfs(used: usize, xor: i64, b: &[Vec<i64>]) -> i64 {
    let n = b.len();
    if used == (1 << n) - 1 {
        return xor;
    }
    let mut ans = 0;
    let mut seen = false;
    for i in 0..n {
        if (used & 1 << i) != 0 { continue; }
        if seen { break; }
        seen = true;
        for j in i + 1..n {
            if (used & 1 << j) == 0 {
                let sub = dfs(used | 1 << i | 1 << j, xor ^ b[i][j], b);
                ans = max(ans, sub);
            }
        }
    }
    ans
}

fn main() {
    input! {
        n: usize,
        a: [i64; n * (2 * n - 1)],
    }
    let mut b = vec![vec![0; 2 * n]; 2 * n];
    let mut pos = 0;
    for i in 0..2 * n {
        for j in i + 1..2 * n {
            b[i][j] = a[pos];
            b[j][i] = a[pos];
            pos += 1;
        }
    }
    let ma = dfs(0, 0, &b);
    println!("{}", ma);
}
