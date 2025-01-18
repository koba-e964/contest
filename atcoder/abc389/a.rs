fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let s = getline().bytes().collect::<Vec<_>>();
    println!("{}", (s[0] - b'0') * (s[2] - b'0'));
}
