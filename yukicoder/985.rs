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
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

const MOD: i64 = 998_244_353;

// https://yukicoder.me/problems/no/985 (4.5)
// C をアダマール変換。
fn main() {
    input! {
        n: usize,
        c: [i64; 1 << n],
        lr: [(i64, i64); 1 << n],
    }
    let mut c = c;
    for i in 0..n {
        for bits in 0..1 << n {
            if (bits & 1 << i) != 0 { continue; }
            let x = c[bits];
            let y = c[bits | 1 << i];
            c[bits] = x + y;
            c[bits | 1 << i] = x - y;
        }
    }
    let mut tot = 0;
    for i in 0..1 << n {
        let (l, r) = lr[i];
        let t = (c[i] % MOD + MOD) % MOD;
        if c[i] > 0 {
            tot += (r + MOD) * t % MOD;
        } else {
            tot += (l + MOD) * t % MOD;
        }
    }
    tot %= MOD;
    let inv2 = (MOD + 1) / 2;
    for _ in 0..n {
        tot = tot * inv2 % MOD;
    }
    println!("{}", tot);
}
