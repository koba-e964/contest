fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let r = getline().trim().parse::<i64>().unwrap();
    println!("{}", (399 - r) % 100 + 1);
}
