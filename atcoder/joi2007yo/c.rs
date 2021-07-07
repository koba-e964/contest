fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let s: Vec<_> = getline().trim().chars().collect();
    for c in s {
        let idx = c as u8 - b'A';
        let to = (idx + 23) % 26;
        print!("{}", (b'A' + to) as char);
    }
    println!();
}
