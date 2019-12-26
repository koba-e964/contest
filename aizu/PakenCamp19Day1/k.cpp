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

int n, q;
const int N = 100000;
ll t[N];
double a[N], acc[N];
ll l[N], r[N];

double f(ll x) {
  int idx = lower_bound(t, t + n, x) - t;
  return acc[idx];
}

double perf(ll l, ll r) {
  double x = f(r) - f(l);
  return 1e9 * exp(x);
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  cin >> n;
  REP(i, 0, n) {
    cin >> t[i] >> a[i];
    a[i] = log(1 - a[i] / 10);
    acc[i + 1] = acc[i] + a[i];
  }
  cin >> q;
  REP(i, 0, q) cin >> l[i] >> r[i];
  REP(i, 0, q) {
    cout << fixed << setprecision(15) << perf(l[i], r[i]) << "\n";
  }
}
