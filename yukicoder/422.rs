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
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

// Tags: dp, automaton
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, m: usize,
        s: chars,
        t: chars,
    }
    // mode: 0: replace 1: insert 2: delete
    const INF: i32 = 1 << 28;
    let mut dp = vec![vec![[(INF, 0, 0, 0); 3]; m + 1]; n + 1];
    dp[0][0][0] = (0, 0, 0, 0);
    for i in 0..n + 1 {
        for j in 0..m + 1 {
            for _ in 0..2 {
                let val = (dp[i][j][0].0 + 7, i, j, 0);
                for k in 1..3 {
                    dp[i][j][k].chmin(val);
                    let val = (dp[i][j][k].0, i, j, k);
                    dp[i][j][0].chmin(val);
                }
            }
            // 0: replace
            if i < n && j < m {
                let val = (dp[i][j][0].0 + if s[i] == t[j] { 0 } else { 5 },
                           i, j, 0);
                dp[i + 1][j + 1][0].chmin(val);
            }
            // 1: insert
            if j < m {
                let val = (dp[i][j][1].0 + 2,
                           i, j, 1);
                dp[i][j + 1][1].chmin(val);
            }
            // 2: delete
            if i < n {
                let val = (dp[i][j][2].0 + 2,
                           i, j, 2);
                dp[i + 1][j][2].chmin(val);
            }
        }
    }
    let (sc, _, _, _) = dp[n][m][0];
    puts!("{}\n", sc);
    let mut x = n;
    let mut y = m;
    let mut z = 0;
    let mut rs = vec![];
    let mut rt = vec![];
    while (x, y, z) != (0, 0, 0) {
        let (_, nx, ny, nz) = dp[x][y][z];
        if (nx, ny) == (x, y) {
            z = nz;
            continue;
        }
        assert_eq!(nz, z);
        match z {
            0 => {
                rs.push(s[nx]);
                rt.push(t[ny]);
            }
            1 => {
                rs.push('-');
                rt.push(t[ny]);
            }
            2 => {
                rs.push(s[nx]);
                rt.push('-');
            }
            _ => panic!(),
        }
        x = nx;
        y = ny;
    }
    rs.reverse();
    rt.reverse();
    puts!("{}\n", rs.into_iter().collect::<String>());
    puts!("{}\n", rt.into_iter().collect::<String>());
}
