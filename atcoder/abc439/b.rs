fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn f(mut x: i64) -> i64 {
    let mut ans = 0;
    while x > 0 {
        ans += (x % 10) * (x % 10);
        x /= 10;
    }
    ans
}

fn main() {
    let mut n = getline().trim().parse::<i64>().unwrap();
    let mut ok = false;
    for _ in 0..50 {
        n = f(n);
        if n == 1 {
            ok = true;
        }
    }
    println!("{}", if ok { "Yes" } else { "No" });
}
