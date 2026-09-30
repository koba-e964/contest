fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    getline();
    let s = getline().trim().chars().collect::<Vec<_>>();
    let t = getline().trim().chars().collect::<Vec<_>>();
    let n = t.len();
    if (0..n).all(|i| s[i] == t[i] || t[i] == '*') {
        println!("Yes");
    } else {
        println!("No");
    }
}
