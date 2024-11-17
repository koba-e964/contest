fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let mut s = getline().trim().bytes().collect::<Vec<_>>();
    s.sort();
    println!("{}", if s == b"122333" { "Yes" } else { "No" });
}
