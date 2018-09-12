#include <algorithm>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <complex>
#include <list>
#include <map>
#include <queue>
#include <random>
#include <set>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

typedef ll coord;
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

int main(void) {
  double pi = acos(-1.0);
  int n;
  scanf("%d", &n);
  vector<ll> x(n), y(n);
  vector<P> pt(n);
  map<P, int> tbl;
  REP(i, 0, n) {
    scanf("%lld%lld", &x[i], &y[i]);
    pt[i] = P(x[i], y[i]);
    tbl[pt[i]] = i;
  }
  if (n == 2) {
    printf("0.5\n0.5\n");
    return 0;
  }
  vector<P> ch = convex_hull(pt);
  int m = ch.size();
  vector<double> div(m);
  REP(i, 0, m) {
    P d = (ch[(i + 1) % m] - ch[i]);
    double del = arg(complex<double>(d.real(), d.imag()));
    div[i] = del;
  }
  vector<double> ans(n);
  REP(i, 0, m) {
    int idx = tbl[ch[i]];
    double diff = div[i] - div[(i + m - 1) % m];
    if (diff < 0) {
      diff += 2 * pi;
    }
    ans[idx] = diff / (2 * pi);
  }
  REP(i, 0, n) {
    printf("%.15f\n", ans[i]);
  }
}
