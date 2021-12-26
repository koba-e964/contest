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

// The author read the editorial before implementing this.
fn main() {
    input! {
        n: usize, k: usize,
        s: [String; n],
    }
    let mut s = s;
    s.sort_by(|a, b| {
        let mut x = a.to_owned();
        let mut y = b.to_owned();
        x += b;
        y += a;
        (x, a).cmp(&(y, b))
    });
    let mut dp = vec![vec!["{".to_owned(); k + 1]; n + 1];
    dp[n][0] = "".to_owned();
    for i in (0..n).rev() {
        for j in 0..k + 1 {
            let val = dp[i + 1][j].clone();
            dp[i][j].chmin(val);
            if j < k {
                let mut val = s[i].clone();
                val += &dp[i + 1][j];
                dp[i][j + 1].chmin(val);
            }
        }
    }
    println!("{}", dp[0][k]);
}
