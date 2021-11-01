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

const MOD: i64 = 1_000_000_009;

fn main() {
    input! {
        n: usize, k: usize,
        a: [i64; n],
    }
    if k == 0 {
        println!("0\n0");
        return;
    }
    let mut c = vec![0i32; k];
    for i in 0..k {
        let mut tmp = 0;
        for &a in &a {
            if (a & 1 << i) != 0 {
                tmp += 1;
            }
        }
        c[i] = tmp;
    }
    let mut tmp = (1i64 << k) - 1;
    tmp %= MOD;
    tmp *= (1 << (k - 1)) % MOD;
    tmp %= MOD;
    println!("{}", tmp * n as i64 % MOD);
    let mut ans = 0;
    for i in (0..k).rev() {
        let x = 2 * c[i] as i64 - n as i64;
        let x = x * x;
        ans = (4 * ans + x) % MOD;
    }
    for _ in 0..k - 1 {
        ans = 4 * ans % MOD;
    }
    println!("{}", ans);
}
