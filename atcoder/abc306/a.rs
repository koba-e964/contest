fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    getline();
    for c in getline().trim().chars() {
        print!("{}{}", c, c);
    }
    println!();
}
