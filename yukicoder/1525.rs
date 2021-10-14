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

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut p = vec![0; n];
    for i in 0..n {
        p[a[i]] = i;
    }
    let mut tbl = vec![0; n + 2];
    tbl[0] = n as i64 * (n - 1) as i64 / 2;
    let mut mi = n;
    let mut ma = 0;
    for i in 0..n {
        mi = min(mi, p[i]);
        ma = max(ma, p[i]);
        tbl[i + 1] = (n - ma) as i64 * (mi + 1) as i64;
    }
    let mut ans = 0;
    for i in 1..n + 1 {
        ans += (tbl[i] - tbl[i + 1]) * i as i64;
    }
    println!("{}", ans);
}
