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

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        nst: [(usize, chars, chars)],
    }
    const INF: i64 = 1 << 50;
    for (n, s, t) in nst {
        // 0: empty, 1: white, 2: black
        let mut dp = vec![[-INF; 9]; n + 1];
        dp[0][3 * 1 + if s[0] == '1' { 2 } else { 0 }] = 0;
        for i in 0..n {
            let mut me = [-INF; 9];
            for j in 0..3 {
                for k in 0..3 {
                    let val = dp[i][3 * j + k];
                    let nxt = if i + 1 < n && s[i + 1] == '1' { 2 } else { 0 };
                    me[3 * k + nxt].chmax(val);
                    if t[i] == '0' {
                        continue;
                    }
                    // right
                    if nxt == 2 {
                        me[3 * k + 1].chmax(val + 1);
                    }
                    // forward
                    if s[i] == '0' && k == 0 {
                        me[3 + nxt].chmax(val + 1);
                    }
                    // left
                    if j == 2 {
                        me[3 * k + nxt].chmax(val + 1);
                    }
                }
            }
            dp[i + 1] = me;
        }
        puts!("{}\n", dp[n].iter().max().unwrap());
    }
}
