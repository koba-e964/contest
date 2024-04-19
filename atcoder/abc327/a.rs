fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    getline();
    let s = getline();
    println!("{}", if s.contains("ab") || s.contains("ba") { "Yes" } else { "No" });
}
