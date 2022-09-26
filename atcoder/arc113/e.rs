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
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// https://atcoder.jp/contests/arc113/tasks/arc113_e
// b のまとまりを a で挟む。具体的には、aAbBaA... (A, B はそれぞれ a, b の長さ 0 以上の列) を BbAA... にする。これを左から貪欲に実行し、最後に一番右が bAaBbb であれば bBaA にしたものが答え。
// なお、AB の場合は A の a を可能な限り消すのが最善。
// -> babaaababa の場合で失敗する。(正解は bbbbaa)
// 端ではない a が 1 個のまとまりをまとめた方が効率が良い。
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        t: usize,
        s: [chars; t],
    }
    for s in s {
        let mut b = 0;
        let mut a = 0;
        let n = s.len();
        let mut cur = 0;
        let mut last = 'b';
        let mut v = vec![];
        for i in 0..n {
            if last != s[i] {
                v.push(cur);
                last = s[i];
                cur = 0;
            }
            cur += 1;
        }
        v.push(cur);
        let mut one = 0;
        let mut is_first = true;
        for i in 0..(v.len() - 1) / 2 {
            b += v[2 * i];
            if v[2 * i + 1] == 1 {
                one += 1;
            } else {
                if a == 0 {
                    a = v[2 * i + 1];
                } else {
                    a = a + v[2 * i + 1] - 2;
                }
                if b != 0 {
                    is_first = false;
                }
            }
        }
        if one % 2 == 1 {
            if a == 0 {
                a = 1
            } else {
                a -= 1;
            }
        } else if is_first && v.len() % 2 == 1 {
            a = max(a - 2, 0);
        }
        if v.len() % 2 == 0 {
            b += v[v.len() - 2];
            if a == 0 {
                a += v[v.len() - 1];
            } else {
                a += v[v.len() - 1] - 2;
            }
            let mut t = "".to_string();
            for _ in 0..b {
                t.push('b');
            }
            for _ in 0..a {
                t.push('a');
            }
            puts!("{}\n", t);
            continue;
        }
        let last = v[v.len() - 1];
        if a == 0 || b == 0 {
            let mut t = "".to_string();
            for _ in 0..b {
                t.push('b');
            }
            for _ in 0..a % 2 {
                t.push('a');
            }
            for _ in 0..last {
                t.push('b');
            }
            puts!("{}\n", t);
            continue;
        }
        if last >= 3 && a % 2 == 1 {
            b += last - 2;
            let mut t = "".to_string();
            for _ in 0..b {
                t.push('b');
            }
            for _ in 0..a {
                t.push('a');
            }
            puts!("{}\n", t);
            continue;
        }
        let mut t = "".to_string();
        for _ in 0..b {
            t.push('b');
        }
        for _ in 0..a % 2 {
            t.push('a');
        }
        for _ in 0..last {
            t.push('b');
        }
        puts!("{}\n", t);
    }
}
