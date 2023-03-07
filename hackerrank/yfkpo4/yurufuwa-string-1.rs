fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let mut s = getline().into_bytes();
    for i in 0..4 {
        s[2 * i] = b'?';
    }
    println!("{}", if &s[..8] == b"?u?u?u?a" { "Yes" } else { "No" });
}
