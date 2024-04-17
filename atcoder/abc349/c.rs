fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let mut s: Vec<_> = getline().trim().bytes().collect();
    let t: Vec<_> = getline().trim().bytes().collect();
    s.push(b'x');
    let mut i = 0;
    for c in s {
        if i >= 3 {
            break;
        }
        if t[i] == c + b'A' - b'a' {
            i += 1;
        }
    }
    println!("{}", if i >= 3 { "Yes" } else { "No" });
}
