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
        n: usize, x: usize, y: usize,
        b: i64, c: i64,
        a: [i64; n],
    }
    let mut a = a;
    a.sort();
    let mut mi = 1i64 << 62;
    for k in 0..min(x, y) + 1 {
        let all = x + y - k;
        if all > n {
            continue;
        }
        let kx = x - k;
        let ky = y - k;
        let mut hi = a[n - 1];
        let mut lo = if k > 0 { b + c } else { 0 } + a[n - 1];
        // b+c..b+c b..b c..c 0..0
        if k > 0 {
            hi.chmax(a[k - 1] + b + c);
            lo.chmin(a[0] + b + c);
        }
        if kx > 0 {
            hi.chmax(a[k + kx - 1] + b);
            lo.chmin(a[k] + b);
        }
        if ky > 0 {
            hi.chmax(a[k + kx + ky - 1] + c);
            lo.chmin(a[k + kx] + c);
        }
        if all < n {
            hi.chmax(a[n - 1]);
            lo.chmin(a[all]);
        }
        mi.chmin(hi - lo);
        let mut hi = a[n - 1];
        let mut lo = 1 << 62;
        // b+c..b+c c..c b..b 0..0
        if k > 0 {
            hi.chmax(a[k - 1] + b + c);
            lo.chmin(a[0] + b + c);
        }
        if kx > 0 {
            hi.chmax(a[k + kx + ky - 1] + b);
            lo.chmin(a[k + ky] + b);
        }
        if ky > 0 {
            hi.chmax(a[k + ky - 1] + c);
            lo.chmin(a[k] + c);
        }
        if all < n {
            hi.chmax(a[n - 1]);
            lo.chmin(a[all]);
        }
        mi.chmin(hi - lo);
    }
    println!("{}", mi);
}
