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

void vec_debug(const VI &z) {
  REP(i, 0, z.size()) {
    cerr << " " << z[i];
  }
  cerr << endl;
}

template<class T>
void chmax(T &x, T y) {
  x = max(x, y);
}
template<class T>
void chmin(T &x, T y) {
  x = min(x, y);
}

const int inf = 1e8;

ll calc(const VI &z, const VI &w) {
  int n = z.size();
  // cerr << "z:"; vec_debug(z);
  // cerr << "w:"; vec_debug(w);
  vector<PI> pts;
  REP(i, 0, n) pts.push_back(PI(z[i], w[i]));
  sort(pts.begin(), pts.end());
  ll tot = 0;
  VL lma(n + 1, -inf), lmi(n + 1, inf), rma(n + 1, -inf), rmi(n + 1, inf);
  REP(i, 0, n) {
    ll y = pts[i].second;
    lma[i + 1] = max(lma[i], y);
    lmi[i + 1] = min(lmi[i], y);
  }
  for (int i = n - 1; i >= 0; --i) {
    ll y = pts[i].second;
    rma[i] = max(rma[i + 1], y);
    rmi[i] = min(rmi[i + 1], y);
  }
  REP(i, 1, n) {
    ll xdif = pts[i].first - pts[i - 1].first;
    ll ydif = min(rma[i], lma[i]) - max(rmi[i], lmi[i]);
    if (ydif > 0) {
      tot += xdif * ydif;
    }
  }
  return tot;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VI x(n), y(n);
  REP(i, 0, n) {
    cin >> x[i] >> y[i];
  }
  ll tot = 0;
  REP(r, 0, 2) {
    VI z, w;
    REP(i, 0, n) {
      if (abs(x[i] + y[i]) % 2 == r) {
	z.push_back((x[i] + y[i] + r) / 2);
	w.push_back((x[i] - y[i] + r) / 2);
      }
    }
    tot += calc(z, w);
  }
  cout << tot << "\n";
}
