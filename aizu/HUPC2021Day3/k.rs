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

// O(n)
fn comp(a: &[usize], b: &[usize]) -> Vec<usize> {
    let n = a.len();
    let mut c = vec![0; n];
    for i in 0..n {
        c[i] = a[b[i]];
    }
    c
}

// O(n log e)
fn exp(a: &[usize], mut e: i64) -> Vec<usize> {
    let n = a.len();
    let mut prod: Vec<_> = (0..n).collect();
    let mut cur = a.to_vec();
    while e > 0 {
        if e % 2 == 1 {
            prod = comp(&prod, &cur);
        }
        cur = comp(&cur, &cur);
        e /= 2;
    }
    prod
}

// Tags: permutation
fn main() {
    let n: usize = get();
    let a: Vec<_> = (0..n).map(|_| get::<i64>()).collect();
    let mut p: Vec<_> = (0..n).collect();
    let q: usize = get();
    let mut st = vec![];
    let mut cur: Vec<_> = (0..n).collect();
    for _ in 0..q {
        let ty: i32 = get();
        if ty == 1 {
            let x = get::<usize>() - 1;
            let y = get::<usize>() - 1;
            st.push((x, y));
            cur.swap(x, y);
        } else if ty == 2 {
            let (x, y) = st.pop().unwrap();
            cur.swap(x, y);
        } else {
            let k: i64 = get();
            p = comp(&p, &exp(&cur, k));
        }
    }
    for i in 0..n {
        print!("{}{}", a[p[i]], if i + 1 == n { "\n" } else { " " });
    }
}
