fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let n: i64 = getline().trim().parse().unwrap();
    for i in (0..n + 1).rev() {
        println!("{}", i);
    }
}
