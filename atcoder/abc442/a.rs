fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let mut ans = 0;
    for c in getline().chars() {
        if c == 'i' || c == 'j' {
            ans += 1;
        }
    }
    println!("{ans}");
}
