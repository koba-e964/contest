use std::io::Read;
use std::io::{Write, BufWriter};

fn get_word() -> String {
    let stdin = std::io::stdin();
    let mut stdin=stdin.lock();
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(16);
        loop {
            let res = stdin.read(&mut u8b);
            if res.unwrap_or(0) == 0 || u8b[0] <= b' ' {
                break;
            } else {
                buf.push(u8b[0]);
            }
        }
        if buf.len() >= 1 {
            let ret = String::from_utf8(buf).unwrap();
            return ret;
        }
    }
}

fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

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

fn fact_init(w: usize, p: i64) -> (Vec<i64>, Vec<i64>) {
    let mut fac = vec![1; w];
    let mut invfac = vec![0; w];
    for i in 1..w {
        fac[i] = fac[i - 1] * i as i64 % p;
    }
    invfac[w - 1] = powmod(fac[w - 1], p - 2, p);
    for i in (0..w - 1).rev() {
        invfac[i] = invfac[i + 1] * (i as i64 + 1) % p;
    }
    (fac, invfac)
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    let n: usize = get();
    let p: i64 = get();
    let (fac, invfac) = fact_init(n * n + 1, p);
    let comb = |n: usize, k: usize| -> i64 {
        if n < k {
            0
        } else {
            fac[n] * invfac[k] % p * invfac[n - k] % p
        }
    };
    // (#v prev, #v top, #edges)
    let mut aux = vec![vec![vec![0; n * n + 1]; n + 1]; n];
    for vp in 1..n {
        aux[vp][0][0] = 1;
        for vt in 1..n + 1 {
            for ne in 0..=vp * vt {
                let mut me = 0;
                for nl in 0..ne {
                    let myedge = ne - nl;
                    if myedge > vp { continue; }
                    let coef = comb(vp, myedge);
                    let tmp = coef * aux[vp][vt - 1][nl] % p;
                    me += tmp;
                    if me >= p {
                        me -= p;
                    }
                }
                aux[vp][vt][ne] = me;
            }
        }
    }
    // (#v prev, #v top, #edges)
    let mut aux2 = vec![vec![vec![0; n * n + 1]; n + 1]; n];
    // O(n^6)
    for vp in 1..n {
        for vt in 0..n + 1 {
            for i in vt..=vp * vt {
                for j in i..=n * n {
                    aux2[vp][vt][j] += aux[vp][vt][i] * comb(vt * (vt - 1) / 2, j - i) % p;
                    if aux2[vp][vt][j] >= p {
                        aux2[vp][vt][j] -= p;
                    }
                }
            }
        }
    }
    // (distance max % 2, #vert, #vert at top level, #vert odd, #edges)
    let mut dp = vec![vec![vec![vec![vec![0; n * (n - 1) / 2 + 1]; n + 1]; n]; n + 1]; 2];
    dp[0][1][1][0][0] = 1;
    // O(n^8)
    for v in 1..n {
        for d in 0..2 {
            for vt in 1..=n / 2 {
                for vo in 0..=n / 2 {
                    for ne in 0..=v * (v - 1) / 2 {
                        if dp[d][v][vt][vo][ne] == 0 {
                            continue;
                        }
                        for nl in 1..=(n - v).min(n / 2) {
                            let mut nvo = vo;
                            if d == 0 {
                                nvo += nl;
                            }
                            if nvo > n / 2 {
                                continue;
                            }
                            if v + nl - nvo > n / 2 {
                                continue;
                            }
                            for nle in 0..=(n * (n - 1) / 2 - ne).min(vt * nl + nl * (nl - 1) / 2) {
                                let mut coef = dp[d][v][vt][vo][ne];
                                coef = coef * aux2[vt][nl][nle] % p;
                                coef = coef * comb(v + nl - 1, nl) % p;
                                let nvt = nl;
                                let nne = ne + nle;
                                if nne > n * (n - 1) / 2 {
                                    continue;
                                }
                                dp[1 - d][v + nl][nvt][nvo][nne] += coef;
                                if dp[1 - d][v + nl][nvt][nvo][nne] >= p {
                                    dp[1 - d][v + nl][nvt][nvo][nne] -= p;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    let mut tot = vec![0; n * (n - 1) / 2 + 1];
    for d in 0..2 {
        for vt in 1..n {
            let vo = n / 2;
            for ne in 0..=n * (n - 1) / 2 {
                tot[ne] += dp[d][n][vt][vo][ne];
                if tot[ne] >= p {
                    tot[ne] -= p;
                }
            }
        }
    }
    let ans = &tot[n - 1..];
    putvec!(ans);
}
