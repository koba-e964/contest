fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let a: i32 = getline().trim().parse().unwrap();
    let b: i32 = getline().trim().parse().unwrap();
    println!("{}", a * b);
}
