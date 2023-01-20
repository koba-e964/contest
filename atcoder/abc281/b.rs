fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn check(s: &[char]) -> bool {
    if s.len() != 8 {
        return false;
    }
    if !s[0].is_ascii_uppercase() || !s[7].is_ascii_uppercase() {
        return false;
    }
    (1..7).all(|i| '0' <= s[i] && s[i] <= '9') && s[1] != '0'
}

fn main() {
    let s: Vec<_> = getline().trim().chars().collect();
    println!("{}", if check(&s) { "Yes" } else { "No" });
}
