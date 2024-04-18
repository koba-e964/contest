fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let mut s = getline().trim().chars().collect::<Vec<_>>();
    let ok = s[0].is_ascii_uppercase() && s[1..].iter().all(|&c| c.is_ascii_lowercase());
    println!("{}", if ok { "Yes" } else { "No" });
}
