fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let ints = getline().trim().split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    for i in 1..=ints[0] {
        println!("{}", if i <= ints[1] {
            "OK"
        } else {
            "Too Many Requests"
        });
    }
}
