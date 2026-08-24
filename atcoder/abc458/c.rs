fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let s = getline().trim().bytes().collect::<Vec<_>>();
    let n = s.len();
    let mut ans = 0i64;
    for i in 0..n {
        if s[i] == b'C' {
            ans += i.min(n - 1 - i) as i64 + 1;
        }
    }
    println!("{ans}");
}
