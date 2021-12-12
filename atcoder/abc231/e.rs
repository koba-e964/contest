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

fn dfs(x: i64, v: usize, a: &[i64], memo: &mut HashMap<(i64, usize), i64>) -> i64 {
    let key = (x, v);
    if let Some(&val) = memo.get(&key) {
        return val;
    }
    let n = a.len();
    let q = x / a[v];
    if v + 1 < n {
        let r = q % (a[v + 1] / a[v]);
        let sub = dfs(x - r * a[v], v + 1, a, memo);
        let mut mi = sub + r;
        if r > 0 {
            let t = a[v + 1] / a[v] - r;
            let sub = dfs(x + t * a[v], v + 1, a, memo);
            mi = min(mi, sub + t);
        }
        memo.insert(key, mi);
        return mi;
    }
    x / a[v]
}

fn main() {
    input! {
        n: usize, x: i64,
        a: [i64; n],
    }
    let mut memo = HashMap::new();
    println!("{}", dfs(x, 0, &a, &mut memo));
}
