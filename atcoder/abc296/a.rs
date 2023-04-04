fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    getline();
    let s: Vec<_> = getline().trim().chars().collect();
    let n = s.len();
    println!("{}", if (0..n - 1).all(|i| s[i] != s[i + 1]) { "Yes" } else { "No" });
}
