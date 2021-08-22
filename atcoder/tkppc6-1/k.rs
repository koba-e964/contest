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

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut imos = vec![0; n + 1];
    imos[0] = a[0];
    for i in 1..n {
        imos[i] = (a[i] + 10 - a[i - 1]) % 10;
    }
    imos[n] = (10 - a[n - 1]) % 10;
    imos.sort();
    let mut acc_l = vec![0; n + 2];
    let mut acc_r = vec![0; n + 2];
    for i in 0..n + 1 {
        acc_l[i + 1] = acc_l[i] + imos[i];
    }
    for i in (0..n + 1).rev() {
        acc_r[i] = acc_r[i + 1] + if imos[i] == 0 { 0 } else { 10 - imos[i] };
    }
    let mut tot = 1 << 30;
    for i in 0..n + 2 {
        tot.chmin(max(acc_l[i], acc_r[i]));
    }
    println!("{}", tot);
}
