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
        n: usize, m: i64, q: i64,
        ab: [(i64, i64); n],
    }
    let mut val = vec![(0, 0); 1 << n];
    for bits in 1..1 << n {
        let mut sum = (0, 0);
        for i in 0..n {
            if (bits & 1 << i) == 0 { continue; }
            sum.0 += ab[i].0;
            sum.1 += ab[i].1;
        }
        val[bits] = sum;
    }
    let res = [m, q];
    let mut dpmq = vec![[0; 2]; 1 << n];
    for bits in 1..1 << n {
        for b in 0..2 {
            let mut me = 0;
            for i in 0..n {
                if (bits & 1 << i) == 0 { continue; }
                let sub = dpmq[bits ^ 1 << i][b];
                me = max(me, sub);
            }
            if val[bits].0 <= res[b] {
                me = max(me, val[bits].1);
            }
            dpmq[bits][b] = me;
        }
    }
    let mut ans = 0;
    for bits in 0..1 << n {
        ans = max(ans, dpmq[bits][0] + dpmq[(1 << n) - 1 - bits][1]);
    }
    println!("{}", ans);
}
