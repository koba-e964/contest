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

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

const ll inf = 1e16;

const int DEBUG = 0;

// !
void output(const VI &ans) {
  REP(i, 0, ans.size()) {
    cout << ans[i] << (i == (int) ans.size() - 1 ? "\n" : " ");
  }
  exit(0);
}

const int N = 315000;
ll x_tbl[N];

void init_x(int k) {
  ll prod = 1;
  ll lim = inf / k;
  REP(i, 0, N) {
    x_tbl[i] = prod;
    if (prod >= lim) {
      prod = inf;
    } else {
      prod = k * prod + 1;
    }
  }
}

void rec2(int k, int n, VI &ans, ll rank) {
  if (n <= 0) {
    return;
  }
  if (rank == 1) {
    return; // empty list
  }
  ll y = x_tbl[n - 1];
  int tip = (rank - 2) / y;
  ans.push_back(tip + 1);
  rec2(k, n - 1, ans, rank - 1 - y * tip);
}


// 0 <= length <= n, (x + delta) / 2 - th (exact division)
// x = 1 + k + ... + k^n
// (x + delta) / 2 is in [1, x]
void rec(int k, int n, VI &ans, int delta) {
  ll x = x_tbl[n];
  if (DEBUG) {
    cerr << "rec " << k << " " << n << " " << delta << endl;
    cerr << "x = " << x << endl;
  }
  if (n <= 0) {
    return;
  }
  assert (x + delta >= 2);
  assert (x + delta <= 2 * x);
  ll y = x_tbl[n - 1];
  int tip = k / 2;
  if (y <= 2 * abs(delta - 1)) {
    ll rank = (x + delta) / 2;
    assert ((x + delta) % 2 == 0);
    rec2(k, n, ans, rank);
    return;
  }
  ans.push_back(tip + 1);
  if (DEBUG) {
    cerr << "tip[" << n << "]=" << tip << endl;
  }
  rec(k, n - 1, ans, delta - 1);
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int k, n;
  cin >> k >> n;
  init_x(k);
  VI ans;
  if (k % 2 == 0) {
    ans = VI(n, k);
    ans[0] = k / 2;
    output(ans);
  }
  rec(k, n, ans, n % 2 == 0 ? 1 : 2);
  output(ans);
}
