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
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        ts: [(chars, [chars])],
    }
    for (t, s) in ts {
        let n = s.len();
        let k = t.len();
        const INF: i32 = 1 << 28;
        let mut dp = vec![(INF, 0, 0); k + 1];
        dp[0] = (0, 0, 0);
        for i in 1..k + 1 {
            for j in 0..n {
                if i >= s[j].len() && t[i - s[j].len()..i] == s[j] {
                    for l in i - s[j].len()..i {
                        let (x, _, _) = dp[l];
                        dp[i] = min(dp[i], (x + 1, j, l));
                    }
                }
            }
        }
        if dp[k].0 >= INF {
            puts!("-1\n");
            continue;
        }
        puts!("{}\n", dp[k].0);
        let mut cur = k;
        while cur > 0 {
            let (_, idx, pre) = dp[cur];
            puts!("{} {}\n", idx + 1, cur - s[idx].len() + 1);
            cur = pre;
        }
    }
}
