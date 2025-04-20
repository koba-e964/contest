fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let mut t = "".to_string();
    for c in getline().trim().chars() {
        if c.is_ascii_uppercase() {
            t.push(c);
        }
    }
    println!("{t}");
}
