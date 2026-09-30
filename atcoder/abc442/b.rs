fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn yn(a: bool) -> &'static str {
    if a {
        "Yes"
    } else {
        "No"
    }
}

fn main() {
    let q = getline().trim().parse::<i32>().unwrap();
    let mut play = false;
    let mut vol = 0;
    for _ in 0..q {
        let a = getline().trim().parse::<i32>().unwrap();
        match a {
            1 => vol += 1,
            2 => vol = 0.max(vol - 1),
            3 => play ^= true,
            _ => todo!(),
        }
        println!("{}", yn(play && vol >= 3));
    }
}
