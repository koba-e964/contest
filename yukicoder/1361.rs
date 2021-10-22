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

fn two(a: &[i64], b: &[i64]) -> Vec<(i64, i64, i64)> {
    let mut ans = vec![];
    for &a in a {
        for &b in b {
            ans.push((a * b, a, b));
        }
    }
    ans.sort_unstable();
    ans
}

fn quo(a: i64, b: i64) -> i64 {
    assert!(b > 0);
    let mut r = a % b;
    if r < 0 {
        r += b;
    }
    (a - r) / b
}

const INF: i64 = 1 << 60;

fn main() {
    input! {
        k: usize, l: usize, m: usize, n: usize, s: i64,
        a: [i64; k],
        b: [i64; l],
        c: [i64; m],
        d: [i64; n],
    }
    let fst = two(&a, &b);
    let snd = two(&c, &d);
    let mut pass = INF;
    let mut fail = -INF;
    while pass - fail > 1 {
        let mid = fail + (pass - fail) / 2;
        let mut count = 0;
        for &(f, _, _) in &fst {
            if f == 0 {
                if mid >= 0 {
                    count += snd.len() as i64;
                }
            } else if f > 0 {
                let idx = snd.binary_search(&(quo(mid, f), INF, INF)).unwrap_err();
                count += idx as i64;
            } else {
                let idx = snd.binary_search(&(quo(-mid - f - 1, -f), -INF, -INF)).unwrap_err();
                count += (snd.len() - idx) as i64;
            }
        }
        if count >= s {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    println!("{}", pass);
    for &(f, f1, f2) in &fst {
        if f == 0 {
            if pass != 0 {
                continue;
            }
            let (_, s1, s2) = snd[0];
            println!("{} {} {} {}",  f1, f2, s1, s2);
            return;
        }
        if pass % f != 0 {
            continue;
        }
        let q = pass / f;
        let lo = snd.binary_search(&(q, -INF, -INF)).unwrap_err();
        let hi = snd.binary_search(&(q, INF, INF)).unwrap_err();
        if lo < hi {
            let (_, s1, s2) = snd[lo];
            println!("{} {} {} {}", f1, f2, s1, s2);
            return;
        }
    }
}
