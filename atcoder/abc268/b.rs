fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let s = getline().trim().to_string();
    let t = getline().trim().to_string();
    println!("{}", if t.starts_with(&s) { "Yes" } else { "No" });
}
