#include <algorithm>
#include <cmath>
#include <complex>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef pair<int, int> PI;
const double EPS=1e-9;
typedef complex<double> comp;



/*
 * Convex hull
 * Requirement, <vector>, <complex>, std, comp
 * Note: ps is modified
 * reference : arihon (ISBN-10: 4839931992)
 */
struct cmp_x {
  bool operator()(const comp &x, const comp &y) {
    return x.real() < y.real();
  }
};
vector<comp> convex_hull(vector<comp> &ps) {
  sort(ps.begin(), ps.end(), cmp_x());
  const int n = ps.size();
  int k = 0;
  vector<comp> qs(2 * n);
  // lower
  for (int i = 0; i < n; ++i) {
    while (k > 1 &&  (conj(qs[k - 1] - qs[k - 2]) * (ps[i] - qs[k - 1])).imag() <= 0) {
      k--;
    }
    qs[k++] = ps[i];
  }
  // upper
  for (int i = n - 2, t = k; i >= 0; --i) {
    while (k > t && (conj(qs[k - 1] - qs[k - 2]) * (ps[i] - qs[k - 1])).imag() <= 0) {
      k--;
    }
    qs[k++] = ps[i];
  }
  qs.resize(k - 1);
  return qs;
}


int main(void){
  double ax = 0, ay = 0;
  vector<comp> ps;
  REP(i, 0, 5) {
    double p, q;
    cin >> p >> q;
    ps.push_back(comp(p, q));
  }
  cout << (convex_hull(ps).size() == 5 ? "YES" : "NO") << endl;
}
