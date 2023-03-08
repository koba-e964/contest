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
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input! {
        n: i64, m: usize, k: i64,
        ab: [(i64, i64); m],
    }
    let mut ab = ab;
    if ab.last().unwrap().0 != n {
        ab.push((n, 0));
    }
    let mut c = 0;
    let mut t = 0;
    let mut ans = 0;
    for (a, b) in ab {
        if c >= k {
            ans += min(c - k + 1, a - t);
        }
        c = max(c - (a - t - 1), 0);
        c += b;
        c = max(c - 1, 0);
        t = a;
    }
    if c >= k {
        ans += 1;
    }
    println!("{}", ans);
}
