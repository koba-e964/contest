fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let ints = getline().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
    let mut x = ints[0] + ints[1];
    if x > 12 {
        x -= 12;
    }
    println!("{x}");
}
