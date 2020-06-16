namespace Solution {
    open Microsoft.Quantum.Intrinsic;
    open Microsoft.Quantum.Arithmetic;
    open Microsoft.Quantum.Arrays;

    operation Solve (qs : Qubit[]) : Unit {
        mutable ans = One;
        using (anc = Qubit()) {
            repeat {
                Reset(anc);
                ResetAll(qs);
                let _ = ForEach(H, qs);
                Controlled X(qs, anc);
                set ans = M(anc);
            } until (ans == Zero)
            fixup {}
        }
        let _ = ForEach(X, qs);
    }
    operation Test(count: Int): Int[] {
        mutable arr = new Int[4];
        for (test in 1..count) {
            using (qs = Qubit[2]) {
                Solve(qs);
                let res = MeasureInteger(LittleEndian(qs));
                set arr = arr w/ res <- arr[res] + 1;
            }
        }
        return arr;
    }
}
