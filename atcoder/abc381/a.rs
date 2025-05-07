fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    getline();
    let s = getline().trim().to_string();
    let n = s.len();
    if n % 2 != 1 {
        println!("No");
        return;
    }
    let t = "1".repeat(n / 2) + "/" + &"2".repeat(n / 2);
    println!("{}", if s == t { "Yes" } else { "No" });
}
