fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    getline();
    let a = getline().trim().split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let k = getline().trim().parse::<i32>().unwrap();
    println!("{}", a.iter().filter(|&&x| x >= k).count());
}
