fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let mut s = getline().chars().collect::<Vec<char>>();
    s.pop();
    let t = getline();
    let n = s.len();
    let mut ok = true;
    for i in 0..n - 1 {
        if s[i + 1].is_ascii_uppercase() {
            if !t.chars().any(|c| c == s[i]) {
                ok = false;
                break;
            }
        }
    }
    println!("{}", if ok { "Yes" } else { "No" });
}
