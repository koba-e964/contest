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
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    const W: usize = 100_001;
    let mut divs = vec![vec![]; W];
    for i in 1..W {
        for j in 1..=(W - 1) / i {
            divs[i * j].push(i);
        }
    }
    let mut dp = vec![0; W];
    for i in 0..n {
        let mut ma = 0;
        for &d in &divs[a[i]] {
            ma = max(ma, dp[d]);
        }
        dp[a[i]] = ma + 1;
    }
    println!("{}", dp.iter().max().unwrap());
}
