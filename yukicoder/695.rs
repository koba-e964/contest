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

// Verified by: https://yukicoder.me/submissions/706484
fn ext_gcd(a: i128, b: i128) -> (i128, i128, i128) {
    if b == 0 {
        return (a, 1, 0);
    }
    let r = a % b;
    let q = a / b;
    let (g, x, y) = ext_gcd(b, r);
    (g, y, x - q * y)
}

fn inv_mod(a: i128, b: i128) -> i128 {
    let (_, mut x, _) = ext_gcd(a, b);
    x %= b;
    if x < 0 {
        x += b;
    }
    x
}
// gcd(rm[i].1, rm[j].1) == 1 for i != j
// Ref: https://www.creativ.xyz/ect-gcd-crt-garner-927/
// O(n^2)
fn garner(rm: &[(i128, i128)], mo: i128) -> i128 {
    let n = rm.len();
    let mut x_mo = (rm[0].0 % rm[0].1) % mo;
    let mut mp_mo = 1;
    let mut coef = Vec::with_capacity(n);
    coef.push(rm[0].0 % rm[0].1);
    for i in 1..n {
        let (r, m) = rm[i];
        let r = r % m;
        let mut mp_mi = 1;
        let mut x_mi = 0;
        mp_mo = mp_mo * (rm[i - 1].1 % mo) % mo;
        for j in 0..i {
            x_mi = (x_mi + mp_mi * (coef[j] % m)) % m;
            mp_mi = mp_mi * (rm[j].1 % m) % m;
        }
        let t = (r - x_mi + m) % m * inv_mod(mp_mi, m) % m;
        x_mo = (x_mo + t % mo * mp_mo) % mo;
        coef.push(t);
    }
    x_mo
}

fn zadd(a: i32, b: i32, c: i32) -> i32 {
    let mut x = a + b;
    if x >= c {
        x -= c;
    }
    x
}

fn calc(n: usize, x: &[usize], mo: i32) -> i32 {
    let n2 = (n + 1) / 2;
    let mut dp = vec![0i32; n2];
    dp[0] = 1;
    for i in 1..n2 {
        let mut tmp = 0;
        for &x in x {
            if i >= x {
                tmp = zadd(tmp, dp[i - x], mo);
            }
        }
        dp[i] = tmp;
    }
    let mut res = 0i64;
    for &x in x {
        for i in 0..n2 {
            if n - i - 1 >= x && n - i - 1 - x < n - n2 {
                res += dp[i] as i64 * dp[n - i - 1 - x] as i64;
                res %= mo as i64;
            }
        }
    }
    res as i32
}

// https://yukicoder.me/problems/no/695 (4, 答えを見た)
// この問題の法は 10^{17} + 7 = 17×9920467×592951213 (異なる素因数 3 個) を満たす。
// 素因数ごとに計算することにすれば 1 エントリあたり 4 バイトで済むため、必要なメモリは 20M * 4B = 80MB。
// 両側から DP をして、半分の境界を超えるところを全探索する方式でできる。両側の DP 配列は同じ値になるので、半分のメモリでできる。
fn main() {
    input! {
        n: usize, m: usize,
        x: [usize; m],
    }
    let mo = [17, 9920467, 592951213];
    let mut res = [(0, 0); 3];
    let mut prod = 1i128;
    for i in 0..3 {
        res[i] = (calc(n, &x, mo[i]) as i128, mo[i] as i128);
        prod *= mo[i] as i128;
    }
    let res = garner(&res, prod);
    println!("{}", res);
}
