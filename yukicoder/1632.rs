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

fn gcd(mut x: i64, mut y: i64) -> i64 {
    while y != 0 {
        let r = x % y;
        x = y;
        y = r;
    }
    x
}

fn powsum(a: i64, mut e: i64, mo: i64) -> (i64, i64) {
    let mut prod = 1;
    let mut sum = 0;
    let mut tmp = 1;
    let mut cur = a;
    while e > 0 {
        if e % 2 == 1 {
            sum = (sum + prod * tmp) % mo;
            prod = prod * cur % mo;
        }
        tmp = tmp * (cur + 1) % mo;
        cur = cur * cur % mo;
        e /= 2;
    }
    (prod, sum)
}

// Tags: sum-of-geometric-sequences
fn main() {
    input! {
        n: i64,
        c: [i64; 9],
    }
    let pop = c.iter().filter(|&&x| x > 0).count();
    if pop == 1 {
        let mut ans = powsum(10, n, MOD).1;
        for i in 1..10 {
            if c[i - 1] > 0 {
                ans *= i as i64;
                ans %= MOD;
            }
        }
        println!("{}", ans);
        return;
    }
    let mut g = 0;
    for i in 0..9 {
        for j in 0..9 {
            if c[i] > 0 && c[j] > 0 {
                g = gcd(g, (i as i64 - j as i64).abs());
            }
        }
    }
    let mut sum = 0;
    for i in 1..10 {
        let (a, b) = powsum(10, c[i - 1], 9 * g);
        sum = (sum * a + b * i as i64) % (9 * g);
    }
    println!("{}", gcd(g * 9, sum));
}
