fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let mut ans = 0;
    for i in 1..13 {
        let s = getline().trim().to_string();
        if s.len() == i {
            ans += 1;
        }
    }
    println!("{ans}");
}
