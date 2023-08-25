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

// p must be prime.
// Returns (B, H) where HB = A, rows(B) = rank(A)
// A: n * m, B: r * m, H: n * r
fn find_basis_mod_p(a: &[Vec<i64>], p: i64) -> (Vec<Vec<i64>>, Vec<Vec<i64>>) {
    let n = a.len();
    let m = a[0].len();
    let aold = a.clone();
    let mut a = a.to_vec();
    let mut pos = vec![];
    let mut r = 0;
    for i in 0..n {
        for j in 0..r {
            let idx = pos[j];
            assert_eq!(a[j][idx], 1);
            let val = a[i][idx];
            for k in 0..m {
                a[i][k] -= a[j][k] * val % p;
                if a[i][k] < 0 {
                    a[i][k] += p;
                }
            }
        }
        let mut c = 0;
        while c < m && a[i][c] == 0 {
            c += 1;
        }
        if c >= m {
            continue;
        }
        a.swap(r, i);
        pos.push(c);
        let aic = a[r][c];
        let aicinv = powmod(aic, p - 2, p);
        a[r][c] = 1.into();
        for j in c + 1..m {
            a[r][j] *= aicinv;
            a[r][j] %= p;
        }
        for j in r + 1..n {
            let ajc = a[j][c];
            a[j][c] = 0;
            for k in c + 1..m {
                let val = ajc * a[r][k] % p;
                a[j][k] -= val;
                if a[j][k] < 0 {
                    a[j][k] += p;
                }
            }
        }
        r += 1;
    }
    let mut tr = vec![vec![0; r]; n];
    for i in 0..n {
        let mut cur = aold[i].clone();
        for j in 0..r {
            let idx = pos[j];
            let val = cur[idx];
            tr[i][j] = val;
            for k in idx..m {
                cur[k] -= val * a[j][k] % p;
                if cur[k] < 0 {
                    cur[k] += p;
                }
            }
        }
        assert!(cur.iter().all(|&x| x == 0), "{:?} {:?} {:?}", cur, a, aold);
    }
    (a[..r].to_vec(), tr)
}

fn main() {
    input! {
        l: usize, m: usize, n: usize,
        a: [[i64; m]; l],
        b: [[i64; n]; m],
    }
    let mut c = vec![vec![0; n]; l];
    for i in 0..l {
        for j in 0..m {
            for k in 0..n {
                c[i][k] += a[i][j] * b[j][k];
            }
        }
    }
    if (0..l).any(|i| (0..n).any(|j| c[i][j] != 0)) {
        eprintln!("not zero");
        println!("No");
        return;
    }
    let p = 1_000_000_007;
    let mut a = a;
    let mut b = b;
    for v in &mut a {
        for v in v {
            if *v < 0 {
                *v += p;
            }
        }
    }
    for v in &mut b {
        for v in v {
            if *v < 0 {
                *v += p;
            }
        }
    }
    let (b, _) = find_basis_mod_p(&b, p);
    let rb = b.len();
    let (a, _) = find_basis_mod_p(&a, p);
    let ra = a.len();
    println!("{}", if m - ra == rb { "Yes" } else { "No" });
}
