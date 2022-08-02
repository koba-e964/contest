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
        h: usize, w: usize, k: usize,
        a: [[i64; w]; h],
    }
    const INF: i64 = 1 << 50;
    let mut ans = INF;
    for i in 0..h {
        for j in 0..w {
            let mut dp = vec![vec![vec![INF; h + w - k]; w]; h];
            if (a[i][j], i, j) <= (a[0][0], 0, 0) {
                dp[0][0][0] = a[0][0];
            } else {
                if k < h + w - 1 {
                    dp[0][0][1] = 0;
                }
            };
            for x in 0..h {
                for y in 0..w {
                    if x + y == 0 { continue; }
                    for z in 0..h + w - k {
                        let mut me = INF;
                        let mut toidx = z;
                        if x > 0 {
                            me.chmin(dp[x - 1][y][z]);
                        }
                        if y > 0 {
                            me.chmin(dp[x][y - 1][z]);
                        }
                        if (a[x][y], x, y) >= (a[i][j], i, j) {
                            me += a[x][y];
                        } else {
                            toidx += 1;
                        }
                        if toidx < h + w - k {
                            dp[x][y][toidx].chmin(me);
                        }
                    }
                }
            }
            let s = dp[h - 1][w - 1][h + w - 1 - k];
            ans.chmin(s);
        }
    }
    println!("{}", ans);
}
