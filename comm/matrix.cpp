/*
 * Requirement: VL
 * Header Requirement: vector
 */

vector<VL> mat_mul(const vector<VL> &A, const vector<VL> &B) {
  int n = A.size();
  int m = B.size();
  int l = B[0].size();
  vector<VL> result(n, VL(m));
  REP(i, 0, n) {
    REP(j, 0, m) {
      REP(k, 0, l) {
	result[i][k] += A[i][j] * B[j][k];
	result[i][k] %= mod;
      }
    }
  }
  return result;
}

vector<VL> mat_pow(const vector<VL> &mat, ll e) {
  int n = mat.size();
  vector<VL> sum(n, VL(n));
  REP(i, 0, n) { sum[i][i] = 1; }
  vector<VL> cur = mat;
  while (e > 0) {
    if (e % 2 == 1) {
      sum = mat_mul(sum, cur);
    }
    cur = mat_mul(cur, cur);
    e /= 2;
  }
  return sum;
}
