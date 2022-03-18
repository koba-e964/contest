use std::io::Read;

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

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

fn squmul(a: &[Vec<i64>], b: &[Vec<i64>], mo: i64) -> Vec<Vec<i64>> {
    let n = a.len();
    let mut ret = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                ret[i][k] += a[i][j] * b[j][k];
                ret[i][k] %= mo;
            }
        }
    }
    ret
}

fn squpow(a: &[Vec<i64>], mut e: i64, mo: i64) -> Vec<Vec<i64>> {
    let n = a.len();
    let mut sum = vec![vec![0; n]; n];
    for i in 0..n { sum[i][i] = 1; }
    let mut cur = a.to_vec();
    while e > 0 {
        if e % 2 == 1 {
            sum = squmul(&sum, &cur, mo);
        }
        cur = squmul(&cur, &cur, mo);
        e /= 2;
    }
    sum
}

fn main() {
    let n: i64 = get();
    const MOD: i64 = 1_000_000_007;
    let mut mat;
    if n % 2 == 0 {
        mat = vec![vec![0; 4]; 4];
        for i in 0..2 {
            mat[i][2 * i + 1] += 1;
            mat[2 + i][2 * i] += 1;
        }
        mat[1][0] += 1;
    } else {
        mat = vec![vec![0; 8]; 8];
        for j in 0..2 {
            for i in 0..2 {
                mat[4 * j + i][4 * j + 2 * i + 1] += 1;
                mat[4 * j + 2 + i][4 * j + 2 * i] += 1;
            }
            mat[4 * j + 1][4 * j] += 1;
        }
        for i in 0..2 {
            mat[i][4 + 2 * i] += 1;
        }
    }
    let pw = squpow(&mat, n, MOD);
    let ans = pw[0][4 * (n % 2) as usize];
    println!("{}", ans);
}
