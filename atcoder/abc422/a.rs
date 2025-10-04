fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let ints = getline().trim().split("-")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    if ints[1] == 8 {
        println!("{}-1", ints[0] + 1);
    } else {
        println!("{}-{}", ints[0], ints[1] + 1);
    }
}
