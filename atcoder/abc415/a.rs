fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    getline();
    let a = getline().split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    let x = getline().trim().to_string();
    if a.contains(&x) {
        println!("Yes");
    } else {
        println!("No");
    }
}
