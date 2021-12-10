use std::cmp::*;
use std::io::{Write, BufWriter};
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
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn calc(mut a: Vec<usize>) -> bool {
    let n = a.len();
    let mut de = vec![];
    for k in 1..n - 1 {
        let ok = (a[k] > a[k + 1]) == (a[k] > a[k - 1])
            && a[k] != a[k + 1] && a[k] != a[k - 1] && a[k - 1] != a[k + 1];
        if !ok { de.push(k); }
    }
    if de.len() > 6 { return false; }
    let mut cand = vec![];
    for &d in &de {
        for i in d - 1..d + 2 {
            cand.push(i);
        }
    }
    cand.sort(); cand.dedup();
    let m = cand.len();
    for i in 0..m {
        let x = cand[i];
        for y in 0..n {
            let ok = de.iter().all(|&pos| min((pos as i64 - x as i64).abs(),
                                              (pos as i64 - y as i64).abs())
                                   <= 1);
            if !ok { continue; }
            a.swap(x, y);
            let mut t = vec![];
            for k in max(1, x) - 1..min(x + 2, n) {
                t.push(k);
            }
            for k in max(1, y) - 1..min(y + 2, n) {
                t.push(k);
            }
            t.sort_unstable(); t.dedup();
            let ok = t.iter().all(|&k| {
                k == 0 || k == n - 1 || (
                    (a[k] > a[k + 1]) == (a[k] > a[k - 1])
                        && a[k] != a[k + 1]
                        && a[k] != a[k - 1]
                        && a[k - 1] != a[k + 1]
                )
            });
            a.swap(x, y);
            if ok { return true; }
        }
    }
    false
}

// https://yukicoder.me/problems/no/1369 (3)
// 多数 (7 個以上) 門松列になっていない点がある場合は不可能。
// そうでない場合はそれらを全探索。-> 誤り。矛盾している点と矛盾していない点を交換することも
// ありえる。
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input!(a: [[usize]]);
    for a in a {
        puts!("{}\n", if calc(a) { "Yes" } else { "No" });
    }
}
