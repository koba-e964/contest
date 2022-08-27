fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let s: Vec<_> = getline().trim().chars().collect();
    println!("{}", s[s.len() / 2]);
}
