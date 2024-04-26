fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let s = getline().trim().to_string();
    let mut ans = 0;
    for (i, c) in s.bytes().enumerate() {
        if c == b'.' {
            ans = i;
        }
    }
    println!("{}", &s[ans + 1..]);
}
