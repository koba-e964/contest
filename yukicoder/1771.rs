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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

const MOD: i64 = 998_244_353;

// https://yukicoder.me/problems/no/1771 (3)
// https://yukicoder.me/problems/no/1772 (3.5)
// 4000^2 が許される。以下の 5 種類の遷移に分けて DP:
// 1. “” (if x == y)
// 2. “AB” + ?? (x-1, y)
// 3. “BA” + ?? (x, y-1)
// 4. “AA” + ?? (x-1, y-1)
// 5. “BB” + ?? (x-1, y-1)
fn main() {
    input!(x: usize, y: usize);
    assert!(x <= 4000 && y <= 4000);
    let mut dp = vec![vec![0; y + 1]; x + 1];
    dp[0][0] = 1;
    for i in 0..x + 1 {
        for j in 0..y + 1 {
            if i + j == 0 { continue; }
            let mut me = 0;
            if i == j {
                me += 1;
            }
            if i > 0 {
                me += dp[i - 1][j];
            }
            if j > 0 {
                me += dp[i][j - 1];
            }
            if i > 0 && j > 0 {
                me += 2 * dp[i - 1][j - 1];
            }
            dp[i][j] = me % MOD;
        }
    }
    println!("{}", dp[x][y]);
}
