fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let s = getline();
    for c in 'a'..='z' {
        if s.chars().all(|x| x != c) {
            println!("{c}");
            return;
        }
    }
}
