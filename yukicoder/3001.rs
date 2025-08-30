fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn factorize_with_prime_factors(mut x: usize) -> Vec<usize> {
    let mut p = 2;
    let mut ans = vec![];
    while p * p <= x {
        let mut e = 0;
        while x % p == 0 {
            x /= p;
            e += 1;
        }
        if e > 0 {
            ans.push(p);
        }
        p += 1;
    }
    if x > 1 {
        ans.push(x);
    }
    ans
}

// https://yukicoder.me/problems/no/3001 (3)
// -> TLE。全ての約数ではなく全ての素因数を見るべきかもしれない。素因数 p に対して周期が n/p であるものを調べる。
// -> AC。
fn main() {
    let s = getline().trim().chars().collect::<Vec<_>>();
    let n = s.len();
    let ps = factorize_with_prime_factors(n);
    let mut ans = (n + 1, 0);
    for p in ps {
        let d = n / p;
        if n % d != 0 { continue; }
        let mut tot = 0;
        for i in 0..d {
            let mut f = [0; 26];
            for j in 0..n / d {
                let c = s[j * d + i] as u8 - b'A';
                f[c as usize] += 1;
            }
            tot += n / d - *f.iter().max().unwrap();
        }
        ans = ans.min((tot, d));
    }
    let d = ans.1;
    let mut ans = vec!['+'; n];
    for i in 0..d {
        let mut f = [0; 26];
        for j in 0..n / d {
            let c = s[j * d + i] as u8 - b'A';
            f[c as usize] += 1;
        }
        let idx = f.iter().enumerate().max_by_key(|&(_, v)| v).unwrap().0;
        let c = (b'A' + idx as u8) as char;
        for j in 0..n / d {
            ans[j * d + i] = c;
        }
    }
    println!("{}", ans.into_iter().collect::<String>());
}
