fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let y = getline().ends_with("san\n");
    println!("{}", if y { "Yes" } else { "No" });
}
