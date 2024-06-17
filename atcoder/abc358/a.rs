fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let s = getline();
    println!("{}", if s == "AtCoder Land\n" { "Yes" } else { "No" });
}
