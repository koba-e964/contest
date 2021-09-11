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

fn powmod(x: i64, mut e: i64, m: i64) -> i64 {
    let mut sum = 1;
    let mut cur = x % m;
    while e > 0 {
        if e % 2 != 0 {
            sum = sum * cur % m;
        }
        cur = cur * cur % m;
        e /= 2;
    }
    sum
}

fn fact_init(w: usize, mo: i64) -> (Vec<i64>, Vec<i64>) {
    let mut fac = vec![1; w];
    let mut invfac = vec![0; w];
    for i in 1..w {
        fac[i] = fac[i - 1] * i as i64 % mo;
    }
    invfac[w - 1] = powmod(fac[w - 1], mo - 2, mo);
    for i in (0..w - 1).rev() {
        invfac[i] = invfac[i + 1] * (i as i64 + 1) % mo;
    }
    (fac, invfac)
}

fn main() {
    input!(n: usize, mo: i64);
    let (fac, invfac) = fact_init(n + 1, mo);
    let mut tree = vec![0; n + 1];
    tree[1] = 1;
    for i in 2..n + 1 {
        tree[i] = powmod(i as i64, i as i64 - 2, mo);
    }
    let mut dp = vec![vec![0; n + 1]; n + 1];
    dp[0][0] = 1;
    for i in 1..n + 1 {
        for l in 0..i + 1 {
            let mut tmp = 0;
            for j in 1..std::cmp::min(i, l + 1) + 1 {
                tmp = (tmp + tree[j] * dp[i - j][l - (j - 1)] % mo
                       * fac[i - 1] % mo
                       * invfac[i - j] % mo
                       * invfac[j - 1]) % mo;
            }
            dp[i][l] = tmp;
        }
    }
    for i in 0..n {
        println!("{}", dp[n][i]);
    }
}
