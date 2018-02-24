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
const ll mod = 1e9 + 7;

const double EPS = 1e-8;
const double INF = 1e12;
// http://www.prefield.com/algorithm/geometry/geometry.html
typedef complex<ll> P;
namespace std {
  bool operator < (const P& a, const P& b) {
    return real(a) != real(b) ? real(a) < real(b) : imag(a) < imag(b);
  }
}
double cross(const P& a, const P& b) {
  return imag(conj(a)*b);
}
double dot(const P& a, const P& b) {
  return real(conj(a)*b);
}

struct L : public vector<P> {
  L(const P &a, const P &b) {
    push_back(a); push_back(b);
  }
};

typedef vector<P> G;

struct C {
  P p; double r;
  C(const P &p, double r) : p(p), r(r) { }
};

int ccw(P a, P b, P c) {
  b -= a; c -= a;
  if (cross(b, c) > 0)   return +1;       // counter clockwise
  if (cross(b, c) < 0)   return -1;       // clockwise
  if (dot(b, c) < 0)     return +2;       // c--a--b on line
  if (norm(b) < norm(c)) return -2;       // a--b--c on line
  return 0;
}

typedef P point;



vector<point> convex_hull(vector<point> ps) {
  int n = ps.size(), k = 0;
  sort(ps.begin(), ps.end());
  vector<point> ch(2*n);
  for (int i = 0; i < n; ch[k++] = ps[i++]) // lower-hull
    while (k >= 2 && ccw(ch[k-2], ch[k-1], ps[i]) <= 0) --k;
  for (int i = n-2, t = k+1; i >= 0; ch[k++] = ps[i--]) // upper-hull
    while (k >= t && ccw(ch[k-2], ch[k-1], ps[i]) <= 0) --k;
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
