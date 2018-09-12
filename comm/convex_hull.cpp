/*
 * Convex hull.
 * Verified by https://beta.atcoder.jp/contests/agc021/submissions/3179522
 * Dependencies: algorithm, complex, vector
 */
typedef ll coord; // the type of coordinates
typedef complex<coord> P;
namespace std {
  bool operator<(const P& a, const P& b) {
    return real(a) != real(b) ? real(a) < real(b) : imag(a) < imag(b);
  }
}

coord det(P a, P b) {
  return imag(conj(a) * b);
}

vector<P> convex_hull(vector<P> ps) {
  int n = ps.size(), k = 0;
  sort(ps.begin(), ps.end());
  vector<P> ch(2 * n);
  // lower
  for (int i = 0; i < n; k++, i++) {
    while (k >= 2 && det(ps[i] - ch[k - 2], ch[k - 1] - ch[k - 2]) >= 0) --k;
    ch[k] = ps[i];
  }
  // upper
  for (int i = n-2, t = k+1; i >= 0; k++, i--) {
    while (k >= t && det(ps[i] - ch[k - 2], ch[k - 1] - ch[k - 2]) >= 0) --k;
    ch[k] = ps[i];
  }
  ch.resize(k-1);
  return ch;
}
