fn main() {
    let x = 597965565i64;
    const MOD: i64 = 998_244_353;
    for i in 0..100000 {
        let v = x + MOD * i;
        let mut c = 0;
        {
            let mut v = v;
            while v % 3 == 0 {
                v /= 3;
                c += 1;
            }
        }
        if c >= 7 {
            eprintln!("i = {i}, v = {v}, c = {c}");
        }

    }
}
