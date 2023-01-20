fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let s: Vec<_> = getline().trim().bytes().collect();
    let n = s.len();
    let mut ans = 0;
    for i in 0..n {
        ans = 26 * ans + (s[i] - b'A') as i64;
    }
    let mut cur = 1;
    for _ in 0..n - 1 {
        cur *= 26;
        ans += cur;
    }
    println!("{}", ans + 1);
}
