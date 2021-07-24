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

fn dfs(k: usize, p: u64, st: usize, s: &[Vec<char>],
       seen: &mut HashSet<u64>) {
    let n = s.len();
    if k == 0 {
        seen.insert(p);
        return;
    }
    for idx in st..n * n {
        let i = idx / n;
        let j = idx % n;
        if s[i][j] == '#' {
            continue;
        }
        let v = 1u64 << idx;
        if (p & v) != 0 {
            continue;
        }
        let mut mask = v;
        if i > 0 {
            mask |= v >> n;
        }
        if i < n - 1 {
            mask |= v << n;
        }
        if j > 0 {
            mask |= v >> 1;
        }
        if j < n - 1 {
            mask |= v << 1;
        }
        if (mask & p) != 0 {
            dfs(k - 1, p | v, st, s, seen);
        }
    }
}

fn main() {
    input! {
        n: usize,
        k: usize,
        s: [chars; n],
    }
    let mut seen = HashSet::new();
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == '#' {
                continue;
            }
            dfs(k - 1, 1 << (i * n + j), i * n + j + 1, &s, &mut seen);
        }
    }
    println!("{}", seen.len());
}
