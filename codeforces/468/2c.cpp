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

void output(int cnt, const VL &y) {
  cout << cnt << "\n";
  REP(i, 0, y.size()) {
    cout << y[i] << (i == (int) y.size() - 1 ? "\n" : " ");
  }
  exit(0);
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VL x(n);
  ll ma = -1e8, mi = 1e8;
  REP(i, 0, n) {
    cin >> x[i];
    ma = max(ma, x[i]);
    mi = min(mi, x[i]);
  }
  if (ma - mi <= 1) {
    output(n, x);
  }
  VL cnt(3);
  ll dif = 0;
  REP(i, 0, n) {
    cnt[x[i] - mi] += 1;
    dif += x[i] - mi;
  }
  pair<ll, PI> rmi(1e15, PI(-1, -1));
  REP(z, 0, n + 1) {
    if (dif - 2 * z < 0 || dif - 2 * z > n - z) {
      continue;
    }
    ll q = dif - 2 * z;
    ll p = n - q - z;
    if (p < 0 || q < 0) continue;
    ll cost = min(cnt[0], p) + min(cnt[1], q) + min(cnt[2], (ll) z);
    rmi = min(rmi, make_pair(cost, PI(p, q)));
  }
  VL y(n);
  REP(i, 0, n) {
    if (i < rmi.second.first) {
      y[i] = mi;
    } else if (i < rmi.second.first + rmi.second.second) {
      y[i] = mi + 1;
    } else {
      y[i] = mi + 2;
    }
  }
  output(rmi.first, y);
}
