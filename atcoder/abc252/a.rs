fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let n: u8 = getline().trim().parse().unwrap();
    println!("{}", n as char);
}
