fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    getline();
    let mut ans = 0;
    let mut c = 0;
    let mut d = 0;
    for ch in getline().trim().chars() {
        if ch == 'o' {
            c += 1;
            ans = std::cmp::max(ans, c);
        } else {
            c = 0;
            d = 1;
        }
    }
    println!("{}", if ans == 0 || d == 0 { -1 } else { ans });
}
