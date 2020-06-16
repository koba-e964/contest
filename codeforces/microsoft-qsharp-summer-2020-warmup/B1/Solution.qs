namespace Solution {
    open Microsoft.Quantum.Arithmetic;
    open Microsoft.Quantum.Intrinsic;
    open Microsoft.Quantum.Arithmetic;

    operation Solve (register : LittleEndian) : Unit is Adj+Ctl {
        let qs = register!;
        for (i in 0..Length(qs) - 1) {
            let idx = Length(qs) - 1 - i;
            Controlled X(qs[...(idx-1)], qs[idx]);
        }
    }
    operation Test(): Int {
        mutable ans = -1;
        using (q = Qubit[2]) {
            X(q[0]);
            let reg = LittleEndian(q);
            Solve(reg);
            set ans = MeasureInteger(reg);
            ResetAll(q);
        }
        return ans;
    }
}
