#include <iostream>
#include <iomanip>
#include <string>
#include <algorithm>
#include <utility>
#include <vector>
#include <sstream>

using namespace std;

#define REP(i, s, n) for (int i = (int)(s); i < (int)(n); ++i)

typedef long long ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
typedef pair<ll, ll> PL;
typedef pair<double, double> PD;

const int N = 100100;

ll x[N];
bool var[N];

VL g[N];

const ll inf = 1e16;

PD dfs(int v, int par, double tol) {
  double lo = -inf, hi = inf;
  if (not var[v]) {
    lo = hi = x[v];
  }
  for (int w: g[v]) {
    if (w == par) continue;
    PD sub = dfs(w, v, tol);
    if (sub.first > sub.second) {
      return PD(0, -1);
    }
    lo = max(lo, sub.first - tol);
    hi = min(hi, sub.second + tol);
  }
  return PD(lo, hi);
}

int main(void) {
  ios_base::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  REP(i, 0, n) {
    string s;
    cin >> s;
    if (s == "*") {
      var[i] = 1;
    } else {
      stringstream ss(s);
      ss >> x[i];
    }
  }
  REP(i, 0, n - 1) {
    int u, v;
    cin >> u >> v;
    u--, v--;
    g[u].push_back(v);
    g[v].push_back(u);
  }
  double pass = 2.5e6, fail = 0;
  REP(_, 0, 100) {
    double mid = (pass + fail) / 2;
    // Is mid possible?
    PD res = dfs(0, -1, mid);
    if (res.first <= res.second) {
      pass = mid;
    } else {
      fail = mid;
    }
  }
  cout << setprecision(15) << pass << endl;
}
