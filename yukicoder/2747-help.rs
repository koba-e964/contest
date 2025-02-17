const MOD: i64 = 998_244_353;
fn main() {
    let mut fac = 1;
    let len = 100;
    let step = 10_000_000;
    println!("const STEP: usize = {step};");
    println!("const LEN: usize = {len};");
    println!("const FACT_TABLE: [i64; {len}] = [");
    let mut x = 0;
    for _ in 0..len {
        println!("    {},", fac);
        for _ in 0..step {
            x += 1;
            fac = fac * x as i64 % MOD;
        }
    }
    println!("];");
}
