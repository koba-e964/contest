#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::*;
#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok();
    return ret;
}
fn get_word() -> String {
    let mut stdin = std::io::stdin();
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
            let ret = std::string::String::from_utf8(buf).unwrap();
            return ret;
        }
    }
}
fn parse<T: std::str::FromStr>(s: &str) -> T { s.parse::<T>().ok().unwrap() }

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { parse(&get_word()) }

/*
 * Find an assignment (result) s.t. xor_i a[i] * result[i] = b (in GF(2))
 * Returns Some if such an assignment was found, None otherwise.
 */
fn gauss_elim_gf2_i64(basis: &[i64], mut b: i64) -> Option<Vec<bool>> {
    let n = basis.len();
    let mut a = basis.to_vec();
    let mut c = 0;
    let mut orig = vec![0; n];
    for i in 0 .. n { orig[i] = i; }
    let mut revmap = Vec::new();
    let w = 64; // i64's size
    for r in 0 .. w {
        if c >= n {
            break;
        }
        let mut c2 = None;
        for i in c .. n {
            if (a[i] & 1 << r) != 0 {
                c2 = Some(i);
                break;
            }
        }
        if c2 == None {
            revmap.push(None);
            continue;
        }
        let c2 = c2.unwrap();
        a.swap(c, c2);
        orig.swap(c, c2);
        let rm = a[c] & -(1 << r) << 1;
        a[c] ^= rm;
        for k in c + 1 .. n {
            if (a[k] & 1 << r) != 0 {
                a[k] ^= rm;
            }
        }
        if (b & 1 << r) != 0 {
            b ^= rm;
        }
        revmap.push(Some(c));
        c += 1;
    }
    // recover
    let rank = revmap.len();
    let mut result = vec![false; n];
    for i in (0 .. rank).rev() {
        if (b & 1 << i) != 0 {
            match revmap[i] {
                None => return None,
                Some(c) => {
                    b ^= a[c];
                    result[orig[c]] = true;
                },
            }
        }
    }
    if b == 0 {
        Some(result)
    } else {
        None
    }
}

fn solve(board: i64, m: usize, n: usize) -> Option<usize> {
    let mut basis = Vec::new();
    let mut varbasis = Vec::new();
    for i in 0 .. m {
        for j in 0 .. n {
            let mut v: i64 = 0;
            for dx in -1 .. 2 {
                for dy in -1 .. 2 {
                    let ni = i as i32 + dx;
                    let nj = j as i32 + dy;
                    if ni < 0 || ni >= m as i32 || nj < 0 || nj >= n as i32 {
                        continue;
                    }
                    v |= 1i64 << (ni * n as i32 + nj);
                }
            }
            if i > 0 && j > 0 {
                basis.push(v)
            } else {
                varbasis.push(v)
            };
        }
    }
    let q = varbasis.len();
    let mut mi = 100;
    for bits in 0 .. 1usize << q {
        let mut bd = board;
        for i in 0 .. q {
            if (bits & 1 << i) != 0 {
                bd ^= varbasis[i];
            }
        }
        if let Some(x) = gauss_elim_gf2_i64(&basis, bd) {
            // verify
            {
                let mut sum = 0;
                for i in 0 .. x.len() {
                    if x[i] { sum ^= basis[i]; }
                }
                assert_eq!(bd, sum);
            }
            mi = min(mi, x.into_iter().filter(|&a| a).count()
                     + bits.count_ones() as usize);
        }
    }
    if mi >= 100 { None } else { Some(mi) }
}

fn main() {
    let m = get();
    let n = get();
    let mut board: i64 = 0;
    for i in 0 .. m {
        for j in 0 .. n {
            let tmp: i64 = get();
            board |= tmp << (i * n + j);
        }
    }
    match solve(board, m, n) {
        Some(x) => println!("{}", x),
        None => println!("Impossible"),
    };
}
