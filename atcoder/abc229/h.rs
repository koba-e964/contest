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
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

const INF: i64 = 1 << 60;

fn quo(x: i64, y: i64) -> i64 {
    let r = (x % y + y) % y;
    (x - r) / y
}

fn rec(n: usize, w: u8, b: u8, memo: &mut HashMap<(u8, u8), i64>) -> i64 {
    let key = (w, b);
    if let Some(&val) = memo.get(&key) {
        return val;
    }
    let mut lo = -INF;
    let mut hi = INF;
    for v in 0..n {
        if v > 0 && (w & 1 << v) != 0 {
            if ((w | b) & (1 << (v - 1))) == 0 {
                let nw = w ^ 1 << v ^ 1 << (v - 1);
                let nb = b;
                let sub = rec(n, nw, nb, memo);
                lo = max(lo, sub);
            }
        }
        if (b & 1 << v) != 0 {
            let nw = w;
            let nb = b ^ 1 << v;
            let sub = rec(n, nw, nb, memo);
            lo = max(lo, sub);
        }
        if v > 0 && (b & 1 << v) != 0 {
            if ((w | b) & (1 << (v - 1))) == 0 {
                let nw = w;
                let nb = b ^ 1 << v ^ 1 << (v - 1);
                let sub = rec(n, nw, nb, memo);
                hi = min(hi, sub);
            }
        }
        if (w & 1 << v) != 0 {
            let nw = w ^ 1 << v;
            let nb = b;
            let sub = rec(n, nw, nb, memo);
            hi = min(hi, sub);
        }
    }
    let base = 1 << 40;
    assert!(lo < hi);
    let l = quo(lo + base, base);
    let r = quo(hi - 1, base);
    let ans = if l <= r {
        if l <= 0 && 0 <= r {
            0
        } else if l > 0 {
            l * base
        } else {
            r * base
        }
    } else {
        let mut tmp = 0;
        for i in (0..40).rev() {
            let base = 1 << i;
            let l = quo(lo + base, base);
            let r = quo(hi - 1, base);
            if l <= r {
                tmp = l * base;
                break;
            }
        }
        tmp
    };
    memo.insert(key, ans);
    ans
}

// The author read the editorial before implementing this.
fn main() {
    input! {
        n: usize,
        s: [chars; n],
    }
    let mut memo = HashMap::new();
    let mut del = 0;
    for j in 0..n {
        let mut w = 0;
        let mut b = 0;
        for i in 0..n {
            let val = 1 << i;
            if s[i][j] == 'W' {
                w |= val;
            }
            if s[i][j] == 'B' {
                b |= val;
            }
        }
        let sub = rec(n, w, b, &mut memo);
        del += sub;
    }
    println!("{}", if del > 0 { "Takahashi" } else { "Snuke" });
}
