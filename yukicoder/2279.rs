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

// https://yukicoder.me/problems/no/2279 (3)
// 桁の位置ごとに、そこが 1 となるような分け方の総数が分かれば良い。
fn main() {
    input! {
        n: usize,
        s: chars,
    }
    const MOD: i64 = 998_244_353;
    let mut cur = 1;
    let mut ans = 0;
    for pos in 0..n {
        let mut dp0 = vec![0; n + 1];
        let mut dp1 = vec![0; n + 1];
        let mut acc0 = vec![0; n + 1];
        let mut acc1 = vec![0; n + 1];
        dp0[0] = 1;
        acc0[0] = 1;
        for i in 1..n + 1 {
            if i >= pos + 1 && s[i - pos - 1] == '1' {
                let tmp = (acc0[i - pos - 1] + acc1[i - 1]) % MOD;
                dp1[i] = tmp;
                let tmp = (acc0[i - 1] - acc0[i - pos - 1] + MOD) % MOD;
                dp0[i] = tmp;
            } else {
                dp0[i] = acc0[i - 1];
                dp1[i] = acc1[i - 1];
            }
            acc0[i] = (acc0[i - 1] + dp0[i]) % MOD;
            acc1[i] = (acc1[i - 1] + dp1[i]) % MOD;
        }
        ans = (ans + cur * dp1[n]) % MOD;
        cur = cur * 2 % MOD;
    }
    println!("{}", ans);
}
