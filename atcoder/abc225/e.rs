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

fn cmp(a: (i64, i64), b: (i64, i64)) -> Ordering {
    0.cmp(&(a.0 * b.1 - a.1 * b.0))
}

// Tags: interval-maximum-independent-set, sort-by-argument
fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }
    let mut int = vec![];
    for &(x, y) in &xy {
        int.push(((x, y - 1), (x - 1, y)));
    }
    int.sort_by(|&(_, r1), &(_, r2)| cmp(r1, r2));
    let mut ans = 0;
    let mut last = (1, -1);
    for (l, r) in int {
        if cmp(last, l) <= Ordering::Equal {
            ans += 1;
            last = r;
        }
    }
    println!("{}", ans);
}
