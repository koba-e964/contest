/*
 * Sparse Table.
 * biop should be the type of a binary operator which is
 * associative, commutative and idempotent.
 * (For example, min and gcd satisfy them.)
 * Verified by: Codeforces #361 (Div. 2) D (TLE, http://codeforces.com/contest/689/submission/22087204)
 */
final class SparseTable<T> {
    private java.util.function.BinaryOperator<T> biop;
    private Object[][] st;
    private void create_sparse_table(int n, T[] lcp) {
	int h = 1;
	while ((1 << h) < n) {
	    ++h;
	}
	st = new Object[h + 1][n];

	for (int i = 0; i < n; ++i) {
	    st[0][i] = lcp[i];
	}
	for (int j = 1; j <= h; ++j) {
	    for (int i = 0; i <= n - (1 << j); ++i) {
		st[j][i] = biop.apply((T)st[j - 1][i], (T)st[j - 1][i + (1 << (j-1))]);
	    }
	}
    }
    static final int top_bit(int t) {
	return 31 - Integer.numberOfLeadingZeros(t);
    }
    /*
     * Initializes this sparse table. O(n log n) where n = ary.size().
     */
    public SparseTable(java.util.function.BinaryOperator<T> biop, T[] ary) {
	this.biop = biop;
	create_sparse_table(ary.length, ary);
    }
    /*
     * Computes biop(ary[f], ary[f+1], ..., ary[s]). O(1).
     * Note: the interval is inclusive.
     */
    public final T query(int f, int s) {
	assert (f <= s);
	int diff = top_bit(s + 1 - f);
	return biop.apply((T) st[diff][f], (T) st[diff][s + 1 - (1 << diff)]);
    }
}
/*
 * Sparse Table for long[].
 * BiOp should be the type of a binary operator which is
 * associative, commutative and idempotent.
 * (For example, min and gcd satisfy them.)
 * Verified by: Codeforces #361 (Div. 2) D (http://codeforces.com/contest/689/submission/22087253)
 */
final class LongSparseTable {
    private java.util.function.LongBinaryOperator biop;
    private long[][] st;
    private void create_sparse_table(int n, long[] lcp) {
	int h = 1;
	while ((1 << h) < n) {
	    ++h;
	}
	st = new long[h + 1][n];

	for (int i = 0; i < n; ++i) {
	    st[0][i] = lcp[i];
	}
	for (int j = 1; j <= h; ++j) {
	    for (int i = 0; i <= n - (1 << j); ++i) {
		st[j][i] = biop.applyAsLong((long)st[j - 1][i], (long)st[j - 1][i + (1 << (j-1))]);
	    }
	}
    }
    static final int top_bit(int t) {
	return 31 - Integer.numberOfLeadingZeros(t);
    }
    /*
     * Initializes this sparse table. O(n log n) where n = ary.size().
     */
    public LongSparseTable(java.util.function.LongBinaryOperator biop, long[] ary) {
	this.biop = biop;
	create_sparse_table(ary.length, ary);
    }
    /*
     * Computes biop(ary[f], ary[f+1], ..., ary[s]). O(1).
     * Note: the interval is inclusive.
     */
    public final long query(int f, int s) {
	assert (f <= s);
	int diff = top_bit(s + 1 - f);
	return biop.applyAsLong((long) st[diff][f], (long) st[diff][s + 1 - (1 << diff)]);
    }
}
