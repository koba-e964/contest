#include <algorithm>
#include <complex>
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

#define rep(i, n) for (int i = 0, N = n; i < N; i++)

using ld = long double;
using P = complex<ld>;
using VP = vector<P>;
const ld eps = 1e-8, pi = acos(-1.0);

#define EQ(a, b) (abs((a) - (b)) < eps)

ld dot(P a, P b) { return real(conj(a) * b); }
ld cross(P a, P b) { return imag(conj(a) * b); }

namespace std {
bool operator<(const P &lhs, const P &rhs) {
  return lhs.real() == rhs.real() ? lhs.imag() < rhs.imag()
                                  : lhs.real() < rhs.real();
}
}  // namespace std

struct L {
  P a, b;
};
struct C {
  P p;
  ld r;
};

// counter clockwise
int ccw(P a, P b, P c) {
  b -= a;
  c -= a;
  if (cross(b, c) > eps) return 1;    // counter clockwise
  if (cross(b, c) < -eps) return -1;  // clockwise
  if (dot(b, c) < 0) return 2;        // c--a--b on line
  if (norm(b) < norm(c)) return -2;   // a--b--c on line
  return 0;                           // a--c--b on line
}

VP is_cc(C c1, C c2){
  VP res;
  ld d = abs(c1.p - c2.p);
  if (d == 0) {
    return vector<P>();
  }
  ld rc = (d * d + c1.r * c1.r - c2.r * c2.r) / (2 * d);
  ld dfr = c1.r * c1.r - rc * rc;
  if (abs(dfr) < eps) dfr = 0.0;
  else if (dfr < 0.0) return res; // no intersection
  ld rs = sqrt(dfr);
  P diff = (c2.p-c1.p)/d;
  res.push_back(c1.p + diff * P(rc, rs));
  if (dfr != 0.0) res.push_back(c1.p + diff * P(rc, -rs));
  return res;
}

int n;
ld r;
const int N = 251;
ld x[N], y[N], h[N];

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  cin >> n >> r;
  vector<P> cand;
  REP(i, 0, n) cin >> x[i] >> y[i] >> h[i];
  REP(i, 0, n) {
    REP(j, 0, i) {
      REP(sgn1, 0, 2) {
        REP(sgn2, 0, 2) {
          C c1, c2;
          c1.p = P(x[i], y[i]);
          c1.r = max(ld(0.0), h[i] + (sgn1 ? r : -r));
          c2.p = P(x[j], y[j]);
          c2.r = max(ld(0.0), h[j] + (sgn2 ? r : -r));
          vector<P> ints = is_cc(c1, c2);
          REP(l, 0, ints.size()) {
            assert(!isnan(ints[l].real()));
            assert(!isnan(ints[l].imag()));
            cand.push_back(ints[l]);
          }
        }
      }
    }
    REP(sgn1, 0, 2) {
      ld tt = max(h[i] + (sgn1 ? r : -r), ld(0.0));
      cand.push_back(P(x[i] + tt, y[i]));
    }
  }
  pair<int, P> ma(-1, P(0, 0));
  for (P c: cand) {
    int tmp = 0;
    REP(i, 0, n) {
      ld dx = c.real() - x[i];
      ld dy = c.imag() - y[i];
      ld tt = r + h[i] + eps;
      ld tp = max(ld(0.0), h[i] - r - eps);
      ld sq = dx * dx + dy * dy;
      if (sq <= tt * tt && tp * tp <= sq) {
        tmp++;
      }
    }
    ma = max(ma, make_pair(tmp, c));
  }
  cout << ma.first << endl;
  cout << fixed << setprecision(15)
       << ma.second.real() << " " << ma.second.imag() << endl;
  vector<int> pts;
  vector<ld> crs;
  ld pi = acos(-1);
  REP(i, 0, n) {
    ld dx = ma.second.real() - x[i];
    ld dy = ma.second.imag() - y[i];
    ld tt = r + h[i] + eps;
    ld tp = max(ld(0.0), h[i] - r - eps);
    ld sq = dx * dx + dy * dy;
    if (sq <= tt * tt && tp * tp <= sq) {
      P dif = ma.second - P(x[i], y[i]);
      ld a = dif == P(0, 0) ? 0 : arg(dif);
      if (a < 0) {
        a += 2 * pi;
      }
      pts.push_back(i);
      crs.push_back(a);
    }
  }
  assert((int) pts.size() == ma.first);
  REP(i, 0, ma.first) {
    cout << pts[i] + 1 << " " << min(ld(360.0), crs[i] * 180 / pi) << endl;
  }
}
