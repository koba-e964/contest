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

const MOD: i64 = 998_244_353;
const INF: i64 = 1 << 50;

// |y - x| <= d
fn sub(a: &[(i64, i64)], picked: &[(i64, i64)], d: i64, pw: &[i64]) -> i64 {
    let (mindiff, maxdiff) = if picked.len() >= 2 {
        let d0 = picked[0].1 - picked[0].0;
        let d1 = picked[1].1 - picked[1].0;
        if (d0 - d1).abs() > d {
            return 0;
        }
        (min(d0, d1), max(d0, d1))
    } else {
        let x = picked[0].1 - picked[0].0;
        (x, x)
    };
    let mut a = a.to_vec();
    a.sort_by_key(|&(x, y)| y - x);
    let n = a.len();
    let mut tot = 0;
    let mut end = n;
    for i in 0..n {
        let (x, y) = a[i];
        if y - x > mindiff {
            end = i;
            break;
        }
        if y - x + d < maxdiff { continue; }
        let idx = a.binary_search_by_key(&(y - x + d, INF), |&(x, y)| (y - x, x)).unwrap_err();
        if idx > i {
            tot += pw[idx - i - 1];
        }
    
    }
    let idx = a.binary_search_by_key(&(mindiff + d, INF), |&(x, y)| (y - x, x)).unwrap_err();
    if idx >= end {
        tot += pw[idx - end];
    }
    tot
}

// O(n^3 log n)
// Tags: counting-cliques-in-special-graphs
fn main() {
    input! {
        n: usize, d: i64,
        a: [(i64, i64); n],
    }
    let mut pw = vec![0; n + 1];
    pw[0] = 1;
    for i in 1..n + 1 {
        pw[i] = pw[i - 1] * 2 % MOD;
    }
    let mut a = a;
    a.sort();
    // |x| <= d, |y| <= d, |y - x| <= d
    let mut tot = 0;
    for i in 0..n {
        let (xi, yi) = a[i];
        let idx = a.binary_search(&(xi + d, INF)).unwrap_err();
        let mut b = a[i + 1..idx].to_vec();
        b.sort_by_key(|p| p.1);
        let mut end = b.len();
        for j in 0..b.len() {
            let (_xj, yj) = b[j];
            if yj > yi {
                end = j;
                break;
            }
            if yj + d < yi { continue; }
            let idx = b.binary_search_by_key(&(yj + d, INF), |&(x, y)| (y, x)).unwrap_err();
            tot += sub(&b[j + 1..idx], &[a[i], b[j]], d, &pw);
        }
        let idx = b.binary_search_by_key(&(yi + d, INF), |&(x, y)| (y, x)).unwrap_err();
        tot += sub(&b[end..idx], &[a[i]], d, &pw);
    }
    println!("{}", tot % MOD);
}
