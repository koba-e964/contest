fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let mut x: i64 = getline().trim().parse().ok().unwrap();
    for _ in 0..2 {
        x *= 10;
        x += x / 1000;
        x %= 1000;
        println!("{x}");
    }
}
