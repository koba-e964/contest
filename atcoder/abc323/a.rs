fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let s = getline().chars().collect::<Vec<_>>();
    let n = s.len();
    println!("{}", if (0..8).all(|i| s[2 * i + 1] == '0') { "Yes" } else { "No" });
}
