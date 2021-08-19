// (basis, x) where basis = x a
fn find_basis_gf2(a: &[Vec<i32>]) -> Vec<(Vec<i32>, Vec<i32>)> {
    let n = a.len();
    let m = a[0].len();
    let mut basis: Vec<(Vec<_>, Vec<_>)> = vec![];
    for i in 0..n {
        let mut cur = a[i].to_vec();
        let mut real = vec![0; n];
        real[i] = 1;
        for &(ref b, ref br) in &basis {
            let mut xor = cur.clone();
            for j in 0..m {
                xor[j] ^= b[j];
            }
            if xor < cur {
                cur = xor;
                for j in 0..n {
                    real[j] ^= br[j];
                }
            }
        }
        if cur.iter().any(|&x| x != 0) {
            basis.push((cur, real));
        }
    }
    basis
}
