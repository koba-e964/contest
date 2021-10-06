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

const MOD: i64 = 1_000_000_007;

fn powmod(x: i64, mut e: i64) -> i64 {
    let mut sum = 1;
    let mut cur = x % MOD;
    while e > 0 {
        if e % 2 != 0 {
            sum = sum * cur % MOD;
        }
        cur = cur * cur % MOD;
        e /= 2;
    }
    sum
}

fn calc(x: u32, y: u32, len: usize) -> i64 {
    if y == 0 {
        let tmp = powmod(2, x as i64) - 1;
        return powmod(tmp, len as i64);
    }
    // \sum (-1)^i C(len, i) * 2^{x * (len - i)} * (2^{len - i} - 1)^y
    let mut comb = 1;
    let mut tot = 0;
    for i in 0..len + 1 {
        let row = powmod(2, (len - i) as i64) - 1;
        let tmp = comb * powmod(2, x as i64 * (len - i) as i64) % MOD
            * powmod(row, y as i64) % MOD;
        if i % 2 == 0 {
            tot = (tot + tmp) % MOD;
        } else {
            tot = (tot + MOD - tmp) % MOD;
        }
        comb = comb * (len - i) as i64 % MOD
            * powmod(i as i64 + 1, MOD - 2) % MOD;
    }
    tot
}

fn main() {
    input! {
        n: usize,
        b: [i64; n],
    }
    let mut c = vec![(0, 0)];
    for i in 0..n {
        if b[i] >= 0 {
            c.push((b[i], i + 1));
        }
    }
    let mut prod = 1;
    for i in 0..c.len() - 1 {
        let (x, y) = c[i];
        let (z, w) = c[i + 1];
        if (x & z) != x {
            println!("0");
            return;
        }
        prod = prod * calc(x.count_ones(), (x ^ z).count_ones(), w - y) % MOD;
    }
    println!("{}", prod);
}
