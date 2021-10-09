/// Depends on ModInt.rs
/// Finds x modulo M1*M2 s.t. x = a (mod M1), x = b (mod M2).
/// Verified by https://yukicoder.me/submissions/303386.
fn garner2<M1: mod_int::Mod, M2: mod_int::Mod>(a: mod_int::ModInt<M1>,
                                               b: mod_int::ModInt<M2>)
                                               -> i64 {
    let factor2 = mod_int::ModInt::new(M1::m()).inv();
    let factor1 = mod_int::ModInt::new(M2::m()).inv();
    ((b * factor2).x * M1::m() + (a * factor1).x * M2::m()) % (M1::m() * M2::m())
}

