fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn quo(a: i64, b: i64) -> i64 {
    assert!(b > 0);
    let mut r = a % b;
    if r < 0 {
        r += b;
    }
    (a - r) / b
}

fn main() {
    getline();
    let s = getline().trim().bytes().collect::<Vec<_>>();
    let n = s.len();
    let mut dig = vec![0; n + 1];
    for i in 0..n {
        let d = (s[i] - b'0') as i64 * (i as i64 + 1);
        dig[i] += d;
        dig[n] -= d;
    }
    for i in (0..n).rev() {
        let q = quo(dig[i + 1], 10);
        dig[i] += q;
        dig[i + 1] -= q * 10;
    }
    // /= 9
    let mut carry = 0;
    for i in 0..n + 1 {
        let q = quo(dig[i] + carry * 10, 9);
        carry = (dig[i] + carry * 10) - q * 9;
        dig[i] = q;
    }
    while dig[0] == 0 {
        dig.remove(0);
    }
    for d in dig {
        print!("{d}");
    }
    println!();
}
