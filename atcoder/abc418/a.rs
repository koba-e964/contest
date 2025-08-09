fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    getline();
    let s = getline().trim().to_string();
    println!("{}", if s.ends_with("tea") {
        "Yes"
    } else {
        "No"
    });
}
