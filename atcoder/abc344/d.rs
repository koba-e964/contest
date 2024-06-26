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
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    const INF: i64 = 1 << 58;
    input! {
        t: chars,
        n: usize,
        s: [[chars]; n],
    }
    let m = t.len();
    let mut dp = vec![INF; m + 1];
    dp[0] = 0;
    for s in s {
        let mut ep = dp.clone();
        for s in s {
            let l = s.len();
            for i in 0..(m + 1).max(l) - l {
                if t[i..i + l] == s {
                    ep[i + l] = ep[i + l].min(dp[i] + 1);
                }
            }
        }
        dp = ep;
    }
    println!("{}", if dp[m] >= INF { -1 } else { dp[m] })
}
