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

fn gcd(mut x: usize, mut y: usize) -> usize {
    while y > 0 {
        let r = x % y;
        x = y;
        y = r;
    }
    x
}

// #{(s, t, c, d) | s,t,c,d > 0, gcd(s, t) = 1, sc = td, s,t <= x, c,d <= n}
fn calc(x: usize, n: i64) -> i64 {
    let mut ans = 0;
    for s in 1..x + 1 {
        for t in 1..x + 1 {
            if gcd(s, t) != 1 { continue; }
            ans += n / std::cmp::max(s, t) as i64;
        }
    }
    ans
}

fn nth(a: i64, n: i64) -> i64 {
    let mut pass = 0;
    let mut fail = std::cmp::min(a, 1 << ((60 + n - 1) / n)) + 1;
    while fail - pass > 1 {
        let mid = (fail + pass) / 2;
        let mut tmp = 1i64;
        for _ in 0..n {
            tmp = tmp.saturating_mul(mid);
        }
        if tmp <= a {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    pass
}

fn main() {
    input! {
        t: usize,
        n: [i64; t],
    }
    for n in n {
        let mut tmp = vec![0; 31];
        for b in 1..31 {
            tmp[b] = nth(n, b as i64);
        }
        let mut ans = n * n;
        for b in 1..30 {
            ans += (tmp[b] - tmp[b + 1]) * calc(b, n);
        }
        println!("{}", ans);
    }
}
