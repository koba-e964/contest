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
    const W: usize = 200_001;
    let mut f = vec![0i64; W];
    for a in a {
        f[a] += 1;
    }
    let mut acc = vec![0; W + 1];
    let mut accv = vec![0; W + 1];
    for i in 0..W {
        acc[i + 1] = acc[i] + f[i];
        accv[i + 1] = accv[i] + f[i] * i as i64;
    }
    let mut ans = 0i64;
    for i in 1..W {
        let mut me = 0;
        me += accv[i];
        me -= accv[W] - accv[i];
        for j in 1..(W - 1) / i + 1 {
            me += (j * i) as i64 * (acc[min(W, i * j + i)] - acc[i * j]);
        }
        ans += me * f[i];
    }
    println!("{}", ans);
}
