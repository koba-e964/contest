fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let mut s = getline().trim().to_string();
    if s.ends_with("e") {
        s += "r"
    } else {
        s += "er"
    }
    println!("{s}")
}
