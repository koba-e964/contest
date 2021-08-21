fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let s = getline().trim().to_owned();
    println!("{}", if s == "Hello,World!" { "AC" } else { "WA" });
}
