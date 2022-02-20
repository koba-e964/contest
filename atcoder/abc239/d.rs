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
        for j in 2..(n - 1) / i + 1 {
            pr[i * j] = false;
        }
    }
    pr
}

fn main() {
    let a: usize = get();
    let b: usize = get();
    let c: usize = get();
    let d: usize = get();
    let pr = is_primes_tbl(201);
    for i in a..b + 1 {
        if (c..d + 1).all(|j| !pr[i + j]) {
            println!("Takahashi");
            return;
        }
    }
    println!("Aoki");
}
