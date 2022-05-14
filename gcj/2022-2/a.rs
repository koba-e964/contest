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
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

// 0-origin
pub fn pos(n: i64, x: i64, y: i64) -> i64 {
    let iter = min(min(x, y), min(n - 1 - x, n - 1 - y));
    let offset = n * n - (n - iter * 2) * (n - iter * 2) + 1;
    if x <= y {
        if x == iter {
            offset + (y - iter)
        } else {
            offset + (n - 1 - 2 * iter) + (x - iter)
        }
    } else {
        if y == iter {
            offset + 4 * (n - 1 - 2 * iter) - (x - iter)
        } else {
            offset + 2 * (n - 1 - 2 * iter) + (n - 1 - iter - y)
        }
    }
}

// (0, 0) -> ((n - 1) / 2, 0)
fn prep(n: i64, rem: i64) -> Vec<(i64, i64)> {
    assert!(rem < n - 1);
    assert_eq!(rem % 2, 0);
    let mut ans = vec![];
    let cent = (n - 1) / 2;
    for i in 0..cent - rem / 2 {
        ans.push((i, 0));
    }
    for i in 0..rem / 2 {
        ans.push((cent - rem / 2, i));
    }
    for i in 0..rem / 2 {
        ans.push((cent - rem / 2 + i, rem / 2));
    }
    for i in 0..rem / 2 {
        ans.push((cent, rem / 2 - i));
    }
    ans.push((cent, 0));
    assert_eq!(ans.len() as i64, rem + 1 + (n - 1) / 2);
    ans
}

pub fn calc(n: i64, k: i64, pre: &mut Vec<(i64, i64)>, bias: i64) -> Option<Vec<(i64, i64)>> {
    if k < n - 1 || k % 2 != 0 {
        return None;
    }
    if n == 1 {
        assert_eq!(k, 0);
        return Some(vec![(bias, bias)]);
    }
    if n == 3 {
        
    }
    let mut ans = vec![];
    if k >= n - 1 && k < 2 * n - 2 {
        let sub = prep(n, k - n + 1);
        for i in 0..(n - 1) / 2 {
            ans.push((0, i));
        }
        for (x, y) in sub {
            ans.push((x, y + (n - 1) / 2));
        }
    }
    if k >= 2 * n - 2 && k < 3 * n - 3 {
        let sub = prep(n, k - 2 * n + 2);
        for i in 0..n - 1 {
            ans.push((0, i));
        }
        for i in 0..(n - 1) / 2 {
            ans.push((i, n - 1));
        }
        for (x, y) in sub {
            ans.push(((n - 1) / 2 + y, n - 1 - x));
        }
    }
    if k >= 3 * n - 3 && k < 4 * n - 4 {
        let sub = prep(n, k - 3 * n + 3);
        for i in 0..n - 1 {
            ans.push((0, i));
        }
        for i in 0..n - 1 {
            ans.push((i, n - 1));
        }
        for i in 0..(n - 1) / 2 {
            ans.push((n - 1, n - 1 - i));
        }
        for (x, y) in sub {
            ans.push((n - 1 - x, (n - 1) / 2 - y));
        }
    }
    if k >= 4 * n - 4 && k < 5 * n - 7 {
        // special
        let rem2 = (k - 4 * n + 4) / 2;
        for i in 0..n - 1 {
            ans.push((0, i));
        }
        for i in 0..n - 1 {
            ans.push((i, n - 1));
        }
        for i in 0..n - 1 {
            ans.push((n - 1, n - 1 - i));
        }
        for i in 0..(n - 1) / 2 {
            ans.push((n - 1 - i, 0));
        }
        for i in 0..rem2 {
            ans.push(((n - 1) / 2 - i, 0));
        }
        for i in 0..(n - 1) / 2 {
            ans.push(((n - 1) / 2 - rem2, i));
        }
        for i in 0..rem2 {
            ans.push(((n - 1) / 2 - rem2 + i, (n - 1) / 2));
        }
        ans.push(((n - 1) / 2, (n - 1) / 2));
    }
    if k >= 5 * n - 7 {
        let sub = calc(n - 2, k - 4 * n + 4, pre, bias + 1);
        return sub;
    }
    assert_eq!(ans.len() as i64, k + 1);
    for p in &mut ans {
        p.0 += bias;
        p.1 += bias;
    }
    Some(ans)
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    #[allow(unused)]
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    input! {
        t: usize,
        nk: [(i64, i64); t],
    }
    for i in 0..t {
        let (n, k) = nk[i];
        let mut pre = vec![];
        if let Some(suffix) = calc(n, k, &mut pre, 0) {
            let mut raw = pre;
            raw.extend_from_slice(&suffix);
            let mut ans = vec![];
            let m = raw.len();
            for j in 0..m - 1 {
                let from = pos(n, raw[j].0, raw[j].1);
                let to = pos(n, raw[j + 1].0, raw[j + 1].1);
                if from + 1 != to {
                    ans.push((from, to));
                }
            }
            puts!("Case #{}: {}\n", i + 1, ans.len());
            for (from, to) in ans {
                puts!("{} {}\n", from, to);
            }
        } else {
            puts!("Case #{}: IMPOSSIBLE\n", i + 1);
        }
    }
}
