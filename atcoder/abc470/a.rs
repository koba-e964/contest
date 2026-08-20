fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let n = getline().trim().parse::<i32>().unwrap();
    for i in 1..=n {
        if i % 3 == 0 {
            println!("Fizz");
        } else {
            println!("{i}");
        }
    }
}
