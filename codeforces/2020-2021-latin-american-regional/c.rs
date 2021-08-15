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

fn calc(g: &[i64], ev: i64) -> i64 {
    let n = g.len();
    let mut acc = vec![0; n + 1];
    for i in 0..n {
        acc[i + 1] = acc[i] + g[i] - ev;
    }
    let mut mi = (acc[0], 0);
    for i in 0..n + 1 {
        mi.chmin((acc[i], i));
    }
    let idx = mi.1;
    let mut tot = 0;
    let mut cur = 0;
    for i in 0..n {
        let j = (idx + i) % n;
        cur += g[j] - ev;
        tot += cur;
    }
    tot
}

fn main() {
    input! {
        n: usize,
        g: [i64; n],
    }
    let s: i64 = g.iter().sum();
    let ev = s / n as i64;
    let mut g = g;
    let mut mi = calc(&g, ev);
    g.reverse();
    mi.chmin(calc(&g, ev));
    println!("{}", mi);
}
