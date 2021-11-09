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
        n: usize, c: usize,
        l: [usize; n],
        w: [usize; n],
    }
    const INF: i64 = 1 << 50;
    const B: usize = 51;
    let mut dp = vec![vec![vec![-INF; B]; B]; c + 1];
    let mut pr = vec![B; B];
    for i in 0..n {
        pr[l[i]].chmin(w[i]);
    }
    let mut ma = 0;
    for i in 0..B {
        if pr[i] == B { continue; }
        if pr[i] > c { continue; }
        for j in 0..B {
            if i == j || pr[j] == B { continue; }
            if pr[i] + pr[j] <= c {
                dp[pr[i] + pr[j]][i][j].chmax((i + j) as i64);
            }
        }
    }
    for i in 0..c + 1 {
        for j in 0..B {
            for k in 0..B {
                let val = dp[i][j][k];
                if val < 0 { continue; }
                for l in 0..B {
                    if pr[l] + i > c { continue; }
                    // katomatsu
                    if j != l && k != l && j.cmp(&k) == l.cmp(&k) {
                        dp[i + pr[l]][k][l].chmax(val + l as i64);
                        ma.chmax(val + l as i64);
                    }
                }
            }
        }
    }
    println!("{}", ma);
}
