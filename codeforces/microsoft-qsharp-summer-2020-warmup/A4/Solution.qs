namespace Solution {
    open Microsoft.Quantum.Intrinsic;

    operation Solve (unitary : (Qubit[] => Unit is Adj+Ctl)) : Int {
        mutable ans = -1;
        using (q = Qubit[2]) {
            unitary(q);
            if (M(q[1]) == One) {
                set ans = 0;
                X(q[1]);
            } else {
                set ans = 1;
            }
        }
        return ans;
    }
}
