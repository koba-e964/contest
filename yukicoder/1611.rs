use std::io::{Write, BufWriter};
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

fn gcd(mut x: i64, mut y: i64) -> i64 {
    while y != 0 {
        let r = x % y;
        x = y;
        y = r;
    }
    x
}

fn factorize(mut x: i64) -> Vec<(i64, usize)> {
    let mut p = 2;
    let mut ans = vec![];
    while p * p <= x {
        let mut e = 0;
        while x % p == 0 {
            x /= p;
            e += 1;
        }
        if e > 0 {
            ans.push((p, e));
        }
        p += 1;
    }
    if x > 1 {
        ans.push((x, 1));
    }
    ans
}

// Returns a table pr that satisfies pr[i] <=> i is prime (0 <= i < n).
// Complexity: O(n log log n)
fn is_primes_tbl(n: usize) -> Vec<bool> {
    if n <= 2 {
        return vec![false; n];
    }
    let mut pr = vec![true; n];
    pr[0] = false;
    pr[1] = false;
    for i in 2..n {
        if !pr[i] { continue; }
        for j in 2..(n - 1) / i {
            pr[i * j] = false;
        }
    }
    pr
}

// https://yukicoder.me/problems/no/1611 (2.5)
// factorize(x) は時間的に間に合わない。小さい方の整数を試す。
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        t: usize,
        x: [i64; t],
    }
    const W: usize = 1000;
    let pr = is_primes_tbl(W);
    for x in x {
        let mut mi = 0;
        for i in 2..W {
            if !pr[i] { continue; }
            if x % i as i64 == 0 { continue; }
            mi = i as i64;
            break;
        }
        for i in 2..mi {
            let mut orig = 1;
            let mut added = 1;
            let g = gcd(i, x);
            let pe = factorize(g);
            let mut y = x;
            let mut j = i;
            for &(p, _) in &pe {
                let mut fx = 0;
                let mut fi = 0;
                while y % p == 0 {
                    fx += 1;
                    y /= p;
                }
                while j % p == 0 {
                    fi += 1;
                    j /= p;
                }
                orig *= fx + 1;
                added *= fx + fi + 1;
            }
            if j == 1 && added == 2 * orig {
                mi = i;
                break;
            }
        }
        puts!("{}\n", x * mi);
    }
}
