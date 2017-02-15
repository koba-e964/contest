/*
 * Find an assignment (result) s.t. xor_i a[i] * result[i] = b (in GF(2))
 * Returns true if such an assignment was found.
 * Verified by: yukicoder No.460 (http://yukicoder.me/submissions/150485)
 */
bool gauss_elim_gf2_i64(vector<ll> a, ll b, vector<bool> &result) {
  int n = a.size();
  int c = 0;
  vector<int> revmap;
  REP(r, 0, 64) {
    if (c >= n) {
      break;
    }
    int c2 = -1;
    REP(i, c, n) {
      if (a[i] & (1LL << r)) {
	c2 = i;
	break;
      }
    }
    if (c2 < 0) {
      revmap.push_back(-1);
      continue;
    }
    if (c != c2) {
      swap(a[c], a[c2]);
    }
    ll rm = a[c] & -(1LL << r) << 1;
    a[c] ^= rm;
    REP(k, c + 1, n) {
      if (a[k] & (1LL << r)) {
	a[k] ^= rm;
      }
    }
    if (b & (1LL << r)) {
      b ^= rm;
    }
    revmap.push_back(c);
    c++;
  }
  // recover
  int rank = revmap.size();
  result.assign(n, false);
  for (int i = rank - 1; i >= 0; --i) {
    if (b & 1LL << i) {
      int c = revmap[i];
      if (c < 0) {
	return false;
      }
      b ^= a[c];
      result[c] = true;
    }
  }
  return b == 0;
}
