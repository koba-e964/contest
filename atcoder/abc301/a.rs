fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    getline();
    let s = getline().trim().chars().collect::<Vec<_>>();
    let n = s.len();
    let a = s.iter().filter(|&&c| c == 'A').count();
    println!("{}", if 2 * a > n || (2 * a == n && s[n - 1] == 'T') { "A" } else { "T" });
}
