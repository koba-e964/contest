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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn rec(n: i64, m: i64, memo: &mut HashMap<(i64, i64), i64>) -> i64 {
    if n == 0 {
        return 0;
    }
    let key = (n, m);
    if let Some(&val) = memo.get(&key) {
        return val;
    }
    let mut c = 1;
    while c < m {
        c *= 2;
    }
    if n / c != (n + m - 1) / c {
        let q = (n + c - 1) / c;
        let r = q * c - n;
        let excess = rec(r, m - r, memo);
        let x = rec(n, r, memo) + if q.count_ones() % 2 == 1 {
            m - r - excess
        } else {
            excess
        };
        memo.insert(key, x);
        return x;
    }
    if n >= c {
        let x = rec(n % c, m, memo);
        let x = if (n / c).count_ones() % 2 == 1 {
            m - x
        } else {
            x
        };
        memo.insert(key, x);
        return x;
    }
    assert!(c >= 2);
    let x = rec(n, c / 2, memo) + rec(n, m - c / 2, memo);
    memo.insert(key, x);
    x
}

fn calc(n: i64, m: i64) -> i64 {
    let mut memo = HashMap::new();
    rec(n, m, &mut memo)
}

fn main() {
    input! {
        t: usize,
        nm: [(i64, i64); t],
    }
    for (n, m) in nm {
        println!("{}", calc(n, m));
    }
}
