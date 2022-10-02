fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let a: usize = getline().trim().parse().unwrap();
    let b: Vec<_> = "0123456789ABCDEF".chars().collect();
    println!("{}{}", b[a / 16], b[a % 16]);
}
