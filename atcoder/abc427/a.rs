fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let s = getline().trim().chars().collect::<Vec<_>>();
    let n = s.len();
    print!("{}", s[..n / 2].iter().cloned().collect::<String>());
    print!("{}", s[n / 2 + 1..].iter().cloned().collect::<String>());
}
