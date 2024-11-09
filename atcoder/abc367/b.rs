fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let mut s = getline().trim().to_string();
    while let Some(c) = s.pop() {
        if c == '0' {
            continue;
        }
        s.push(c);
        break;
    }
    if let Some(c) = s.pop() {
        if c != '.' { s.push(c); }
    }
    println!("{s}");
}
