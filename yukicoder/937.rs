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

fn f2x_rem(mut x: usize, y: usize) -> usize {
    let mut count = 0;
    let mut cur = y;
    while 2 * x >= cur {
        cur *= 2;
        count += 1;
    }
    for _ in 0..count {
        cur >>= 1;
        if (x ^ cur) < x {
            x ^= cur;
        }
    }
    x
}

fn f2x_gcd(mut x: usize, mut y: usize) -> usize {
    while y != 0 {
        let r = f2x_rem(x, y);
        x = y;
        y = r;
    }
    x
}

fn f2x_mul(mut x: usize, y: usize) -> usize {
    let mut cur = 1;
    let mut prod = 0;
    while cur <= y {
        if (y & cur) != 0 {
            prod ^= x;
        }
        cur <<= 1;
        x <<= 1;
    }
    prod
}

// https://yukicoder.me/problems/no/937 (4)
// 作れる整数は、F_2[x] の元としてみたときの A_i の gcd の倍数である。
// それらを全探索すれば良い。
// Tags: gf2-polynomial-ring
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    const W: usize = 100_001;
    let mut f = vec![0; W];
    let mut g = 0;
    let mut sum = 0;
    for &a in &a {
        g = f2x_gcd(g, a);
        f[a] += 1;
        sum += a as i64;
    }
    let mut ma = 0;
    for i in 1..1 << 17 {
        let p = f2x_mul(g, i);
        if p >= W { continue; }
        let mut dec = 0;
        for j in 1..=(W - 1) / p {
            dec += f[p * j] * (p - 1) as i64 * j as i64;
        }
        ma = std::cmp::max(ma, dec);
    }
    println!("{}", sum - ma);
}
