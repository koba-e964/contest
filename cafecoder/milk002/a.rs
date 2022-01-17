fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let mut s: Vec<_> = getline().chars().collect();
    let n = s.len();
    s.sort(); s.dedup();
    println!("{}", if s.len() == n { "Tea" } else { "Coffee" });
}
