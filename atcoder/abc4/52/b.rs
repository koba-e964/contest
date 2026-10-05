fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let ints = getline().trim().split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let [h, w] = ints[..] else { panic!() };
    for i in 0..h {
        for j in 0..w {
            print!("{}", if i == 0 || i == h - 1 || j == 0 || j == w - 1 {
                "#"
            } else {
                "."
            });
        }
        println!();
    }
}
