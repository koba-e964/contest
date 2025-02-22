fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let mut ans = "".to_string();
    for c in getline().chars() {
        if c == '2' {
            ans.push('2');
        }
    }
    println!("{ans}");
}
