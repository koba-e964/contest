fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn digitsum(mut x: i64) -> i64 {
    let mut ans = 0;
    while x > 0 {
        ans += x % 10;
        x /= 10;
    }
    ans
}

fn main() {
    let n: usize = getline().trim().parse().unwrap();
    let mut a = vec![0; n + 1];
    a[0] = 1;
    for i in 1..=n {
        for j in 0..i {
            a[i] += digitsum(a[j]);
        }
    }
    println!("{}", a[n]);
}
