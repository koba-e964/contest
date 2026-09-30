fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn yn(a: bool) -> &'static str {
    if a {
        "Yes"
    } else {
        "No"
    }
}

fn main() {
    let s = getline().trim().chars().collect::<Vec<_>>();
    println!("{}", yn(s[0] == s[s.len() - 1]));
}
