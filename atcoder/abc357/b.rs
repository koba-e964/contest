fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let s: String = getline().trim().to_string();
    let n = s.len();
    let low = s.chars().filter(|&c| c.is_ascii_lowercase()).count();
    for c in s.chars() {
        if low > n - low {
            print!("{}", c.to_ascii_lowercase());
        } else {
            print!("{}", c.to_ascii_uppercase());
        }
    }
    println!();
}
