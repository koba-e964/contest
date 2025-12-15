fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn op(a: &[i64], b: &[i64], p: i64) -> Vec<i64> {
    let n = a.len();
    let mut num = vec![0; n];
    let mut den = vec![0; n];
    for i in 0..n {
        num[i] = (a[i] + b[i]) % p;
        for j in 0..n - i {
            den[i + j] = (den[i + j] + a[i] * b[j]) % p;
        }
    }
    let mut ans = vec![0; n];
    for i in 0..n {
        let l = num[i];
        ans[i] = l;
        for j in 2..n - i {
            num[i + j] = (num[i + j] + l * den[j]) % p;
        }
    }
    ans
}

fn pow(a: &[i64], mut e: i64, p: i64) -> Vec<i64> {
    let n = a.len();
    let mut cur = a.to_vec();
    let mut prod = vec![0; n];
    while e > 0 {
        if e % 2 != 0 {
            prod = op(&cur, &prod, p);
        }
        cur = op(&cur, &cur, p);
        e /= 2;
    }
    prod
}

// https://yukicoder.me/problems/no/3404 (4)
// O(N^2 log M)
fn main() {
    let ints = getline().trim().split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let [n, m, p] = ints[..] else { panic!() };
    let n = n as usize;
    let mut a = getline().trim().split_whitespace()
        .map(|x| x.parse::<i64>().unwrap() % p)
        .collect::<Vec<_>>();
    a.insert(0, 0);
    let res = pow(&a, m, p);
    for i in 1..=n {
        print!("{}{}", res[i], if i == n { "\n" } else { " " });
    }
}
