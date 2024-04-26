fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    getline();
    let mut good = 0;
    let mut bad = 0;
    for c in getline().trim().chars() {
        if c == 'o' {
            good += 1;
        }
        if c == 'x' {
            bad += 1;
        }
    }
    println!("{}", if good > 0 && bad == 0 { "Yes" } else { "No" });
}
