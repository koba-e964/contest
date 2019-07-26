include!("pollard_rho.rs");

fn main() {
    use pollard_rho::*;
    assert_eq!(factorize(4681), vec![(31, 1), (151, 1)]);
}
