use std::io::{Write, BufWriter};
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
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    #[allow(unused)]
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    input! {
        n: usize, m: usize,
        a: [usize; n],
        p: [usize1; n],
    }
    let mut invp = vec![0; n];
    for i in 0..n {
        invp[p[i]] = i;
    }
    const INF: usize = 1 << 30;
    let mut dp = vec![vec![INF; m + 1]; n + 1];
    dp[n][m] = 0;
    for i in (0..n).rev() {
        for j in (0..m + 1).rev() {
            let mut me = dp[i][j];
            if j + a[i] <= m {
                if dp[i + 1][j + a[i]] < INF {
                    me.chmin(p[i]);
                }
            }
            me.chmin(dp[i + 1][j]);
            dp[i][j] = me;
        }
    }
    if dp[0][0] >= INF {
        puts!("-1\n");
        return;
    }
    let mut ans = vec![];
    let mut x = 0;
    let mut y = 0;
    while y < m {
        let idx = invp[dp[x][y]];
        ans.push(idx + 1);
        x = idx + 1;
        y += a[idx];
    }
    puts!("{}\n", ans.len());
    putvec!(ans);
}
