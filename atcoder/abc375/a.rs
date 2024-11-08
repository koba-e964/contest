fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    getline();
    let s = getline().trim().bytes().collect::<Vec<_>>();
    let mut ans = 0;
    for i in 2..s.len() {
        if &s[i - 2..i + 1] == b"#.#" {
            ans += 1;
        }
    }
    println!("{ans}");
}
