fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let y = getline().trim().parse::<i64>().unwrap();
    println!("{}", 365 + ((y % 400 == 0) ^ (y % 100 == 0) ^ (y % 4 == 0)) as i32);
}
