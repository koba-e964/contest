fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn freq(mut s: i32) -> [i32; 10] {
    let mut f = [0; 10];
    while s > 0 {
        f[(s % 10) as usize] += 1;
        s /= 10;
    }
    f
}

fn main() {
    let s = getline().trim().parse::<i32>().unwrap();
    let f = freq(s);
    let mut mi = 100000;
    for i in 1..100000 {
        if f == freq(i) {
            mi = mi.min(i);
        }
    }
    println!("{mi}");
}
