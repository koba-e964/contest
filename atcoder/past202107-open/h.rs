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
    let mut s = 0;
    for i in 0..n {
        s += a[i];
    }
    if n == 2 {
        println!("1.0");
        return;
    }
    const INF: f64 = 1.0e18;
    let mut dp = vec![vec![vec![INF; s + 1]; s + 1]; n];
    dp[0][0][0] = 0.0;
    for i in 1..n {
        for j in 0..s + 1 {
            for k in 0..j + 1 {
                let mut me = INF;
                for l in 0..j - k + 1 {
                    let d = l as f64 - k as f64;
                    let dist = (d * d + 1.0).sqrt();
                    me.chmin(dp[i - 1][j - k][l] + dist);
                }
                dp[i][j][k] = me;
            }
        }
    }
    println!("{}", dp[n - 1][s][0]);
}
