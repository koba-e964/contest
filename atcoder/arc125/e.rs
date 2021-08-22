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

// Tags: max-flow-on-special-graphs
fn main() {
    input! {
        n: usize, m: usize,
        a: [i64; n],
        b: [i64; m],
        c: [i64; m],
    }
    let mut a = a;
    a.sort();
    let mut acc = vec![0; n + 1];
    for i in 0..n {
        acc[i + 1] = acc[i] + a[i];
    }
    let bsum: i64 = b.iter().sum();
    let mut mi = 1i64 << 60;
    let mut ev = vec![vec![]; n + 1];
    for i in 0..m {
        let q = (c[i] + b[i]) / b[i];
        if q <= n as i64 {
            ev[q as usize].push((b[i], c[i]));
        }
    }
    let mut bs = 0;
    let mut cs = 0;
    for s1 in 0..n + 1 {
        let mut tmp = acc[n - s1] + s1 as i64 * bsum;
        for &(b, c) in &ev[s1] {
            bs += b;
            cs += c;
        }
        tmp += cs - s1 as i64 * bs;
        mi.chmin(tmp);
    }
    println!("{}", mi);
}
