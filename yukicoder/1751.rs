use std::cmp::*;
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

const MOD: i64 = 998_244_353;

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

// https://yukicoder.me/problems/no/1751 (3.5)
// 普通の Nim で losing なら勝つ確率は 1/2 未満で、winning であれば 1/2 より大きい。
// losing なら 1 手で 1 個減るように強制することができるはずなので、勝つ確率は
// (1-3^{-sum A})/2 である。
// winning なら xorsum A = 0 で sum A が最小な状態にもっていくのが最善。
fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut xor = 0;
    let mut s = 0;
    for &a in &a {
        xor ^= a;
        s += a;
    }
    let inv2 = (MOD + 1) / 2;
    if xor == 0 {
        let ans = powmod(3, MOD - 1 - s % (MOD - 1), MOD);
        println!("{}", (MOD + 1 - ans) * inv2 % MOD);
        return;
    }
    let mut mins = s;
    for &a in &a {
        if a ^ xor < a {
            mins = min(mins, s - a + (a ^ xor));
        }
    }
    let ans = powmod(3, MOD - 2 - mins % (MOD - 1), MOD);
    println!("{}", (1 + ans) * inv2 % MOD);
}
