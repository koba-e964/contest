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
        n: usize, vmax: usize, c: i32,
        vw: [(usize, i32); n],
    }
    let mut dp = vec![0; vmax + 1];
    let mut ep = vec![0; vmax + 1];
    for (v, w) in vw {
        if v > vmax { continue; }
        for x in &mut ep { *x = 0; }
        for i in 0..vmax + 1 - v {
            ep[i + v] = dp[i] + c + w;
        }
        for i in 0..vmax + 1 - v {
            ep[i + v] = max(ep[i + v],ep[i] + w);
        }
        for i in 0..vmax + 1 {
            dp[i] = max(dp[i], ep[i]);
        }
    }
    println!("{}", dp[vmax]);
}
