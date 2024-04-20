fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let s = getline().trim().to_string();
    for i in 1..350 {
        if i == 316 { continue; }
        let t = format!("ABC{:03}", i);
        if s == t {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
